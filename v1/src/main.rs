use bevy::app::{App, AppExit, Plugin, Startup, Update};
use bevy::input::Input;
use bevy::math::{vec2, vec3, Vec2, Vec3};
use bevy::prelude::*;
use bevy::render::render_resource::encase::private::Length;
use bevy::sprite::SpriteBundle;
use bevy::text::{Text, TextAlignment, TextStyle};
use bevy::time::{Timer, TimerMode};
use bevy::ui::{PositionType, Style};
use bevy::utils::default;
use bevy::DefaultPlugins;

const MOVEMENT_SPEED: f32 = 500.0;
const BOUNDARY_BOX: Vec2 = Vec2::new(600.0, 400.0);
const GRID_BOX: Vec2 = Vec2::new(20.0, 20.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Nebula Vault"),
                    resolution: (800.0, 600.0).into(),
                    ..default()
                }),
                ..default()
            }),
            NebulaVault,
        ))
        .run();
}

#[derive(Resource)]
struct NebulaTime(Timer);

#[derive(Component)]
struct Item {
    number: i64,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Movement(f32);

#[derive(Component)]
struct TagCamera;

#[derive(Component)]
struct DevText {
    mov_num: f32,
}

#[derive(Component)]
struct Grid(String);

#[derive(Component)]
struct GridBox;

#[derive(Component)]
struct GridPos(Vec3);

const SCORE_COLOR: Color = Color::rgb(1., 1., 1.);

fn setup(mut commands: Commands) {
    println!("Starting NebulaVault!");

    // main camera
    commands.spawn((
        Camera2dBundle {
            transform: Transform::default(),
            ..default()
        },
        TagCamera,
    ));

    // dev info text
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 15f32,
                color: SCORE_COLOR,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        DevText { mov_num: 0f32 },
    ));

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.15, 0.15, 0.15),
            custom_size: Some(Vec2::new(BOUNDARY_BOX.x, BOUNDARY_BOX.y)),
            ..default()
        },
        ..default()
    },));

    create_grid(&mut commands, GRID_BOX, BOUNDARY_BOX, "MainGrid".into());

    // player sprite
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.55),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 1.0)),
            ..default()
        },
        Player,
        Movement(0f32),
        GridPos(Vec3::ZERO),
    ));
}

fn create_grid(commands: &mut Commands, grid_box: Vec2, boundary_box: Vec2, grid_name: String) {
    let grid_x = grid_box.x as usize;
    let grid_y = grid_box.y as usize;

    let mut grid = commands.spawn((Grid(grid_name), SpatialBundle { ..default() }));

    grid.with_children(|parent| {
        for x in
            ((boundary_box.x - grid_box.x) as i32 / -2..boundary_box.x as i32 / 2).step_by(grid_x)
        {
            for y in ((boundary_box.y - grid_box.y) as i32 / -2..boundary_box.y as i32 / 2)
                .step_by(grid_y)
            {
                spawn_box(parent, (x) as f32, (y) as f32);
            }
        }
    });
}

fn spawn_box(commands: &mut ChildBuilder, x: f32, y: f32) {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(vec3(x, y, 1.0)),
                visibility: Visibility::Visible,
                ..default()
            },
            GridBox,
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.2, 0.2),
                    custom_size: Some(vec2(GRID_BOX.x, GRID_BOX.y)),
                    ..default()
                },
                ..default()
            });
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.15, 0.15, 0.15),
                    custom_size: Some(vec2(
                        GRID_BOX.x - GRID_BOX.x / GRID_BOX.y,
                        GRID_BOX.y - GRID_BOX.y / GRID_BOX.y,
                    )),
                    ..default()
                },
                ..default()
            });
        });
}

