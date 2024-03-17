#![allow(dead_code)]

use std::thread::current;

use bevy::{a11y::Focus, utils::HashSet};
use bracket_pathfinding::prelude::BaseMap;

use crate::prelude::*;

pub const CARDINAL: [(i32, i32); 8] =
    //      N       NE       E       ES       S       SW        W       WN
    [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
pub const CARDINALS: [((i32, i32), (i32, i32)); 8] = [
    ((1, 0), (0, 1)),   // 0 E-NE
    ((0, 1), (1, 0)),   // 1 NE-N
    ((0, -1), (1, 0)),  // 2 N-NW
    ((-1, 0), (0, 1)),  // 3 NW-W
    ((-1, 0), (0, -1)), // 4 W-SW
    ((0, -1), (-1, 0)), // 5 SW-S
    ((0, 1), (-1, 0)),  // 6 S-SE
    ((1, 0), (0, -1)),  // 7 SE-E
];

pub const DIAGONALS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

#[derive(Copy, Clone)]
pub struct Octant {
    pub depth_direction: Point,
    pub end_vec: (i32, i32),
    pub scan_direction: Point,
    pub left_view_slope: f32,
    pub right_view_slope: f32,
}

impl Octant {
    pub fn new(depth_direction: Point, end_vec: (i32, i32), scan_direction: Point) -> Self {
        Self { depth_direction, end_vec, scan_direction, left_view_slope: 1., right_view_slope: 0. }
    }
}

// TODO: refactor into using visiblle_tiles from each individual viewshed instead of always creating a new one
pub fn fov_system(
    mut q_map: Query<&mut Map>,
    mut query: Query<(&mut Viewshed, &Position, &Renderable, Option<&PlayerMarker>)>,
) {
    let mut map = q_map.single_mut();

    for (mut viewshed, pos, renderable, player) in &mut query {
        if viewshed.dirty {
            continue;
        }

        viewshed.previous_tiles = viewshed.visible_tiles.clone();
        viewshed.visible_tiles.clear();

        viewshed.visible_tiles = recursive_shadowrevi(&map, viewshed.range, Point::new(pos.xy.x, pos.xy.y));
        //viewshed.visible_tiles = recursive_shadowcast_ba(pos.0, viewshed.range, &map);

        viewshed.visible_tiles.retain(|i| {
            let p = map.idx_xy(*i);
            p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height
        });
        

        if player.is_some() {
            map.visible_tiles.clear();
            for tile in &viewshed.visible_tiles {
                map.visible_tiles.insert(*tile);
                map.revealed_tiles.insert(*tile);
            }
        }

        viewshed.dirty = true;
    }
}

pub fn recursive_shadowrevi(map: &Map, range: i32, pos: Point) -> HashSet<usize> {
    let mut visible_tiles = HashSet::new();
    for d in DIAGONALS {
        let octant_one = Octant::new(Point::new(0, d.0), (0, 0), Point::new(d.1, 0));
        let octant_two = Octant::new(Point::new(d.0, 0), (0, 0), Point::new(0, d.1));

        cast_light(map, &mut visible_tiles, range, 1, pos, octant_one);
        cast_light(map, &mut visible_tiles, range, 1, pos, octant_two);
    }
    visible_tiles
}

pub fn cast_light(
    map: &Map,
    visible_tiles: &mut HashSet<usize>,
    range: i32,
    start_depth: i32,
    start_pos: Point,
    mut octant: Octant,
) {
    if range < 0 {
        return;
    }

    let mut depth = start_depth;
    let mut saved_right_slope = -1.0;

    while depth <= range + 1 {
        //let mut current_cell = start_pos;
        //let mut first_tcell = None;
        let mut encountered_blocking = false;

        let dy = -depth;
        for scan in -depth..1 {
            let cell_x = start_pos.x + dy * octant.depth_direction.x + scan * octant.depth_direction.y;
            let cell_y = start_pos.y + dy * octant.scan_direction.x + scan * octant.scan_direction.y;

            let cell = Point::new(cell_x, cell_y);
            let cell_idx = map.xy_idx(cell);
            if cell_idx >= map.size() {
                continue;
            }

            let left_slope = (scan as f32 - 0.5) / (dy as f32 + 0.5);
            let right_slope = (scan as f32 + 0.5) / (dy as f32 - 0.5);

            if right_slope >= octant.left_view_slope {
                continue;
            }
            if left_slope <= octant.right_view_slope {
                break;
            }

            let is_opaque = map.is_opaque(cell_idx);
            
            // to skip walls
            let can_see = if is_opaque { true } else { cell.can_see(start_pos, map) };
            
            let distance_squared = depth * depth + scan * scan;
            
            if distance_squared <= range * range && can_see {
                visible_tiles.insert(cell_idx);
            }
            

            if encountered_blocking {
                if is_opaque {
                    saved_right_slope = right_slope;
                } else {
                    encountered_blocking = false;
                    octant.left_view_slope = saved_right_slope;
                }
            } else if is_opaque {
                if left_slope <= octant.left_view_slope {
                    let mut n_octant = octant;
                    n_octant.right_view_slope = left_slope;
                    cast_light(map, visible_tiles, range, depth + 1, start_pos, n_octant);
                }

                encountered_blocking = true;
                saved_right_slope = right_slope;
            }
        }

        if encountered_blocking {
            break;
        }
        //start_pos += octant.depth_direction;
        depth += 1;
    }
}

fn is_symmetric(left_slope: f32, right_slope: f32, depth: f32, column: f32) -> bool {
    column >= depth * left_slope && column <= depth * right_slope
}

pub fn recursive_shadowcast_revi(map: &Map, range: i32, pos: Point) -> HashSet<usize> {
    let mut visible_tiles = HashSet::new();
    for d in DIAGONALS {
        let octant_one = Octant::new(Point::new(0, d.0), (0, 0), Point::new(d.1, 0));
        let octant_two = Octant::new(Point::new(d.0, 0), (0, 0), Point::new(0, d.1));

        calc_cast_light(map, &mut visible_tiles, range, pos, None, 1, octant_one);
        calc_cast_light(map, &mut visible_tiles, range, pos, None, 1, octant_two);
    }
    visible_tiles
}

pub fn calc_cast_light(
    map: &Map,
    visible_tiles: &mut HashSet<usize>,
    range: i32,
    start_pos: Point,
    stop_cell: Option<Point>,
    start_depth: i32,
    octant: Octant,
) {
    if start_depth > range {
        return;
    }

    let mut continue_loop = true;
    let mut r = range;

    for depth in 1..=range {
        let mut encountered_blocking = false;
        let mut saved_transparent: Option<Point> = None;

        let depth_position = start_pos + (octant.depth_direction * depth);
        let mut previous_cell = depth_position;

        for scan in 0..=depth {
            let current_cell = depth_position + octant.scan_direction * scan;
            let current_cell_idx = map.xy_idx(current_cell);

            if current_cell_idx >= map.size() {
                continue;
            }

            if stop_cell.is_some()
                && current_cell == stop_cell.unwrap()
                && !encountered_blocking
                && saved_transparent.is_some()
            {
                let start_cell = saved_transparent.unwrap();
                calc_cast_light(
                    map,
                    visible_tiles,
                    r - 1,
                    start_cell,
                    Some(previous_cell + octant.depth_direction),
                    depth + 1,
                    octant,
                );
                continue_loop = false;
                break;
            }

            let is_opaque = map.is_opaque(current_cell_idx);

            visible_tiles.insert(current_cell_idx);

            if is_opaque {
                if !encountered_blocking && saved_transparent.is_some() {
                    let start_cell = saved_transparent.unwrap();
                    calc_cast_light(
                        map,
                        visible_tiles,
                        r - 1,
                        start_cell,
                        Some(previous_cell + octant.depth_direction),
                        depth + 1,
                        octant,
                    );
                    saved_transparent = None;
                    continue_loop = false;
                }

                encountered_blocking = true;
            } else {
                saved_transparent = match saved_transparent {
                    None => Some(current_cell),
                    _ => saved_transparent,
                };

                encountered_blocking = false;
            }

            previous_cell = current_cell;
        }

        r -= 1;

        if !continue_loop {
            break;
        }
    }
}

pub fn calc_octant(map: &Map, range: i32, pos: Point) -> (HashSet<usize>, HashSet<Vec<usize>>) {
    let mut visible_tiles: HashSet<usize> = HashSet::new();
    let mut octant_borders = HashSet::new();

    for (idx, c) in CARDINAL.iter().enumerate() {
        let mut encountered_opaque = false;
        let mut octant = Vec::new();
        let nidx = if idx >= CARDINAL.len() { 0 } else { idx + 1 };

        for i in 0..=range {
            let v = (pos.x + c.0 * i, pos.y + c.1 * i);
            let idx = map.xy_idx(Point::new(v.0, v.1));
            if idx >= map.tiles.len() {
                continue;
            }

            octant.push(idx);

            if !encountered_opaque {
                visible_tiles.insert(idx);
            }

            if map.is_opaque(idx) {
                encountered_opaque = true;
            }
        }
        octant_borders.insert(octant);
    }
    (visible_tiles, octant_borders)
}

/*
min(current_cell, stop_cell.unwrap()) >= max(current_cell, stop_cell.unwrap())
while r >= 0 && cont_loop {
        let mut current_cell = start_pos;
        let mut first_tcell = None;
        let mut encountered_blocking = false;

        while true {
            let current_cell_idx = map.xy_idx(current_cell);
            if current_cell_idx >= map.size() {
                continue;
            }

            if !map.is_opaque(current_cell_idx) {
                if encountered_blocking {
                    encountered_blocking = false;
                }
                if first_tcell.is_none() {
                    first_tcell = Some(current_cell);
                }
            } else {
                cont_loop = false;
                if !encountered_blocking && first_tcell.is_some() {
                    let mut new_octant = octant.clone();
                    //new_octant.end_vec = new_octant.depth_direction;
                    //calc_cast_light(map, r - 1, visible_tiles, first_tcell.unwrap() + octant.depth_direction, current_cell + octant.depth_direction,  new_octant);
                    first_tcell = None;
                }
                encountered_blocking = true;
            }

            visible_tiles.insert(current_cell_idx);
            //current_cell += octant.scan_direction;
        }
        //start_pos += octant.depth_direction;
        r -= 1;
    }
*/
/*
pub fn calc_vis(
    map: &Map,
    fov_pos: Point,
    range: i32,
    depth: i32,
    mut start_cell: Point,
    mut end_cell: Point,
    mut visible_tiles: HashSet<usize>,
    octant: Octant,
) -> HashSet<usize> {
    if range < 0 {
        return visible_tiles;
    }

    let mut current_cell = start_cell;
    let mut previous_cell = current_cell - octant.scan_direction;
    let mut cont = true;
    let mut encountered_blocking = false;

    'outer: while min(current_cell, end_cell) < max(current_cell, end_cell) {
        let cell_idx = map.xy_idx(current_cell);
        let vec_to_fov = fov_pos - (current_cell.x, current_cell.y);
        if cell_idx >= map.size() {
            break;
        }

        let mut pdeth_cell = current_cell;
        for _ in 0..depth {
            pdeth_cell = pdeth_cell + vec_to_fov;
            let pdeth_cell_idx = map.xy_idx(pdeth_cell);

            if pdeth_cell_idx < map.size() && map.is_opaque(pdeth_cell_idx) {
                previous_cell = current_cell;
                current_cell += octant.scan_direction;
                continue 'outer;
            }
        }

        visible_tiles.insert(cell_idx);

        if map.is_opaque(cell_idx) {
            encountered_blocking = true;

            let previous_cell_idx = map.xy_idx(previous_cell);
            if previous_cell_idx >= map.size() {
                break;
            }

            if !map.is_opaque(previous_cell_idx) {
                visible_tiles = calc_vis(
                    map,
                    fov_pos,
                    range - 1,
                    depth + 1,
                    start_cell + octant.depth_direction,
                    previous_cell + octant.end_vec,
                    visible_tiles,
                    octant,
                );
            }

            if current_cell + octant.end_vec == end_cell {
                cont = false;
            }
        } else if encountered_blocking {
            start_cell = current_cell;
            encountered_blocking = false;
        }

        previous_cell = current_cell;
        current_cell += octant.scan_direction;
    }

    if cont {
        start_cell += octant.depth_direction;
        end_cell += octant.end_vec;
        calc_vis(map, fov_pos, range - 1, depth + 1, start_cell, end_cell, visible_tiles, octant)
    } else {
        visible_tiles
    }
}*/
