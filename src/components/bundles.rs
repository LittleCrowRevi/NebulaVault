use bevy::core::Name;
use bevy::math::ivec2;
use bevy::prelude::*;
use bevy::prelude::{Bundle, Color};

use crate::components::Renderable;
use crate::components::{PlayerMarker, Position, Viewshed};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: PlayerMarker,
    pub position: Position,
    pub renderable: Renderable,
    pub name: Name,
    pub viewshed: Viewshed,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            renderable: Renderable { glyph: '@', bg_color: Color::BLACK, fg_color: Color::WHITE },
            position: Position(ivec2(0, 0)),
            marker: PlayerMarker,
            name: Name::new("Player"),
            viewshed: Viewshed { visible_tiles: Vec::new(), range: 8 },
        }
    }
}

#[derive(Bundle)]
pub struct GoblinBundle {
    //marker: PlayerMarker,
    pub position: Position,
    pub renderable: Renderable,
    pub name: Name,
}

impl Default for GoblinBundle {
    fn default() -> Self {
        GoblinBundle {
            renderable: Renderable {
                glyph: 'g',
                bg_color: Color::BLACK,
                fg_color: Color::DARK_GREEN,
            },
            position: Position(ivec2(0, 0)),
            name: Name::new("Goblin"), //marker: PlayerMarker
        }
    }
}
