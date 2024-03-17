use bevy::core::Name;
use bevy::utils::HashSet;

use crate::prelude::*;

#[derive(Bundle)]
pub struct EntityBundle {
    pub renderable: Renderable,
    pub position: Position,
    pub name: Name,
    pub entity_type: EntityType,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: PlayerMarker,
    pub entity: EntityBundle,
    pub viewshed: Viewshed,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            entity: EntityBundle {
                renderable: Renderable { glyph: '@', bg_color: Color::BLACK, fg_color: Color::WHITE },
                position: Position { xy: Point::new(0, 0), idx: 0 },
                name: Name::new("Player"),
                entity_type: EntityType::Player,
            },
            marker: PlayerMarker,
            viewshed: Viewshed { visible_tiles: HashSet::new(), range: 8, dirty: false, previous_tiles: HashSet::new() },
        }
    }
}

#[derive(Bundle)]
pub struct GoblinBundle {
    //marker: PlayerMarker,
    pub entity: EntityBundle,
    pub viewshed: Viewshed,
    pub enemy: Enemy,
    pub monster: Monster,
    pub blocks: BlocksTile,
}

impl Default for GoblinBundle {
    fn default() -> Self {
        GoblinBundle {
            entity: EntityBundle {
                renderable: Renderable { glyph: 'g', bg_color: Color::BLACK, fg_color: Color::DARK_GREEN },
                position: Position { xy: Point::new(0, 0), idx: 0 },
                name: Name::new("Goblin"),
                entity_type: EntityType::Enemy,
            },
            viewshed: Viewshed { visible_tiles: HashSet::new(), range: 8, dirty: false, previous_tiles: HashSet::new() },
            enemy: Enemy,
            monster: Monster,
            blocks: BlocksTile,
        }
    }
}
