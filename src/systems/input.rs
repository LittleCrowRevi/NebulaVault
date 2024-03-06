use bevy::app::AppExit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::prelude::*;

pub fn input(
    mut exit: EventWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse: EventReader<MouseWheel>,
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

pub fn input_movement(
    mut query_p: Query<(&mut Position), With<PlayerMarker>>,
    query_map: Query<&Map>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let map = query_map.single();
    let mut p = IVec2::ZERO;
    if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::KeyZ) {
        p.x = -1;
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad2)
        || input.just_pressed(KeyCode::KeyX)
        || input.just_pressed(KeyCode::ArrowDown)
    {
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad3) || input.just_pressed(KeyCode::KeyC) {
        p.x = 1;
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad4)
        || input.just_pressed(KeyCode::KeyA)
        || input.just_pressed(KeyCode::ArrowLeft)
    {
        p.x = -1;
    }
    if input.just_pressed(KeyCode::Numpad6)
        || input.just_pressed(KeyCode::KeyD)
        || input.just_pressed(KeyCode::ArrowRight)
    {
        p.x = 1;
    }
    if input.just_pressed(KeyCode::Numpad7) || input.just_pressed(KeyCode::KeyQ) {
        p.x = -1;
        p.y = 1;
    }
    if input.just_pressed(KeyCode::Numpad8)
        || input.just_pressed(KeyCode::KeyW)
        || input.just_pressed(KeyCode::ArrowUp)
    {
        p.y = 1;
    }
    if input.just_pressed(KeyCode::Numpad9) || input.just_pressed(KeyCode::KeyE) {
        p.x = 1;
        p.y = 1;
    }
    for (mut player_pos) in &mut query_p {
        try_move_player(p.x, p.y, &mut player_pos, map);
    }
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
