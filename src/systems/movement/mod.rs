use crate::{prelude::*, PlayerPosition};

#[derive(Component)]
pub struct LeftWalker;

pub fn left_walk_system(
    mut left_walker: Query<&mut Position, With<LeftWalker>>,
    mut nebula_time: ResMut<NebulaTime>,
    time: Res<Time>,
) {
    if !nebula_time.0.tick(time.delta()).just_finished() {
        return;
    };
    for mut pos in &mut left_walker {
        pos.0.x -= 1;
        if pos.0.x < 0 {
            pos.0.x = VIEWPORT_SIZE[0] as i32 - 3
        }
    }
}

pub fn try_move_player(
    delta_x: i32,
    delta_y: i32,
    player_pos: &mut Position,
    viewshed: &mut Viewshed,
    map: &Map,
    pos_res: &mut ResMut<PlayerPosition>,
) {
    let destination = map.xy_idx(Point::new(player_pos.0.x + delta_x, player_pos.0.y + delta_y));
    if map.tiles[destination] != TileType::Wall {
        player_pos.0.x = min(map.width, max(0, player_pos.0.x + delta_x));
        player_pos.0.y = min(map.height, max(0, player_pos.0.y + delta_y));

        // TODO: Make this into its on function which updates whenever
        // the player position updates, to account for teleports and such
        pos_res.idx = map.xy_idx(player_pos.0);
        viewshed.dirty = false;
    }
}