fn print_dev(
    time: Res<Time>,
    mut timer: ResMut<NebulaTime>,
    query: Query<&Item>,
    mut commands: Commands,
    mut count_text: Query<(&mut Text, &mut DevText)>,
    player_query: Query<(&Player, &Movement, &Transform)>,
    window_query: Query<&Window>,
    grid_parent_q: Query<(&Grid, &Children)>,
    grid_boxes_q: Query<(&GridBox, &Transform)>,
) {
    let mut t = count_text.single_mut();
    let text: &mut String = &mut t.0.sections[0].value;
    let dev_stats = &mut t.1;
    *text = "Dev Stats\n".to_string();

    // seconds counter
    let mut count: i64 = query.iter().count() as i64;
    if timer.0.tick(time.delta()).just_finished() {
        count += 1;
        commands.spawn(Item {
            number: (count + 1),
        });
    };
    text.push_str(&format!("Seconds: {count}\n"));

    // Player stats
    let player = player_query.single();

    // movement speed counter
    let mov: f32 = player.1 .0;
    let mut elapsed = (timer.0.elapsed_secs() * 10.0);
    elapsed = elapsed - elapsed.fract();
    if elapsed % 2.0 == 0.0 {
        dev_stats.mov_num = mov;
    }
    let mov_text = format!("Speed: {}\n", dev_stats.mov_num);
    text.push_str(&mov_text);

    // position and grid position
    let position = player.2.translation;
    text.push_str(&*format!(
        "Player Position: x {} | y {} \n",
        position.x, position.y
    ));
    let grid_position = vec2(
        (position.x / GRID_BOX.x).round(),
        (position.y / GRID_BOX.y).round(),
    );
    text.push_str(&*format!(
        "Player Grid Position: x {} | y {} \n",
        grid_position.x, grid_position.y
    ));

    for (grid, children) in grid_parent_q.iter() {
        let child_boxes = children.length();
        text.push_str(&*format!("{} Boxes: {}\n", grid.0, child_boxes));
    }
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Player,
        &mut Transform,
        &mut Movement,
        &Sprite,
        &mut GridPos,
    )>,
    mut exit: EventWriter<AppExit>,
    time: Res<Time>,
    timer: Res<NebulaTime>,
) {
    for (mut player, mut transform, mut mov, sprite, mut gridpos) in query.iter_mut() {
        let mut pan = Vec2::ZERO;
        let mut distance = Vec3::ZERO;
        let mut grid_mov = Vec3::ZERO;

        //TODO: refactor input into config which the user can change

        if keys.pressed(KeyCode::Escape) {
            exit.send(AppExit);
        }

        if keys.pressed(KeyCode::Right) {
            grid_mov.x += 1.0;
            distance.x += MOVEMENT_SPEED;
        }

        if keys.pressed(KeyCode::Left) {
            distance.x += -MOVEMENT_SPEED;
            grid_mov.x += -1.0;
        }

        if keys.pressed(KeyCode::Up) {
            distance.y += MOVEMENT_SPEED;
            grid_mov.y += 1.0;
        }

        if keys.pressed(KeyCode::Down) {
            grid_mov.y += -1.0;
            distance.y += -MOVEMENT_SPEED;
        }

        let mut elapsed = (timer.0.elapsed_secs() * 10.0);
        elapsed = elapsed - elapsed.fract();

        if !distance.eq(&Vec3::ZERO) && elapsed % 2.0 == 0.0 {
            let movement_clamped = distance.clamp_length_max(MOVEMENT_SPEED);
            let mov_delta = movement_clamped * time.delta_seconds();

            //transform.translation += mov_delta;
            let new_grid_pos = gridpos.0 + grid_mov;
            let mut world_mov = vec3((grid_mov.x * GRID_BOX.x), (grid_mov.y * GRID_BOX.y), 0.0)
                .clamp_length_max(GRID_BOX.length())
                + transform.translation;

            let player_size = sprite.custom_size.unwrap() / 2.0;
            world_mov.x = world_mov.x.clamp(
                BOUNDARY_BOX.x / -2.0 + player_size.x,
                BOUNDARY_BOX.x / 2.0 - player_size.x,
            );
            world_mov.y = world_mov.y.clamp(
                BOUNDARY_BOX.y / -2.0 + player_size.y,
                BOUNDARY_BOX.y / 2.0 - player_size.y,
            );
            transform.translation = world_mov;

            mov.0 = world_mov.length();
        } else {
            mov.0 = 0f32;
        }
    }
}

struct NebulaVault;

impl Plugin for NebulaVault {
    fn build(&self, app: &mut App) {
        app.insert_resource(NebulaTime(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, (print_dev, handle_input))
            .add_systems(Startup, setup);
    }
}
