#![warn(clippy::pedantic)]

use bevy::app::{App, Plugin, Startup};
use bevy::math::{ivec2, vec3};
use bevy::prelude::*;
use bevy::time::{Timer, TimerMode};
use bevy::ui::{PositionType, Style};
use bevy::utils::default;
use bevy::DefaultPlugins;
use bevy_ascii_terminal::prelude::*;
use bevy_ascii_terminal::{Border, Terminal, TiledCameraBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use systems::input::{input, respawn_map};
use systems::maps::{new_map_rooms_coors, Map};
use systems::movement::input_movement;

use crate::components::bundles::PlayerBundle;
use crate::components::Position;
use crate::engine::terminal::render_all;
use crate::engine::{CLEAR_TILE, DEVMAP_SIZE, VIEWPORT_SIZE};
use crate::systems::maps::map_builder::{EventGrowBSPTree, Leaf};

mod components;
mod engine;
mod systems;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window { title: String::from("Nebula Vault"), resolution: (1600.0, 800.0).into(), ..default() }),
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
    Exploration,
}

#[derive(Resource)]
struct NebulaTime(Timer);

#[derive(Resource)]
struct PlayerMovTimer(Timer);

// Tags
#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct DevText {}

const SCORE_COLOR: Color = Color::rgb(1., 1., 1.);

fn setup_bsp(mut commands: Commands, mut t_event: EventWriter<EventGrowBSPTree>) {
    // main camera
    commands.spawn((Camera2dBundle { transform: Transform::from_translation(vec3(0., 0., 0.0)), ..default() }, MainCamera));

    spawn_dev_text(&mut commands);
}

fn spawn_dev_text(commands: &mut Commands) {
    // dev info text
    commands.spawn((
        TextBundle::from_section("", TextStyle { font_size: 15f32, color: SCORE_COLOR, ..default() }).with_style(Style {
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

    text.push_str(&format!("Leafs: {l}\n"));
}

#[derive(Component)]
pub struct GameTerminal;

fn setup_dev(mut commands: Commands) {
    // Terminal
    let mut terminal = Terminal::new([DEVMAP_SIZE[0], DEVMAP_SIZE[1]]).with_border(Border::single_line());
    terminal.clear_tile = CLEAR_TILE;
    terminal.put_string([1, 1], "Hello world!".fg(Color::BLUE));

    let term_bundle: TerminalBundle = TerminalBundle::from(terminal);
    commands.spawn((term_bundle, GameTerminal));
    // Camera
    commands.spawn(TiledCameraBundle::pixel_cam(VIEWPORT_SIZE).with_pixels_per_tile([1, 1]).with_clear_color(Color::rgb(0.1, 0.1, 0.1)));

    // Map
    let (map, rooms) = new_map_rooms_coors();
    let (player_x, player_y) = rooms[0].center();
    commands.spawn(Map(map, rooms));
    // Player
    commands.spawn(PlayerBundle { position: Position(ivec2(player_x, player_y)), ..default() });
}

struct NebulaVault;

impl Plugin for NebulaVault {
    fn build(&self, app: &mut App) {
        app
			.add_plugins((WorldInspectorPlugin::new(), TerminalPlugin))
			.add_event::<EventGrowBSPTree>()
			.insert_resource(NebulaTime(Timer::from_seconds(1.0, TimerMode::Repeating)))
			.add_systems(Startup, setup_dev)
			.add_systems(Update, (respawn_map, render_all, input_movement, input))
		//.add_systems(Update, (print_bsp_dev, input, redraw_map))
		//.add_systems(Startup, setup_bsp)
		;
        app.register_type::<Map>().register_type::<Position>();
    }
}
