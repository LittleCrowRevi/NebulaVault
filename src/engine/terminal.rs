use bevy::{
    math::IVec2,
    prelude::{Color, Component, Query, With},
};
use bevy_ascii_terminal::{GridPoint, Terminal, Tile, TileFormatter};

use super::{TileType, CLEAR_TILE};
use crate::components::{Position, Renderable};
use crate::systems::map_gen::map::Map;
use crate::GameTerminal;
use crate::DEVMAP_SIZE;

pub fn render_all(
    mut query_r: Query<(&Renderable, &Position)>,
    mut query_term: Query<&mut Terminal, With<GameTerminal>>,
    query_map: Query<&Map>,
) {
    let mut term = query_term.single_mut();
    term.clear();

    for map in &query_map {
        render_map(&map.tiles, &mut term);
    }

    for (r, pos) in query_r.iter() {
        term.put_tile(pos.0, Tile::from(r));
    }
}

pub fn render_map(map: &Vec<TileType>, mut term: &mut Terminal) {
    let mut x = 0;
    let mut y = 0;

    for tile in map {
        match tile {
            TileType::Wall => term.put_tile(
                IVec2::new(x, y),
                Tile { glyph: '#', fg_color: Color::GREEN, ..Default::default() },
            ),
            TileType::Floor => term.put_tile(IVec2::new(x, y), CLEAR_TILE),
            _ => {}
        }

        // Move the coordinates
        x += 1;
        if x > DEVMAP_SIZE.x() - 1 {
            x = 0;
            y += 1;
        }
    }
}
