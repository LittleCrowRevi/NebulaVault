use bevy::{
    math::IVec2,
    prelude::{Color, Component, Query, With},
};
use bevy_ascii_terminal::{GridPoint, Terminal, Tile, TileFormatter};

use crate::prelude::*;

#[derive(Component)]
pub struct GameTerminal;

pub fn render_all(
    mut query_r: Query<(&Renderable, &Position)>,
    query_view: Query<&Viewshed>,
    mut query_term: Query<&mut Terminal, With<GameTerminal>>,
    query_map: Query<&Map>,
) {
    let mut term = query_term.single_mut();
    term.clear();

    for map in &query_map {
        render_map(map, &mut term);
    }

    for (r, pos) in query_r.iter() {
        term.put_tile(pos.0, Tile::from(r));
    }
}

pub fn render_map(map: &Map, term: &mut Terminal) {
    let mut x = -1;
    let mut y = 0;

    for (idx, tile) in map.tiles.iter().enumerate() {
        // Move the coordinates
        x += 1;
        if x > map.width - 1 {
            x = 0;
            y += 1;
        }
        if !map.revealed_tiles[idx] {
            continue;
        }
        let mut t: Tile = match tile {
            TileType::Floor => CLEAR_TILE,
            _ => Tile { glyph: '#', fg_color: Color::GREEN, ..Default::default() },
        };
        if !map.visible_tiles[idx] {
            t.fg_color = Color::GRAY;
        }
        term.put_tile(IVec2::new(x, y), t);
    }
}
