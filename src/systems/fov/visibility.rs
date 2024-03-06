#![allow(dead_code)]

use crate::prelude::*;

pub fn system_visibility(
    mut query_v: Query<(&mut Viewshed, &Position, Option<&PlayerMarker>)>,
    mut query_map: Query<&mut Map>,
) {
    let mut map = query_map.single_mut();
    for (mut viewshed, pos, player) in &mut query_v {
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles = fov_bsp(Point::new(pos.0.x, pos.0.y), viewshed.range, &map);
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

fn fov_bsp(pos: Point, r: i32, map: &Map) -> Vec<Point> {
    let range = r;
    let mut visible_tiles: Vec<Point> = Vec::new();
    let mut next_pos: Vec<Point> = Vec::new();
    let n = Point::new(pos.x, pos.y + 1);
    fov_check_tile(n, map, &mut visible_tiles, &mut next_pos);
    fov_bsp_traverse(n, range, map, Cardinal::N, &mut visible_tiles, &mut next_pos);
    /*
    let ne = Point::new(pos.x + 1, pos.y + 1);
    fov_check_tile(ne, map, &mut visible_tiles, &mut next_pos);
    fov_bsp_traverse(ne, range, map, Cardinal::E, &mut visible_tiles, &mut next_pos);

    let e = Point::new(pos.x + 1, pos.y);
    fov_check_tile(e, map, &mut visible_tiles, &mut next_pos);
    fov_bsp_traverse(e, range, map, Cardinal::E, &mut visible_tiles, &mut next_pos);

    let es = Point::new(pos.x + 1, pos.y - 1);
        fov_check_tile(es, map, &mut visible_tiles, &mut next_pos);
        fov_bsp_traverse(es, range, map, Cardinal::S, &mut visible_tiles, &mut next_pos);

        let s = Point::new(pos.x, pos.y - 1);
        fov_check_tile(s, map, &mut visible_tiles, &mut next_pos);
        fov_bsp_traverse(s, range, map, Cardinal::S, &mut visible_tiles, &mut next_pos);

        let sw = Point::new(pos.x - 1, pos.y - 1);
        fov_check_tile(sw, map, &mut visible_tiles, &mut next_pos);
        fov_bsp_traverse(sw, range, map, Cardinal::W, &mut visible_tiles, &mut next_pos);

        let w = Point::new(pos.x - 1, pos.y);
        fov_check_tile(w, map, &mut visible_tiles, &mut next_pos);
        fov_bsp_traverse(w, range, map, Cardinal::W, &mut visible_tiles, &mut next_pos);

        let wn = Point::new(pos.x - 1, pos.y + 1);
        fov_check_tile(wn, map, &mut visible_tiles, &mut next_pos);
        fov_bsp_traverse(wn, range, map, Cardinal::N, &mut visible_tiles, &mut next_pos);
    */
    visible_tiles
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Cardinal {
    N,
    E,
    S,
    W,
}

fn fov_bsp_traverse(
    pos: Point,
    range: i32,
    map: &Map,
    direction: Cardinal,
    visible_tiles: &mut Vec<Point>,
    next_pos: &mut Vec<Point>,
) {
    if range <= 0 || !next_pos.contains(&pos) {
        return;
    }
    for i in -1..=1 {
        let p = match direction {
            Cardinal::N => Point::new(pos.x + i, pos.y + 1),
            Cardinal::E => Point::new(pos.x + 1, pos.y + i),
            Cardinal::S => Point::new(pos.x + i, pos.y - 1),
            Cardinal::W => Point::new(pos.x - 1, pos.y + i),
        };
        fov_check_tile(p, map, visible_tiles, next_pos);
        fov_bsp_traverse(p, range - 1, map, direction, visible_tiles, next_pos);
    }
}

fn fov_check_tile(
    pos: Point,
    map: &Map,
    visible_tiles: &mut Vec<Point>,
    next_pos: &mut Vec<Point>,
) {
    if !map.is_opaque(map.xy_idx(pos.x, pos.y)) {
        next_pos.push(pos);
    }
    visible_tiles.push(pos);
}

fn fov(pos: Point, range: i32, map: &Map) -> Vec<Point> {
    let mut tiles = Vec::new();
    for x in -range..=range {
        for y in -range..=range {
            tiles.push(Point::new(pos.x + x, pos.y + y));
        }
    }
    tiles
}
