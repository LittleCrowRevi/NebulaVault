use bevy::prelude::*;

pub fn movement_req_listener(
    mut events: EventReader<MovementReqEvent>
) {
    
}

#[derive(Event)]
pub struct MovementReqEvent {
    pub entity: Entity,
    pub old_position: Vec3,
    pub new_position: Vec3
}