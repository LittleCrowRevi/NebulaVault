use std::cmp::{max, min};

#[cfg(test)]
mod tests {
    use bevy::utils::HashSet;

    use crate::prelude::*;

    #[test]
    fn test_is_tile_traversable() {
        // Create a mock grid with a width of 3, height of 3, and walkable tiles as indicate

        let mut grid = Map {
            rooms: Vec::new(),
            tiles: vec![
                TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall,
                TileType::Wall, TileType::Floor, TileType::Wall, TileType::Floor, TileType::Wall,
                TileType::Wall, TileType::Wall, TileType::Floor, TileType::Wall, TileType::Wall,
                TileType::Wall, TileType::Floor, TileType::Wall, TileType::Floor, TileType::Wall,
                TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall,
            ],
            width: 5,
            height: 5,
            revealed_tiles: HashSet::new(),
            visible_tiles: HashSet::new(),
            tile_entities: vec![Vec::new(); 5 * 5],
            walkable: vec![false; 5 * 5],
        };
        grid.fill_blocking();

        // Test a traversable position
        let traversable_pos = Point { x: 1, y: 1 };
        assert!(grid.is_tile_traversable(traversable_pos));

        // Test a traversable position
        let traversable_pos = Point { x:3, y:3};
        assert!(grid.is_tile_traversable(traversable_pos));

        // Test an impassable position
        let impassable_pos = Point { x: 2, y: 3 };
        assert!(!grid.is_tile_traversable(impassable_pos));
    }

    #[test]
    fn test_pathfinding() {
        let mut grid = Map {
            rooms: Vec::new(),
            tiles: vec![
                TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall,
                TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall,
                TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall,
                TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall,
                TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall,
            ],
            width: 5,
            height: 5,
            revealed_tiles: HashSet::new(),
            visible_tiles: HashSet::new(),
            tile_entities: vec![Vec::new(); 5 * 5],
            walkable: vec![false; 5 * 5],
        };
        grid.fill_blocking();

        let start = Point::new(1, 1);
        let s_index = grid.xy_idx(start);
        let goal = Point::new(3, 3);
        let g_index = grid.xy_idx(goal);

        let path = bracket_pathfinding::prelude::a_star_search(s_index, g_index, &grid);

        assert!(path.success);
    }
}

/*
#[test]
fn test_xy_idx() {
    let map = Map {
        rooms: vec![],
        tiles: vec![TileType::Floor; 100],
        width: 10,
        height: 10,
        revealed_tiles: Vec::new(),
        visible_tiles: Vec::new(),
    };
    assert_eq!(map.xy_idx(2, 3), 32);
}

#[test]
fn test_paint_room_floor() {
    let mut map = Map {
        rooms: vec![],
        tiles: vec![TileType::Wall; 100],
        width: 10,
        height: 10,
        revealed_tiles: Vec::new(),
        visible_tiles: Vec::new(),
    };
    let room = Rect::new(1, 1, 3, 3);
    map.paint_room_floor(&room);
    for y in 1..=3 {
        for x in 1..=3 {
            assert_eq!(map.tiles[map.xy_idx(x, y)], TileType::Floor);
        }
    }
}

#[test]
fn test_paint_htunnel() {
    let mut map = Map {
        rooms: vec![],
        tiles: vec![TileType::Wall; 100],
        width: 10,
        height: 10,
        revealed_tiles: Vec::new(),
        visible_tiles: Vec::new(),
    };
    let room1 = Rect::new(1, 1, 3, 3);
    let room2 = Rect::new(8, 1, 3, 3);
    map.paint_htunnel(room1.center(), room2.center(), 2, 3);
    for y in 2..=4 {
        assert_eq!(map.tiles[map.xy_idx(5, y)], TileType::Floor);
        assert_eq!(map.tiles[map.xy_idx(6, y)], TileType::Floor);
        assert_eq!(map.tiles[map.xy_idx(7, y)], TileType::Floor);
    }
}

#[test]
fn test_paint_vtunnel() {
    let mut map = Map {
        rooms: vec![],
        tiles: vec![TileType::Wall; 100],
        width: 10,
        height: 10,
        revealed_tiles: Vec::new(),
        visible_tiles: Vec::new(),
    };
    let room1 = Rect::new(1, 1, 3, 3);
    let room2 = Rect::new(1, 8, 3, 3);
    {
        let this = &mut map;
        let point1 = &room1.center();
        let point2 = &room2.center();
        let (start_y, end_y) = (min(point1.y, point2.y), max(point1.y, point2.y));
        for y in start_y..=end_y {
            for i in 0..3 {
                let idx = this.xy_idx(2 + i, y);
                if idx > 0 && idx < this.size() {
                    this.tiles[idx] = TileType::Floor;
                }
            }
        }
    };
    for x in 2..=4 {
        assert_eq!(map.tiles[map.xy_idx(x, 5)], TileType::Floor);
        assert_eq!(map.tiles[map.xy_idx(x, 6)], TileType::Floor);
        assert_eq!(map.tiles[map.xy_idx(x, 7)], TileType::Floor);
    }
}
*/
