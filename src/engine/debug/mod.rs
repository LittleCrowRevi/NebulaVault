use bevy::prelude::*;

use crate::prelude::*;

#[derive(Component)]
pub struct DevText;

pub fn setup_debug(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle { font_size: 15f32, color: Color::WHITE, ..default() },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        DevText,
    ));
}

pub fn update_debug_text(mut debug_text: Query<&mut Text, With<DevText>>, query_map: Query<&Map>) {
    let text = &mut debug_text.single_mut().sections[0].value;
    let map = &query_map.single().visible_tiles;
    *text = String::from("[Debug]\n");
    text.push_str(&format!("Visible Tiles: {}\n", map.iter().filter(|x| **x).count()));
}
