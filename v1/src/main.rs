use bevy::app::{App, Plugin, Startup, Update};
use bevy::DefaultPlugins;
use bevy::input::Input;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Camera2dBundle, Color, Commands, Component, KeyCode, Mut, Projection, Query, Res, ResMut, Resource, TextBundle, Time, Transform};
use bevy::text::{Text, TextAlignment, TextSection, TextStyle};
use bevy::time::{Timer, TimerMode};
use bevy::ui::{PositionType, Style};
use bevy::utils::default;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NebulaVault))
        .run();
}

#[derive(Component)]
struct Item {
    number: i64,
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct NebulaTime(Timer);

#[derive(Component)]
struct TagCamera;

const SCORE_COLOR: Color = Color::rgb(0.6, 0.6, 0.8);

fn setup(mut commnads: Commands) {
    println!("Starting NebulaVault!");

    commnads.spawn((Camera2dBundle { transform: Transform::default(), ..default()}, TagCamera));

    commnads.spawn(
        TextBundle::from_section(
            "Count: 0",
            TextStyle { font_size: 30f32, color: SCORE_COLOR, ..default() },
        )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style { position_type: PositionType::Absolute, ..default() })
    );
}

fn print_timer(time: Res<Time>, mut timer: ResMut<NebulaTime>, query: Query<&Item>, mut commands: Commands, mut count_text: Query<&mut Text>) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut count = query.iter().count();
        count += 1;

        let mut text = count_text.single_mut();
        text.sections[0].value = format!("Count: {count}");
        println!("Tick! Items: {count}");

        commands.spawn(Item { number: (count + 1) as i64 });
    };
}

fn handle_input(keys: Res<Input<KeyCode>>, mut query: Query<(&mut TagCamera, &mut Transform)>) {
    for (mut camera, mut transform) in query.iter_mut() {
        let mut pan = Vec2::ZERO;
        let mut distance = Vec3::ZERO;
        
        // if let Projection::Perspective(projection) = projection {
        //     pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov);
        // }

        if keys.just_pressed(KeyCode::Left) {}

        if keys.pressed(KeyCode::Right) {
            distance = Vec3::X;
            println!("{:#?}", distance);
        }
        transform.translation = distance;
    }
}

struct NebulaVault;

impl Plugin for NebulaVault {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(NebulaTime(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, (print_timer, handle_input))
            .add_systems(Startup, setup);
    }
}