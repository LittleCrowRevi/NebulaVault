use bevy::app::{App, AppExit, Plugin, Startup, Update};
use bevy::input::Input;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use bevy::text::{Text, TextAlignment, TextStyle};
use bevy::time::{Timer, TimerMode};
use bevy::ui::{PositionType, Style};
use bevy::utils::default;
use bevy::DefaultPlugins;

const MOVEMENT_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NebulaVault))
        
        .run();
}

#[derive(Resource)]
struct NebulaTime(Timer);

#[derive(Component)]
struct Item { number: i64 }

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Movement(f32);

#[derive(Component)]
struct TagCamera;

#[derive(Component)]
struct DevText { mov_num: f32 }

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
            bottom: Val::Px(10.0),
            ..default()
        }),
        DevText { mov_num: 0f32 }
    ));

    // player sprite
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.55),
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 0.0)),
            ..default()
        },
        Player,
        Movement ( 0f32 )
    ));
}

fn print_dev(
    time: Res<Time>,
    mut timer: ResMut<NebulaTime>,
    query: Query<&Item>,
    mut commands: Commands,
    mut count_text: Query<(&mut Text, &mut DevText)>,
    player_query: Query<(&Player, &Movement)>
) {
    let player = player_query.single();
    let mut mov = 0f32;
    mov = player.1.0;
    
    let mut t = count_text.single_mut();
    let text = &mut t.0.sections[0].value;
    let dev_stats = &mut t.1;
    *text = "Dev Settings\n".to_string();
    
    let mut count = query.iter().count();
    if timer.0.tick(time.delta()).just_finished() {
        count += 1;
        commands.spawn(Item {
            number: (count + 1) as i64,
        });
        
    };
    text.push_str(&format!("Seconds: {count}\n"));
    
    let mut elapsed = (timer.0.elapsed_secs() * 10.0);
    elapsed = elapsed - elapsed.fract();
    
    if elapsed % 2.0 == 0.0 {
        dev_stats.mov_num = mov;
        
    } else if mov == 0.0 {
        dev_stats.mov_num = mov;
    }
    
    let mov_text = format!("Speed: {}\n", dev_stats.mov_num);
    text.push_str(&mov_text);
    
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform, &mut Movement)>,
    mut exit: EventWriter<AppExit>,
    time: Res<Time>,
) {
    for (mut player, mut transform, mut mov) in query.iter_mut() {
        let mut pan = Vec2::ZERO;
        let mut distance = Vec3::ZERO;

        // if let Projection::Perspective(projection) = projection {
        //     pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov);
        // }

        //TODO: refactor input into config which the user can change

        if keys.pressed(KeyCode::Escape) {
            exit.send(AppExit);
        }

        if keys.pressed(KeyCode::Left) {
            distance.x += -MOVEMENT_SPEED;
        }

        if keys.pressed(KeyCode::Up) {
            distance.y += MOVEMENT_SPEED;
        }

        if keys.pressed(KeyCode::Down) {
            distance.y += -MOVEMENT_SPEED;
        }

        if keys.pressed(KeyCode::Right) {
            distance.x += MOVEMENT_SPEED;
        }

        if !distance.eq(&Vec3::ZERO) {
            let movement_clamped = distance.clamp_length_max(MOVEMENT_SPEED);
            transform.translation += movement_clamped * time.delta_seconds();
            mov.0 = movement_clamped.length() * time.delta_seconds();
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
