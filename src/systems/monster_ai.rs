#![allow(clippy::module_name_repetitions)]

use bracket_pathfinding::prelude::{DistanceAlg, Point as BracketPoint};
use pathfinding::prelude::*;

use crate::{prelude::*, PlayerPosition};

pub fn monster_ai_system(
    commands: Commands,
    query_monster: Query<(&Viewshed, &Position, &Name, Entity), With<Monster>>,
    player_resource: Res<PlayerPosition>,
    query_map: Query<&Map>,
    mut ev_mov: EventWriter<MovementRequestEvent>,
) {
    let map = query_map.single();

    for (v, pos, name, entity) in &query_monster {
        // TODO: Maybe store the path as a component? And only update it once a certain distance has been reached
        // or the player leaves a certain range? path.dirty = true
        if !v.visible_tiles.contains(&player_resource.idx) && !v.previous_tiles.contains(&player_resource.idx) {
            continue;
        }

        let player_idx = player_resource.idx;
        let path = bracket_pathfinding::prelude::a_star_search(pos.idx, player_idx, map);

        let player_pos = map.idx_xy(player_idx);
        if DistanceAlg::Pythagoras.distance2d(BracketPoint::from(pos.xy), BracketPoint::from(player_pos)) < 1.5 {
            println!("{name} is seeing you!");
            continue;
        }

        if path.success && path.steps.len() > 1 {
            println!("{name} is following you!");

            let next_sindex = path.steps[1];
            let next_step = map.idx_xy(next_sindex);

            if !map.is_tile_traversable(next_step) {
                println!("t: {}", map.walkable[next_sindex]);
                continue;
            }

            ev_mov.send(MovementRequestEvent::new(pos.xy, next_step, entity));
        }
    }
}
