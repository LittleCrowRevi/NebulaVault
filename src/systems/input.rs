use bevy::app::AppExit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::{prelude::*, GameState, PlayerPosition};

pub fn input_game_system(
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

pub fn input_movement_system(
    query_p: Query<(&Position, &Viewshed, Entity), With<PlayerMarker>>,
    query_map: Query<&Map>,
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
    mut ev_move: EventWriter<MovementRequestEvent>,
) {
    let map = query_map.single();
    let mut p = Point::new(0, 0);
    if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::KeyZ) {
        p.x = -1;
        p.y = -1;
    }
    if input.pressed(KeyCode::Numpad2) || input.just_pressed(KeyCode::KeyX) || input.just_pressed(KeyCode::ArrowDown) {
        p.y = -1;
    }
    if input.pressed(KeyCode::Numpad3) || input.just_pressed(KeyCode::KeyC) {
        p.x = 1;
        p.y = -1;
    }
    if input.pressed(KeyCode::Numpad4) || input.just_pressed(KeyCode::KeyA) || input.just_pressed(KeyCode::ArrowLeft) {
        p.x = -1;
    }
    if input.pressed(KeyCode::Numpad6) || input.just_pressed(KeyCode::KeyD) || input.just_pressed(KeyCode::ArrowRight) {
        p.x = 1;
    }
    if input.pressed(KeyCode::Numpad7) || input.just_pressed(KeyCode::KeyQ) {
        p.x = -1;
        p.y = 1;
    }
    if input.pressed(KeyCode::Numpad8) || input.just_pressed(KeyCode::KeyW) || input.just_pressed(KeyCode::ArrowUp) {
        p.y = 1;
    }
    if input.pressed(KeyCode::Numpad9) || input.just_pressed(KeyCode::KeyE) {
        p.x = 1;
        p.y = 1;
    }

    for (player_pos, viewshed, entity) in &query_p {
        if p.x != 0 || p.y != 0 {
            let target = player_pos.xy + p;
            ev_move.send(MovementRequestEvent::new(player_pos.xy, target, entity));
            state.set(GameState::Ticking);
        }
    }
}

pub fn dev_input_respawn_map(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut query_q: Query<(&mut Position, Option<&PlayerMarker>, &Renderable, Entity)>,
    maps: Query<(&Map, Entity)>,
) {
    if input.just_pressed(KeyCode::Space) {
        commands.entity(maps.single().1).despawn_recursive();

        let map = Map::new_map_rooms_coors(&mut commands, (80, 50));
        let player_spawn = map.rooms[0].center();

        for (mut pos, player, _, entity) in &mut query_q {
            if player.is_some() {
                pos.xy.x = player_spawn.x;
                pos.xy.y = player_spawn.y;
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }

        commands.spawn(map);
    }
}
