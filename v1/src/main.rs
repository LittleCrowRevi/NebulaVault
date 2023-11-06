use bevy::app::{App, Plugin, Startup, Update};
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, Color, Commands, Component, Mut, Query, Res, ResMut, Resource, TextBundle, Time};
use bevy::text::{Text, TextSection, TextStyle};
use bevy::time::{Timer, TimerMode};
use bevy::utils::default;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NebulaVault))
        .run();
}

#[derive(Component)]
struct Item { number: i64 }

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct NebulaTime(Timer);

const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn setup(mut commnads: Commands) {
    println!("Starting NebulaVault!");
    
    commnads.spawn(Camera2dBundle::default());
    
    commnads.spawn(
        TextBundle::from_section(
            "Count: ", TextStyle { font_size: 12f32, color: SCORE_COLOR, ..default() }),     
    );
    
}

fn print_timer(time: Res<Time>, mut timer: ResMut<NebulaTime>, query: Query<&Item>, mut commands: Commands, mut count_text: Query<&mut Text>) {
    
    if timer.0.tick(time.delta()).just_finished() {
        let count = query.iter().count();
        let text = count_text.single_mut();
        println!("Tick! Items: {count}");
        
        commands.spawn(Item { number: (count + 1) as i64 });
    };
}

struct NebulaVault;

impl Plugin for NebulaVault {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(NebulaTime(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Update, print_timer)
            .add_systems(Startup, setup);
    }
}