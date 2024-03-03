use std::cmp::{min, max};
use bevy::ecs::system::Command;
use bevy::prelude::*;
use crate::components::{Position};
use crate::{NebulaTime, VIEWPORT_SIZE};
use crate::components::PlayerMarker;


#[derive(Component)]
pub struct LeftWalker;

pub fn left_walk_system(mut left_walker: Query<&mut Position, With<LeftWalker>>, mut nebula_time: ResMut<NebulaTime>, time: Res<Time>) {
	if !nebula_time.0.tick(time.delta()).just_finished() { return };
	for (mut pos) in &mut left_walker {
		pos.0.x -= 1;
		if pos.0.x < 0 { pos.0.x = VIEWPORT_SIZE[0] as i32 - 3 }
	}
}

fn try_move_player(delta_x: i32, delta_y: i32, player_pos: &mut Position) {
	player_pos.0.x = min(79, max(0, player_pos.0.x + delta_x));
	player_pos.0.y = min(79, max(0, player_pos.0.y + delta_y));
}

pub fn player_input(mut command: Commands, mut query_p: Query<(&mut Position), With<PlayerMarker>>, input: Res<Input<KeyCode>>,) {
	let mut p = IVec2::ZERO;
	if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::Z) {
		p.x = -1;
		p.y = -1;
	}
	if input.just_pressed(KeyCode::Numpad2) || input.just_pressed(KeyCode::X) || input.just_pressed(KeyCode::Down) {
		p.y = -1;
	}
	if input.just_pressed(KeyCode::Numpad3) || input.just_pressed(KeyCode::C) {
		p.x = 1;
		p.y = -1;
	}
	if input.just_pressed(KeyCode::Numpad4) || input.just_pressed(KeyCode::A) || input.just_pressed(KeyCode::Left) {
		p.x = -1;
	}
	if input.just_pressed(KeyCode::Numpad6) || input.just_pressed(KeyCode::D) || input.just_pressed(KeyCode::Right) {
		p.x = 1;
	}
	if input.just_pressed(KeyCode::Numpad7) || input.just_pressed(KeyCode::Q) {
		p.x = -1;
		p.y = 1;
	}
	if input.just_pressed(KeyCode::Numpad8) || input.just_pressed(KeyCode::W) || input.just_pressed(KeyCode::Up) {
		p.y = 1;
	}
	if input.just_pressed(KeyCode::Numpad9) || input.just_pressed(KeyCode::E) {
		p.x = 1;
		p.y = 1;
	}
	for (mut player_pos) in &mut query_p {
		try_move_player(p.x, p.y, &mut player_pos);
	}
}