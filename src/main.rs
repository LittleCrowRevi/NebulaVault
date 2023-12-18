#![warn(clippy::pedantic)]

mod entities;
mod systems;

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
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use entities::races::*;
use entities::components::*;
use systems::maps::map_builder::generate_bsp;
use crate::systems::input::input;
use crate::systems::maps::LEAF_DEV;
use crate::systems::maps::map_builder::{EventGrowBSPTree, Leaf, redraw_map};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Nebula Vault"),
                    resolution: (1600.0, 800.0).into(),
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

// Tags
#[derive(Component)]
struct TagCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct DevText {}

const SCORE_COLOR: Color = Color::rgb(1., 1., 1.);

fn setup_bsp(mut commands: Commands, mut t_event: EventWriter<EventGrowBSPTree>) {

    // main camera
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(vec3(0., 0., 0.0)),
            ..default()
        },
        TagCamera,
    ));
    
    spawn_dev_text(&mut commands);
    t_event.send(LEAF_DEV);
}

fn spawn_dev_text(commands: &mut Commands) {
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
        DevText {},
    ));
}

fn print_bsp_dev(mut commands: Commands, leafs: Query<&Leaf>, mut dev_text: Query<(&mut Text, &mut DevText)>) {
    let text = &mut dev_text.single_mut().0.sections[0].value;
    let l = leafs.iter().len();
    *text = "DevText\n".to_string();
    
    text.push_str(&format!("Leafs: {}\n", l));
}

struct NebulaVault;

impl Plugin for NebulaVault {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(WorldInspectorPlugin::new())
            .add_event::<EventGrowBSPTree>()
            .add_state::<GameState>()
            .insert_resource(NebulaTime(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, (print_bsp_dev, input, redraw_map))
            .add_systems(Startup, setup_bsp);
    }
}
