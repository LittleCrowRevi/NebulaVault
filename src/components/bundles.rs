use bevy::core::Name;
use bevy::math::ivec2;
use bevy::utils::HashSet;

use crate::prelude::*;

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
            position: Position(Point::new(0, 0)),
            marker: PlayerMarker,
            name: Name::new("Player"),
            viewshed: Viewshed { visible_tiles: HashSet::new(), range: 8, dirty: false },
        }
    }
}

#[derive(Bundle)]
pub struct GoblinBundle {
    //marker: PlayerMarker,
    pub position: Position,
    pub renderable: Renderable,
    pub name: Name,
    pub viewshed: Viewshed,
    pub enemy: Enemy,
    pub monster: Monster,
}

impl Default for GoblinBundle {
    fn default() -> Self {
        GoblinBundle {
            renderable: Renderable { glyph: 'g', bg_color: Color::BLACK, fg_color: Color::DARK_GREEN },
            position: Position(Point::new(0, 0)),
            name: Name::new("Goblin"),
            viewshed: Viewshed { visible_tiles: HashSet::new(), range: 8, dirty: false },
            enemy: Enemy,
            monster: Monster,
        }
    }
}
