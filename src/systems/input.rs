use bevy::app::AppExit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use super::map_gen::map::Map;
use crate::components::{PlayerMarker, Position};

pub fn input(mut exit: EventWriter<AppExit>, keys: Res<ButtonInput<KeyCode>>, mut mouse: EventReader<MouseWheel>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
    /* mouse.read().for_each(|ev| {
        camera.for_each_mut(|mut c| {
            let mut log_scale = c.scale.ln();
            log_scale +=  if ev.y > 0. { -0.1 } else { 0.1 };
            c.scale = log_scale.exp();
        });
    }); */
}

pub fn respawn_map(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut query_q: Query<&mut Position, With<PlayerMarker>>,
    maps: Query<(&Map, Entity)>,
) {
    if input.just_pressed(KeyCode::Space) {
        commands.entity(maps.single().1).despawn_recursive();

        let map = Map::new_map_rooms_coors((80, 50));
        let player_spawn = map.rooms[0].center();
        commands.spawn(map);
        let mut player = query_q.single_mut();
        player.0.x = player_spawn.x;
        player.0.y = player_spawn.y;
    }
}
