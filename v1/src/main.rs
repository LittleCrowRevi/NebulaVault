mod entities;

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
use entities::races::*;
use entities::components::*;

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


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    Loading,
    Exploration
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

#[derive(Component)]
struct hud;

#[derive(Bundle)]
struct EntityBundle {
    core_stats: CoreStats,
    vital_stats: VitalStats
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

#[derive(Component)]
struct GridPositions(Vec<Vec3>);

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

    let world_grid = create_square_grid(&mut commands, GRID_BOX, BOUNDARY_BOX, "WorldGrid".into());
    let mut active_grid = commands.spawn((ActiveGrid, SpatialBundle { ..default() }));
    active_grid.push_children(&[world_grid]);

    // player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.55),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(vec3(0.0, 0.0, 3.0)),
            ..default()
        },
        Player,
        Movement(0f32, false),
        GridPositions(vec!(vec3(0.0, 0.0, 3.0))),
        HumanBundle::default(),
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
                transform: Transform::from_translation(vec3(0.0, 0.0, 3.0)),
                ..default()
            },
        );
    });

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 13.0,
                ..default()
            },
        ).with_text_alignment(TextAlignment::Left)
            .with_style(
                Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                }
            )
        , hud));

    commands.spawn((
        GridPositions(vec!(vec3(1.0, 0.0, 2.0))),
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

fn update_hud(query: Query<(&VitalStats, &RaceName), (With<Player>)>, mut hud_q: Query<(&mut Text), (With<hud>)>) {
    let mut hud_text = hud_q.single_mut();
    let (p_stats, p_race) = query.single();
    hud_text.sections[0].value = format!("Health: {}\nMana: {}\nEnergy: {}\n", p_stats.health, p_stats.mana, p_stats.energy);
    hud_text.sections[0].value += &*format!("Race: {}\n", p_race.0);
}

fn create_square_grid(
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
    player_query: Query<(&Player, &Movement, &Transform, &GridPositions)>,
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
    
    let gird_pos = player.3.0.last().unwrap();
    text.push_str(&*format!(
        "Player Grid Position: x {} | y {} \n",
        gird_pos.x, gird_pos.y
    ));

    for (grid, children) in grid_parent_q.iter() {
        let child_boxes = children.length();
        text.push_str(&*format!("{} Boxes: {}\n", grid.0, child_boxes));
    }
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform, &mut Movement, &Sprite, &mut GridPositions)>,
    mut exit: EventWriter<AppExit>,
    time: Res<Time>,
    mut player_mov_timer: ResMut<PlayerMovTimer>,
    active_grid: Query<(&ActiveGrid, &Children)>,
    grid: Query<(&Grid, &Children)>,
    grid_boxes: Query<(&GridBox, &GlobalTransform), Without<Grid>>,
) {
    for (_, mut transform, mut mov, sprite, mut grid_positions) in query.iter_mut() {
        let current_pos = grid_positions.0.last().expect("[input] GridPos missing.");
        let mut grid_mov = Vec3::ZERO;

        //TODO: refactor input into config which the user can change

        if keys.pressed(KeyCode::Escape) {
            exit.send(AppExit);
        }

        if keys.just_pressed(KeyCode::Right) {
            grid_mov.x = 1.0;
        }

        if keys.just_pressed(KeyCode::Left) {
            grid_mov.x = -1.0;
        }

        if keys.just_pressed(KeyCode::Up) {
            grid_mov.y = 1.0;
        }

        if keys.just_pressed(KeyCode::Down) {
            grid_mov.y = -1.0;
        }

        let mut elapsed = player_mov_timer.0.tick(time.delta()).elapsed_secs() * 10.0;
        elapsed = elapsed - elapsed.fract();

        if grid_mov != Vec3::ZERO {
            let new_grid_pos = *current_pos + grid_mov;
            let world_mov = vec3(grid_mov.x * GRID_BOX.x, grid_mov.y * GRID_BOX.y, 0.0)
                .clamp_length_max(GRID_BOX.length()) + transform.translation;

            
            
            // get active grid
            let active_grid = active_grid.single().1.get(0).unwrap();
            let grid = grid.get(*active_grid).unwrap().1;

            for &boxes in grid.iter() {
                let grid_box = grid_boxes.get(boxes).unwrap();
                if grid_box.1.translation().x == world_mov.x && grid_box.1.translation().y == world_mov.y {
                    grid_positions.0.push(new_grid_pos);
                    transform.translation = world_mov;

                    // clamp to boundary
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
        }
    }
}

fn check_position(point: Vec3, board: ) {
    
}

struct NebulaVault;

impl Plugin for NebulaVault {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerMovTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
            .add_state::<GameState>()
            .insert_resource(NebulaTime(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, (handle_input, update_hud, print_dev))
            .add_systems(Startup, setup);
    }
}
