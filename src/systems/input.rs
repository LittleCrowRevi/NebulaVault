use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use crate::systems::maps::map_builder::{EventGrowBSPTree};
use crate::systems::maps::LEAF_DEV;
use crate::MainCamera;

pub fn handle_movement(
    mov_request: Query<()>
) {
    
}

pub fn input(
    keys: Res<Input<KeyCode>>,
    mut e_redraw: EventWriter<EventGrowBSPTree>,
    mut mouse: EventReader<MouseWheel>,
    mut camera: Query<&mut OrthographicProjection, With<MainCamera>>
) {
    if keys.just_pressed(KeyCode::Space) {
        e_redraw.send(LEAF_DEV);
    }
    mouse.read().for_each(|ev| {
        camera.for_each_mut(|mut c| {
            let mut log_scale = c.scale.ln();
            log_scale +=  if ev.y > 0. { -0.1 } else { 0.1 };
            c.scale = log_scale.exp();
        });
    });
}