use crate::{prelude::*, PlayerPosition};

pub fn monster_ai_system(
    commands: Commands,
    mut query_monster: Query<(&Viewshed, &mut Position, &Name), With<Monster>>,
    player_resource: Res<PlayerPosition>,
) {
    for (v, pos, name) in &mut query_monster {
        if v.visible_tiles.contains(&player_resource.idx) {
            println!("[{name}] See you!");
        }
    }
}
