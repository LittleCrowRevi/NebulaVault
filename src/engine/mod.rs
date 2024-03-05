use bevy::prelude::{Color, Reflect};
use bevy_ascii_terminal::Tile;

pub mod input;
pub mod rect;
pub mod terminal;

pub const VIEWPORT_SIZE: [u32; 2] = [82, 52];
pub const DEVMAP_SIZE: [u32; 2] = [80, 50];
pub const CLEAR_TILE: Tile = Tile {
    glyph: '·',
    bg_color: Color::BLACK,
    fg_color: Color::rgb(0.3, 0.3, 0.3),
};

#[derive(PartialEq, Debug, Copy, Clone, Reflect)]
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
