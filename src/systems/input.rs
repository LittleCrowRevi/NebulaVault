use bevy::app::AppExit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::MainCamera;
use crate::systems::maps::map_builder::EventGrowBSPTree;

pub fn input(
    mut exit: EventWriter<AppExit>,
    keys: Res<Input<KeyCode>>,
    mut e_redraw: EventWriter<EventGrowBSPTree>,
    mut mouse: EventReader<MouseWheel>,
    mut camera: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }

    /*if keys.just_pressed(KeyCode::Space) {
        e_redraw.send(LEAF_DEV);
    }
    mouse.read().for_each(|ev| {
        camera.for_each_mut(|mut c| {
            let mut log_scale = c.scale.ln();
            log_scale +=  if ev.y > 0. { -0.1 } else { 0.1 };
            c.scale = log_scale.exp();
        });
    });*/
}

