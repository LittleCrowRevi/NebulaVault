use bevy::{prelude::*, reflect::List};

use super::map_gen::map::Map;
use crate::{
    components::{PlayerMarker, Position, Viewshed},
    engine::rect::Point,
};

pub fn system_visibility(
    mut query_v: Query<(&mut Viewshed, &Position, Option<&PlayerMarker>)>,
    mut query_map: Query<&mut Map>,
) {
    let mut map = query_map.single_mut();
    for (mut viewshed, pos, player) in &mut query_v {
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles = fov(Point::new(pos.0.x, pos.0.y), viewshed.range, &map);
        viewshed
            .visible_tiles
            .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

        if player.is_some() {
            map.visible_tiles.iter_mut().for_each(|t| *t = false);
            for vis in &viewshed.visible_tiles {
                let idx = map.xy_idx(vis.x, vis.y);
                map.revealed_tiles[idx] = true;
                map.visible_tiles[idx] = true;
            }
        }
    }
}

pub fn fov(pos: Point, range: i32, map: &Map) -> Vec<Point> {
    let mut tiles = Vec::new();
    for x in -range..=range {
        for y in -range..=range {
            tiles.push(Point::new(pos.x + x, pos.y + y));
        }
    }
    tiles
}
