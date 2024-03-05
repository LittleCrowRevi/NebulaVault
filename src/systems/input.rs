use bevy::app::AppExit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use super::maps::{new_map_rooms_coors, Map};
use crate::components::{PlayerMarker, Position};
use crate::systems::maps::map_builder::EventGrowBSPTree;
use crate::MainCamera;

pub fn input(
    mut exit: EventWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
    mut e_redraw: EventWriter<EventGrowBSPTree>,
    mut mouse: EventReader<MouseWheel>,
    mut camera: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
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

        let (map, rooms) = new_map_rooms_coors();
        let (player_x, player_y) = rooms[0].center();
        commands.spawn(Map(map, rooms));
        let mut player = query_q.single_mut();
        player.0.x = player_x;
        player.0.y = player_y;
    }
}
