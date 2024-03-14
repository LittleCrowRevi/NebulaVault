use bevy::{
    math::Rect,
    math::{ivec2, IVec2},
    prelude::{Color, Component, Query, With},
};
use bevy_ascii_terminal::{GridPoint, Terminal, Tile, TileFormatter};

use crate::prelude::*;

#[derive(Component)]
pub struct GameTerminal;

pub fn render_all(
    mut query_r: Query<(&Renderable, &Position, Option<&PlayerMarker>)>,
    query_view: Query<&Viewshed>,
    mut query_term: Query<&mut Terminal, With<GameTerminal>>,
    query_map: Query<&Map>,
) {
    let mut term = query_term.single_mut();
    term.clear();

    for map in &query_map {
        
        render_map(map, &mut term);

        for (r, pos, player) in &query_r {
            let idx = map.xy_idx(Point::new(pos.0.x, pos.0.y));

            if map.visible_tiles.contains(&idx) || player.is_some() {
                term.put_tile(ivec2(pos.0.x, pos.0.y), Tile::from(r));
            }
        }
    }
}

pub fn render_map(map: &Map, term: &mut Terminal) {
    for (idx, tile) in map.tiles.iter().enumerate() {
        // Move the coordinates
        let p = map.idx_xy(idx);
        if !map.revealed_tiles.contains(&idx) && !DEBUG_MODE {
            continue;
        }
        let mut t: Tile = match tile {
            TileType::Floor => VFLOOR_TILE,
            _ => WALL_TILE,
        };
        if !map.visible_tiles.contains(&idx) {
            t.fg_color = Color::rgb(0.1, 0.1, 0.1);
        }
        term.put_tile(IVec2::from(p), t);
    }
}
