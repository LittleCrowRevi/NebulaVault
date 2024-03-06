use bevy_ascii_terminal::GridPoint;
use rand::{thread_rng, Rng};

use crate::engine::{TileType, DEVMAP_SIZE};

pub mod map;

pub fn xy_idx_test(x: i32, y: i32) -> usize {
    ((y * DEVMAP_SIZE.x()) + x) as usize
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
pub fn map_test() -> Vec<TileType> {
    let tile_count = (DEVMAP_SIZE.x() * DEVMAP_SIZE.y());
    let mut map = vec![TileType::Floor; tile_count as usize];

    for x in 0..DEVMAP_SIZE.x() {
        map[xy_idx_test(x, 0)] = TileType::Wall;
        map[xy_idx_test(x, DEVMAP_SIZE.y() - 1)] = TileType::Wall;
    }
    for y in 0..DEVMAP_SIZE.y() {
        map[xy_idx_test(0, y)] = TileType::Wall;
        map[xy_idx_test(DEVMAP_SIZE.x() - 1, y)] = TileType::Wall;
    }

    let mut rng = thread_rng();
    for _i in 0..tile_count {
        let x = rng.gen_ratio(1, (DEVMAP_SIZE.x() - 1) as u32);
        let y = rng.gen_ratio(1, (DEVMAP_SIZE.y() - 1) as u32);
        if (x || y) && _i as usize != xy_idx_test(40, 25) {
            map[_i as usize] = TileType::Wall;
        }
    }
    map
}
