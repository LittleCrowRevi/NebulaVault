use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_ascii_terminal::Tile;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;

use crate::prelude::*;

pub mod bundles;
pub mod races;

pub use bundles::*;
pub use races::*;

// Engine Components

#[derive(Component)]
pub struct Position(pub Point);

#[derive(Component, Debug)]
pub struct Renderable {
    pub fg_color: Color,
    pub bg_color: Color,
    pub glyph: char,
}

impl From<&Renderable> for Tile {
    fn from(value: &Renderable) -> Self {
        Tile { glyph: value.glyph, bg_color: value.bg_color, fg_color: value.fg_color }
    }
}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: HashSet<usize>,
    pub range: i32,
    pub dirty: bool,
}

// Markers

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Monster;

// stats

#[allow(dead_code)]
#[derive(Component)]
pub struct VitalStats {
    pub health: i32,
    pub mana: i32,
    pub energy: i32,
}

impl Default for VitalStats {
    fn default() -> Self {
        Self { health: 100, energy: 100, mana: 100 }
    }
}

#[allow(dead_code)]
#[derive(Component)]
pub struct CoreStats {
    pub strength: i32,
    pub intelligence: i32,
    pub agility: i32,
    pub constitution: i32,
    pub fortune: i32,
    pub wisdom: i32,
}

impl Default for CoreStats {
    fn default() -> Self {
        Self { constitution: 10, fortune: 10, agility: 10, strength: 10, wisdom: 10, intelligence: 10 }
    }
}
