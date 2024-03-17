use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{prelude::*, GameState};

#[derive(Component)]
pub struct DevText;

pub fn setup_debug(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section("", TextStyle { font_size: 15f32, color: Color::WHITE, ..default() })
            .with_text_justify(JustifyText::Left)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            }),
        DevText,
    ));
}

pub fn update_debug_text_system(
    mut debug_text: Query<&mut Text, With<DevText>>,
    query_map: Query<&Map>,
    diagnostics: Res<DiagnosticsStore>,
    state: Res<State<GameState>>,
) {
    let text = &mut debug_text.single_mut().sections[0].value;
    let map = &query_map.single().visible_tiles;
    *text = String::from("[Debug]\n");
    if let Some(value) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.smoothed()) {
        let v = value as i64;
        text.push_str(&format!("FPS: {v}\n"));
    }
    text.push_str(&format!("Visible Tiles: {}\n", map.len()));
    text.push_str(&format!("State: {:#?}\n", state.get()));
}
