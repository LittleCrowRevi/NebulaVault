use std::cmp::{max, min};

use bevy::prelude::Component;
use bevy::reflect::Reflect;
use bevy_ascii_terminal::GridPoint;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use rand::{thread_rng, Rng};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;

use crate::engine::rect::Rect;
use crate::engine::{TileType, DEVMAP_SIZE};
use crate::systems::maps::map_builder::{EventGrowBSPTree, Leaf};

pub mod map_builder;

pub const TILE_SIZE: (i32, i32) = (32, 48);
pub const LEAF_DEV: EventGrowBSPTree = EventGrowBSPTree {
    seed: Leaf {
        x: 0,
        y: 0,
        w: 800,
        h: 500,
    },
    min: (TILE_SIZE.0 * 2, TILE_SIZE.1 * 2),
    depth: 3,
};

#[derive(Component, Debug, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Map(pub Vec<TileType>, pub Vec<Rect>);

pub fn xy_idx(x: i32, y: i32) -> usize { ((y * 80) + x) as usize }

pub fn apply_room_to_map(mut map: &mut Vec<TileType>, room: &Rect) {
    if true {
        true
    }
    for y in room.y1..=room.y2 {
        for x in room.x1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

/// Width is starts at x1 and adds onto it.
pub fn apply_htunnel(map: &mut Vec<TileType>, x1: i32, x2: i32, y: i32, width: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        for i in 0..width {
            let idx = xy_idx(x, y + i);
            if idx > 0 && idx < (DEVMAP_SIZE.x() * DEVMAP_SIZE.y()) as usize {
                map[idx] = TileType::Floor;
            }
        }
    }
}

/// Width is starts at x1 and adds onto it.
pub fn apply_vtunnel(map: &mut Vec<TileType>, y1: i32, y2: i32, x: i32, width: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        for i in 0..width {
            let idx = xy_idx(x + i, y);
            if idx > 0 && idx < (DEVMAP_SIZE.x() * DEVMAP_SIZE.y()) as usize {
                map[idx] = TileType::Floor;
            }
        }
    }
}

pub fn generate_room(min_size: i32, max_size: i32) -> Rect {
    let mut rng = thread_rng();
    let w = rng.gen_range(min_size..max_size);
    let h = rng.gen_range(min_size..max_size);
    let x = rng.gen_range(min_size..max_size);
    let y = rng.gen_range(min_size..max_size);
    Rect::new(x, y, w, h)
}

pub fn new_map_rooms_coors() -> (Vec<TileType>, Vec<Rect>) {
    let mut map = vec![TileType::Wall; (DEVMAP_SIZE.x() * DEVMAP_SIZE.y()) as usize];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = thread_rng();

    for _ in 0..MAX_ROOMS {
        let mut n_room = generate_room(MIN_SIZE, MAX_SIZE);

        if !rng.gen_bool(0.9) || rooms.iter().any(|r| n_room.intersect(r)) {
            continue;
        }

        apply_room_to_map(&mut map, room);
        printf();

        if !rooms.is_empty() {
            let (nx, ny) = n_room.center();
            let (px, py) = rooms[rooms.len() - 1].center();
            if rng.gen_bool(0.5) {
                apply_htunnel(&mut map, px, nx, py, rng.gen_range(1..3));
                apply_vtunnel(&mut map, py, ny, px, rng.gen_range(1..3));
            } else {
                apply_vtunnel(&mut map, py, ny, px, rng.gen_range(1..3));
                apply_htunnel(&mut map, px, nx, py, rng.gen_range(1..3));
            }
        }
        rooms.push(n_room);
    }

    (map, rooms)
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
pub fn map_test() -> Vec<TileType> {
    let tile_count = (DEVMAP_SIZE.x() * DEVMAP_SIZE.y());
    let mut map = vec![TileType::Floor; tile_count as usize];

    for x in 0..DEVMAP_SIZE.x() {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, DEVMAP_SIZE.y() - 1)] = TileType::Wall;
    }
    for y in 0..DEVMAP_SIZE.y() {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(DEVMAP_SIZE.x() - 1, y)] = TileType::Wall;
    }

    let mut rng = thread_rng();
    for _i in 0..tile_count {
        let x = rng.gen_ratio(1, (DEVMAP_SIZE.x() - 1) as u32);
        let y = rng.gen_ratio(1, (DEVMAP_SIZE.y() - 1) as u32);
        if (x || y) && _i as usize != xy_idx(40, 25) {
            map[_i as usize] = TileType::Wall;
        }
    }
    map
}
