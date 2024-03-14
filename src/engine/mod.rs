use bevy_ascii_terminal::Tile;

use crate::prelude::*;

pub mod debug;
pub mod rect;
pub mod render;

pub use debug::*;
pub use render::*;

pub const VIEWPORT_SIZE: [u32; 2] = [82, 52];
pub const DEVMAP_SIZE: [u32; 2] = [80, 50];
pub const FLOOR_TILE: Tile = Tile { glyph: '·', bg_color: Color::BLACK, fg_color: Color::rgb(0.3, 0.3, 0.3) };
pub const WALL_TILE: Tile = Tile { glyph: '#', bg_color: Color::BLACK, fg_color: Color::rgb(0.3, 0.3, 0.8) };
pub const VFLOOR_TILE: Tile = Tile { glyph: '·', bg_color: Color::BLACK, fg_color: Color::rgb(0.3, 0.8, 0.7) };

#[derive(Resource)]
pub struct NebulaTime(pub Timer);

#[derive(PartialEq, Debug, Copy, Clone, Component)]
pub enum TileType {
    Wall,
    Floor,
    Player,
    Enemy,
    Npc,
    Item,
    Object,
    Void,
}
