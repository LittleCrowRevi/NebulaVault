use std::cmp::{max, min};

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::engine::rect::Point;
use crate::engine::{rect::Rect, TileType};

#[derive(Component, Debug)]
pub struct Map {
    pub rooms: Vec<Rect>,
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn idx_xy(&self, idx: usize) -> Point {
        let x = idx % self.width as usize;
        let y = idx / self.width as usize;

        Point::new(x as i32, y as i32)
    }

    pub fn size(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    pub fn paint_room_floor(&mut self, room: &Rect) {
        for p in room.points() {
            let idx = self.xy_idx(p.x, p.y);
            self.tiles[idx] = TileType::Floor;
        }
    }

    pub fn paint_htunnel(&mut self, point1: Point, point2: Point, y: i32, width: i32) {
        let (start_x, end_x) = (min(point1.x, point2.x), max(point1.x, point2.x));
        for x in start_x..=end_x {
            for i in 0..width {
                let idx = self.xy_idx(x, y + i);
                if idx > 0 && idx < self.size() {
                    self.tiles[idx] = TileType::Floor;
                }
            }
        }
    }

    pub fn paint_vtunnel(&mut self, point1: Point, point2: Point, x: i32, width: i32) {
        let (start_y, end_y) = (min(point1.y, point2.y), max(point1.y, point2.y));
        for y in start_y..=end_y {
            for i in 0..width {
                let idx = self.xy_idx(x + i, y);
                if idx > 0 && idx < self.size() {
                    self.tiles[idx] = TileType::Floor;
                }
            }
        }
    }

    pub fn generate_room(&self, min_size: i32, max_size: i32) -> Rect {
        let mut rng = thread_rng();
        let w = rng.gen_range(min_size..max_size);
        let h = rng.gen_range(min_size..max_size);
        let x = rng.gen_range(1..self.width - w - 1);
        let y = rng.gen_range(1..self.height - h - 1);
        Rect::new(x, y, w, h)
    }

    pub fn new_map_rooms_coors(size: (i32, i32)) -> Map {
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;
        let s = (size.0 * size.1) as usize;

        let mut map = Map {
            tiles: vec![TileType::Wall; s],
            rooms: Vec::new(),
            width: size.0,
            height: size.1,
            revealed_tiles: vec![false; s],
            visible_tiles: vec![false; s],
        };

        let mut rng = thread_rng();

        for _ in 0..MAX_ROOMS {
            let n_room = map.generate_room(MIN_SIZE, MAX_SIZE);

            if map.rooms.iter().any(|r| n_room.intersect(r)) {
                continue;
            }

            map.paint_room_floor(&n_room);

            if !map.rooms.is_empty() {
                let n_center = n_room.center();
                let p_center = map.rooms[map.rooms.len() - 1].center();
                if rng.gen_bool(0.5) {
                    map.paint_htunnel(p_center, n_center, p_center.y, 1);
                    map.paint_vtunnel(p_center, n_center, n_center.x, 1);
                } else {
                    map.paint_vtunnel(p_center, n_center, p_center.x, 1);
                    map.paint_htunnel(p_center, n_center, n_center.y, 1);
                }
            }
            map.rooms.push(n_room);
        }

        map
    }
}
