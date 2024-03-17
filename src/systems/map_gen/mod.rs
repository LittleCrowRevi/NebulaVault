#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use bevy::utils::smallvec::SmallVec;
use bevy::utils::HashSet;
use bevy_ascii_terminal::GridPoint;

use crate::engine::rect::{Point, Rect};
use crate::prelude::*;

pub mod tiles;
pub use tiles::*;
pub mod map;
pub use map::*;

pub fn index_map_system(
    mut query_map: Query<&mut Map>,
    query_blocking: Query<(&Position, &EntityType, Entity, Option<&BlocksTile>)>,
) {
    let mut map = query_map.single_mut();
    map.clear_tiles();
    map.fill_blocking();

    for (pos, e_type, entity, blocks) in &query_blocking {
        if pos.idx > map.size() {
            continue;
        }
        map.tile_entities[pos.idx].push((entity, *e_type));

        if blocks.is_some() {
            map.walkable[pos.idx] = false;
       }
    }
}
