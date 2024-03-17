pub mod fov;
pub mod input;
pub mod map_gen;
pub mod monster_ai;
pub mod movement;

pub use fov::*;
pub use input::*;
pub use map_gen::*;
pub use monster_ai::*;
pub use movement::*;

use crate::{prelude::*, PlayerPosition};

pub fn update_player_resource_system(
    query_player: Query<&Position, (With<PlayerMarker>, Changed<Position>)>,
    mut p_resource: ResMut<PlayerPosition>,
) {
    for position in &query_player {
        p_resource.idx = position.idx;
    }
}



