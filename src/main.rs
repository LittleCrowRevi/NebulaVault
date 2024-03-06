#![warn(clippy::pedantic)]

use bevy::app::{App, Plugin, Startup};
use bevy::math::ivec2;
use bevy::time::{Timer, TimerMode};
use bevy::utils::default;
use bevy::DefaultPlugins;
use bevy_ascii_terminal::{Border, Terminal, TiledCameraBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use engine::debug::{setup_debug, update_debug_text};

mod components;
mod engine;
mod systems;
mod tests;

mod prelude {
    pub use std::cmp::{max, min};

    pub use bevy::prelude::*;
    pub use bevy_ascii_terminal::prelude::*;
    pub use rand::{thread_rng, Rng};

    pub use crate::components::*;
    pub use crate::engine::rect::{Point, Rect};
    pub use crate::engine::*;
    pub use crate::systems::*;
}

use prelude::*;

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
    Exploration,
}

fn setup_dev(mut commands: Commands) {
    // Terminal
    let mut terminal =
        Terminal::new([DEVMAP_SIZE[0], DEVMAP_SIZE[1]]).with_border(Border::single_line());
    terminal.clear_tile = CLEAR_TILE;
    terminal.put_string([1, 1], "Hello world!".fg(Color::BLUE));

    let term_bundle: TerminalBundle = TerminalBundle::from(terminal);
    commands.spawn((term_bundle, GameTerminal));
    // Camera
    commands.spawn(
        TiledCameraBundle::pixel_cam(VIEWPORT_SIZE)
            .with_pixels_per_tile([1, 1])
            .with_clear_color(Color::rgb(0.1, 0.1, 0.1)),
    );

    // Map
    let map = Map::new_map_rooms_coors((80, 50));
    let player_spawn = map.rooms[0].center();
    commands.spawn(map);
    // Player
    commands.spawn(PlayerBundle {
        position: Position(ivec2(player_spawn.x, player_spawn.y)),
        ..default()
    });
}

struct NebulaVault;

impl Plugin for NebulaVault {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldInspectorPlugin::new(), TerminalPlugin))
            .insert_resource(NebulaTime(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Startup, (setup_dev, setup_debug))
            .add_systems(Update, (respawn_map, render_all, input_movement, input, system_visibility, update_debug_text))
        //.add_systems(Update, (print_bsp_dev, input, redraw_map))
        //.add_systems(Startup, setup_bsp)
        ;
    }
}
