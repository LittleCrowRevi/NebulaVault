pub use crate::prelude::*;

pub const FLOOR_TILE: Tile = Tile { glyph: '·', bg_color: Color::BLACK, fg_color: Color::rgb(0.3, 0.3, 0.3) };
pub const WALL_TILE: Tile = Tile { glyph: '#', bg_color: Color::BLACK, fg_color: Color::rgb(0.3, 0.3, 0.8) };
pub const VFLOOR_TILE: Tile = Tile { glyph: '·', bg_color: Color::BLACK, fg_color: Color::rgb(0.3, 0.8, 0.7) };

#[derive(PartialEq, Debug, Copy, Clone, Component)]
pub enum TileType {
    Floor = 1,
    SwallowWater = 2,
    Wall = 98,
    Void = 99,
}

pub fn is_tile_opaque(tile: TileType) -> bool {
    matches!(tile, TileType::Wall)
}

pub fn is_walkable(tile: TileType) -> bool {
    !matches!(tile, TileType::Wall | TileType::Void)
}
