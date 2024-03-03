use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy::reflect::Reflect;
use bevy::prelude::Component;
use bevy_ascii_terminal::GridPoint;
use rand::{Rng, thread_rng};
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::SeedableRng;

use crate::engine::{TileType, DEVMAP_SIZE};
use crate::systems::maps::map_builder::{EventGrowBSPTree, Leaf};

pub mod map_builder;

pub const TILE_SIZE: (i32, i32) = (32, 48);
pub const LEAF_DEV: EventGrowBSPTree = EventGrowBSPTree { seed: Leaf { x: 0, y: 0, w: 800, h: 500 }, min: (TILE_SIZE.0 * 2, TILE_SIZE.1 * 2), depth: 3 };

#[derive(Component, Debug, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Map(pub Vec<TileType>);

pub fn xy_idx(x: i32, y: i32) -> usize {
	((y * 80) + x) as usize
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
pub fn new_map_test() -> Vec<TileType> {
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
		if (x || y) && _i as usize != xy_idx(40,25) {
			map[_i as usize] = TileType::Wall;
		}
	}
	println!("{:?}", map);
	map
}
