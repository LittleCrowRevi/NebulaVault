#![warn(clippy::pedantic)]
#![allow(clippy::upper_case_acronyms, clippy::module_name_repetitions, unused_variables)]

use bevy::app::{App, Plugin, Startup};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::time::{Timer, TimerMode};
use bevy::utils::default;
use bevy::DefaultPlugins;
use bevy_ascii_terminal::{Border, Terminal, TiledCameraBundle};
use engine::debug::{setup_debug, update_debug_text_system};

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

mod tests;

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
            TerminalPlugin,
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

fn setup_game(mut commands: Commands) {
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
    let idx = map.xy_idx(player_spawn);
    commands.spawn(map);

    let mut player = PlayerBundle::default();
    player.entity.position = Position { xy: Point::new(player_spawn.x, player_spawn.y), idx };
    // Player
    commands.spawn(player);
}

fn ticking(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Paused);
}

struct NebulaVault;

impl Plugin for NebulaVault {
    fn build(&self, app: &mut App) {
        let core_update_systems =
            (movement_requests_handler, update_player_resource_system, index_map_system, fov_system).chain();
        app.init_state::<GameState>()
            .insert_resource(NebulaTime(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(PlayerPosition { idx: 0 })
            .add_event::<MovementRequestEvent>()
            .add_systems(Startup, (setup_game, setup_debug))
            .add_systems(Update, (render_all))
            .add_systems(Update, (dev_input_respawn_map, update_debug_text_system))
            .add_systems(Update, (input_game_system))
            .add_systems(
                Update,
                (
                    core_update_systems,
                    (input_movement_system).run_if(in_state(GameState::Paused)).after(fov_system),
                    monster_ai_system.run_if(in_state(GameState::Ticking)).after(fov_system),
                ),
            )
            .add_systems(Last, ticking.run_if(in_state(GameState::Ticking)));
    }
}
