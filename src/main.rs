#![warn(clippy::pedantic)]

use bevy::app::{App, Plugin, Startup};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::time::{Timer, TimerMode};
use bevy::utils::default;
use bevy::DefaultPlugins;
use bevy_ascii_terminal::{Border, Terminal, TiledCameraBundle};
use engine::debug::{setup_debug, update_debug_text};

mod components;
mod engine;
mod systems;

mod prelude {
    pub use std::cmp::{max, min};

    pub use bevy::prelude::*;
    pub use bevy_ascii_terminal::prelude::*;
    pub use rand::{thread_rng, Rng};

    pub use crate::components::*;
    pub use crate::engine::rect::{Point, Rect};
    pub use crate::engine::*;
    pub use crate::systems::*;

    pub const DEBUG_MODE: bool = false;
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
            FrameTimeDiagnosticsPlugin,
            NebulaVault,
        ))
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Paused,
    Ticking,
}

#[derive(Resource)]
pub struct PlayerPosition {
    idx: usize,
}

fn setup_dev(mut commands: Commands) {
    // Terminal
    let terminal = Terminal::new([DEVMAP_SIZE[0], DEVMAP_SIZE[1]]).with_border(Border::single_line());
    //terminal.clear_tile = FLOOR_TILE;

    let term_bundle: TerminalBundle = TerminalBundle::from(terminal);
    commands.spawn((term_bundle, GameTerminal));
    // Camera
    commands.spawn(
        TiledCameraBundle::pixel_cam(VIEWPORT_SIZE)
            .with_pixels_per_tile([1, 1])
            .with_clear_color(Color::rgb(0.1, 0.1, 0.1)),
    );

    // Map
    let map = Map::new_map_rooms_coors(&mut commands, (80, 50));
    let player_spawn = map.rooms[0].center();
    commands.spawn(map);
    // Player
    commands.spawn(PlayerBundle { position: Position(Point::new(player_spawn.x, player_spawn.y)), ..default() });
}

fn ticking(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Paused);
}

struct NebulaVault;

impl Plugin for NebulaVault {
    fn build(&self, app: &mut App) {
        app.add_plugins((TerminalPlugin))
            .init_state::<GameState>()
            .insert_resource(NebulaTime(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(PlayerPosition { idx: 0 })
            .add_systems(Startup, (setup_dev, setup_debug))
            .add_systems(Update, (respawn_map, render_all, input_movement, input, fov_system, update_debug_text))
        //.add_systems(Update, (print_bsp_dev, input, redraw_map))
        //.add_systems(Startup, setup_bsp)
        ;

        app.add_systems(OnEnter(GameState::Ticking), (monster_ai_system));
        app.add_systems(Last, ticking.run_if(in_state(GameState::Ticking)));
    }
}
