use bevy_ascii_terminal::Tile;

use crate::prelude::*;

pub mod debug;
pub mod rect;
pub mod render;

pub use debug::*;
pub use render::*;

pub const VIEWPORT_SIZE: [u32; 2] = [82, 52];
pub const DEVMAP_SIZE: [u32; 2] = [80, 50];

#[derive(Resource)]
pub struct NebulaTime(pub Timer);
