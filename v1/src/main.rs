use bevy::app::{App, AppExit, Plugin, Startup, Update};
use bevy::input::Input;
use bevy::math::{vec2, vec3, Vec2, Vec3};
use bevy::prelude::*;
use bevy::render::render_resource::encase::private::Length;
use bevy::sprite::{Anchor, SpriteBundle};
use bevy::time::{Timer, TimerMode};
use bevy::ui::{PositionType, Style};
use bevy::utils::default;
use bevy::DefaultPlugins;

const MOVEMENT_SPEED: f32 = 500.0;
const BOUNDARY_BOX: Vec2 = Vec2::new(500.0, 500.0);
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

#[derive(Resource)]
struct PlayerMovTimer(Timer);

#[derive(Component)]
struct Item {
    number: i64,
}

#[derive(Component)]
struct Movement(f32, bool);

// Tags
#[derive(Component)]
struct TagCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct DevText {
    mov_num: f32,
}

// stats
#[allow(dead_code)]
#[derive(Component)]
struct VitalStats {
    health: i32,
    mana: i32,
    energy: i32,
}

#[allow(dead_code)]
#[derive(Component)]
struct CoreStats {
    strength: i32,
    intelligence: i32,
    agility: i32,
    constitution: i32,
    fortune: i32,
    wisdom: i32,
}

// Grid
#[derive(Component)]
struct ActiveGrid;

#[derive(Component)]
struct GridBox(Vec3);

#[derive(Component)]
struct Grid(String);

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
    }, ));

    let world_grid = create_grid(&mut commands, GRID_BOX, BOUNDARY_BOX, "WorldGrid".into());
    let mut active_grid = commands.spawn((ActiveGrid, SpatialBundle { ..default() }));
    active_grid.push_children(&[world_grid]);

    // player sprite
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.55),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(vec3(0.0, 0.0, 1.0)),
            ..default()
        },
        Player,
        Movement(0f32, false),
        GridPos(vec3(-1.0, -1.0, 1.0)),
    )).with_children(|parent| {
        parent.spawn(
            Text2dBundle {
                text: Text::from_section(
                    "@".to_string(),
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    },

                ).with_alignment(TextAlignment::Center),
                text_anchor: Anchor::Center,
                transform: Transform::from_translation(vec3(0.0, 0.0, 2.0)),
                ..default()
            },  
        );
    });

    commands.spawn((
        GridPos(vec3(1.0, 0.0, 1.0)),
        Text2dBundle {
            text: Text::from_section(
                "A".to_string(),
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
                
            ).with_alignment(TextAlignment::Center),
            text_anchor: Anchor::Center,
            transform: Transform::from_translation(vec3(20.0, 0.0, 2.0)),
            ..default()
        }
    ));
}

fn create_grid(
    commands: &mut Commands,
    grid_box: Vec2,
    boundary_box: Vec2,
    grid_name: String,
) -> Entity {
    let grid_x = grid_box.x as usize;
    let grid_y = grid_box.y as usize;

    let mut grid = commands.spawn((Grid(grid_name), SpatialBundle { ..default() }));

    grid.with_children(|parent| {
        for x in ((boundary_box.x - grid_box.x) as i32 / -2..=boundary_box.x as i32 / 2).step_by(grid_x) {
            for y in ((boundary_box.y - grid_box.y) as i32 / -2..=boundary_box.y as i32 / 2).step_by(grid_y) {
                spawn_box(parent, (x) as f32, (y) as f32);
            }
        }
    });

    grid.id()
}

fn spawn_box(commands: &mut ChildBuilder, x: f32, y: f32) {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(vec3(x, y, 1.0)),
                visibility: Visibility::Visible,
                ..default()
            },
            GridBox(vec3((x / GRID_BOX.x).round(), (y / GRID_BOX.y).round(), 1.0)),
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
    grid_parent_q: Query<(&Grid, &Children)>,
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
    let mov: f32 = player.1.0;
    let mut elapsed = timer.0.elapsed_secs() * 10.0;
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
    mut keys: ResMut<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform, &mut Movement, &Sprite, &mut GridPos)>,
    mut exit: EventWriter<AppExit>,
    time: Res<Time>,
    mut player_mov_timer: ResMut<PlayerMovTimer>,
    active_grid: Query<(&ActiveGrid, &Children)>,
    grid: Query<(&Grid, &Children)>,
    grid_boxes: Query<(&GridBox, &GlobalTransform), Without<Grid>>,
) {
    for (_player, mut transform, mut mov, sprite, mut gridpos) in query.iter_mut() {
        let mut grid_mov = Vec3::ZERO;
        let mut movable = [true, true];

        //TODO: refactor input into config which the user can change

        if keys.pressed(KeyCode::Escape) {
            exit.send(AppExit);
        }

        if keys.just_pressed(KeyCode::Right) && movable[0] {
            movable[0] = false;
            grid_mov.x = 1.0;
        }

        if keys.just_pressed(KeyCode::Left) && movable[0] {
            movable[0] = false;
            grid_mov.x = -1.0;
        }

        if keys.just_pressed(KeyCode::Up) && movable[1] {
            movable[1] = false;
            grid_mov.y = 1.0;
        }

        if keys.just_pressed(KeyCode::Down) && movable[1] {
            movable[1] = false;
            grid_mov.y = -1.0;
        }

        let mut elapsed = player_mov_timer.0.tick(time.delta()).elapsed_secs() * 10.0;
        elapsed = elapsed - elapsed.fract();


        if !mov.1 && grid_mov != Vec3::ZERO && movable.contains(&false) {
            mov.1 = !mov.1;
            let new_grid_pos = gridpos.0 + grid_mov;
            let world_mov = vec3((grid_mov.x * GRID_BOX.x), (grid_mov.y * GRID_BOX.y), 0.0)
                .clamp_length_max(GRID_BOX.length()) + transform.translation;

            let active_grid = active_grid.single().1.get(0).unwrap();
            let grid = grid.get(*active_grid).unwrap().1;
            for (&boxes) in grid.iter() {
                let grid_box = grid_boxes.get(boxes).unwrap();
                if grid_box.1.translation().x == world_mov.x && grid_box.1.translation().y == world_mov.y {
                    gridpos.0 = new_grid_pos;
                    transform.translation = world_mov;

                    let player_size = sprite.custom_size.unwrap() / 2.0;
                    transform.translation.x = transform.translation.x.clamp(
                        BOUNDARY_BOX.x / -2.0 + player_size.x,
                        BOUNDARY_BOX.x / 2.0 - player_size.x,
                    );
                    transform.translation.y = transform.translation.y.clamp(
                        BOUNDARY_BOX.y / -2.0 + player_size.y,
                        BOUNDARY_BOX.y / 2.0 - player_size.y,
                    );
                };
            }
            for element in movable.iter_mut() { *element = true }
            mov.1 = !mov.1;
        } else {
            mov.0 = 0f32;
        }
    }
}

struct NebulaVault;

impl Plugin for NebulaVault {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerMovTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
            .insert_resource(NebulaTime(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, handle_input)
            .add_systems(Update, (print_dev))
            .add_systems(Startup, setup);
    }
}
