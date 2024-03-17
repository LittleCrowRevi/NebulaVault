use crate::{prelude::*, PlayerPosition};

#[derive(Event)]
pub struct MovementRequestEvent {
    original_pos: Point,
    target: Point,
    entity: Entity,
}

impl MovementRequestEvent {
    pub fn new(original_pos: Point, target: Point, entity: Entity) -> Self {
        Self { original_pos, target, entity }
    }
}

pub fn movement_requests_handler(
    commands: Commands,
    mut ev_move: EventReader<MovementRequestEvent>,
    mut query_e: Query<(Entity, &mut Position, &mut Viewshed, Option<&BlocksTile>)>,
    mut query_map: Query<&mut Map>,
) {
    let mut map = query_map.single_mut();
    for ev in ev_move.read() {
        for (e, mut pos, mut vs, blocks) in &mut query_e {
            if e != ev.entity {
                continue;
            }

            if !map.is_tile_traversable(ev.target) {
                continue;
            }

            pos.xy.x = ev.target.x;
            pos.xy.y = ev.target.y;
            pos.idx = map.xy_idx(pos.xy);

            if blocks.is_some() {
                map.walkable[pos.idx] = false;
            }
            //map.tile_content[pos.idx].push((e, *e_type));
            vs.dirty = false;
        }
    }
}
