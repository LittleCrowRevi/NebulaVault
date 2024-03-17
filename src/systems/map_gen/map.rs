#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use bevy::utils::smallvec::SmallVec;
use bevy::utils::HashSet;
use bevy_ascii_terminal::GridPoint;
use bracket_pathfinding::prelude::{BaseMap, DistanceAlg};

use crate::engine::rect::{Point, Rect};
use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,

    pub rooms: Vec<Rect>,
    pub tiles: Vec<TileType>,
    pub tile_entities: Vec<Vec<(Entity, EntityType)>>,
    pub walkable: Vec<bool>,

    pub revealed_tiles: HashSet<usize>,
    pub visible_tiles: HashSet<usize>,
}

impl Map {
    pub fn xy_idx(&self, point: Point) -> usize {
        ((point.y * self.width) + point.x) as usize
    }

    pub fn idx_xy(&self, idx: usize) -> Point {
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;

        Point::new(x, y)
    }

    pub fn out_of_bounds(&self, point: Point) -> bool {
        point.x < 0 || point.x > self.width || point.y < 0 || point.y > self.height
    }

    pub fn size(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn is_tile_traversable(&self, pos: Point) -> bool {
        if pos.x < 1 || pos.x > self.width - 1 || pos.y < 1 || pos.y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(pos);

        let b = self.walkable[idx];

        b
        //self.tiles[idx] != TileType::Wall
    }

    pub fn clear_tiles(&mut self) {
        for c in &mut self.tile_entities {
            c.clear();
        }
    }

    pub fn fill_blocking(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.walkable[i] = is_walkable(*tile);
        }
    }

    pub fn get_neighbours_cost(&self, point: Point) -> Vec<(Point, f32)> {
        let neighbours = point.neighbors();

        let mut exits = Vec::new();

        for (n, cost) in &neighbours {
            if self.is_tile_traversable(*n) {
                exits.push((*n, *cost));
            };
        }

        exits
    }

    pub fn paint_room_floor(&mut self, room: &Rect) {
        for p in room.points() {
            let idx = self.xy_idx(p);
            self.tiles[idx] = TileType::Floor;
        }

        for o in room.outer_rim() {
            let idx = self.xy_idx(o);
            self.tiles[idx] = TileType::Wall;
        }
    }

    pub fn paint_htunnel(&mut self, point1: Point, point2: Point, y: i32, width: i32) {
        let (start_x, end_x) = (min(point1.x, point2.x), max(point1.x, point2.x));
        for x in start_x..=end_x {
            for i in 0..width {
                let idx = self.xy_idx(Point::new(x, y + i));
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
                let idx = self.xy_idx(Point::new(x + i, y));
                if idx > 0 && idx < self.size() {
                    self.tiles[idx] = TileType::Floor;
                }
            }
        }
    }

    pub fn generate_room(&self, min_size: i32, max_size: i32) -> Rect {
        let mut rng = thread_rng();
        // TODO: REMEMBER THIS SHOULDNT BE INCLUSIVE =max_size, NOR -1 ON RECT
        let w = rng.gen_range(min_size..=max_size);
        let h = rng.gen_range(min_size..=max_size);
        let x = rng.gen_range(1..self.width - w - 1);
        let y = rng.gen_range(1..self.height - h - 1);
        Rect::new(x - 1, y - 1, w, h)
    }

    pub fn new_map_rooms_coors(commands: &mut Commands, size: (i32, i32)) -> Map {
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 20;
        let s = (size.0 * size.1) as usize;

        let mut map = Map {
            tiles: vec![TileType::Wall; s],
            rooms: Vec::new(),
            width: size.0,
            height: size.1,
            revealed_tiles: HashSet::new(),
            visible_tiles: HashSet::new(),
            tile_entities: vec![Vec::new(); s],
            walkable: vec![false; s],
        };

        let mut rng = thread_rng();

        'outer: for _ in 0..MAX_ROOMS {
            let n_room: Rect = map.generate_room(MIN_SIZE, MAX_SIZE);

            if map.rooms.iter().any(|r: &Rect| r.intersect(&n_room, 1)) {
                continue 'outer;
            }

            map.paint_room_floor(&n_room);
            map.rooms.push(n_room);
        }

        let rooms = map.rooms.clone();

        for room in rooms {
            let n_center = room.center();
            let p_center = map.rooms[map.rooms.len() - 1].center();
            if rng.gen_bool(0.5) {
                map.paint_htunnel(p_center, n_center, p_center.y, 1);
                map.paint_vtunnel(p_center, n_center, n_center.x, 1);
            } else {
                map.paint_vtunnel(p_center, n_center, p_center.x, 1);
                map.paint_htunnel(p_center, n_center, n_center.y, 1);
            }
        }

        for room in map.rooms.iter().skip(1) {
            let center = room.center();
            let mut goblin = GoblinBundle::default();
            goblin.entity.position.xy = center;
            goblin.entity.position.idx = map.xy_idx(center);

            commands.spawn(goblin);
        }

        map.fill_blocking();

        map
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        if idx > self.tiles.len() {
            return true;
        }
        let tile = self.tiles[idx];

        is_tile_opaque(tile)
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let neighbours = self.idx_xy(idx).neighbors();

        let mut exits = SmallVec::new();

        for (n, cost) in &neighbours {
            if self.is_tile_traversable(*n) {
                exits.push((self.xy_idx(*n), *cost));
            };
        }

        exits
    }

    fn get_pathing_distance(&self, start: usize, end: usize) -> f32 {
        let s = self.idx_xy(start);
        let e = self.idx_xy(end);
        let p1 = bracket_pathfinding::prelude::Point::new(s.x, s.y);
        let p2 = bracket_pathfinding::prelude::Point::new(e.x, e.y);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}
