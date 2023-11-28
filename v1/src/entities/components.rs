use bevy::prelude::*;

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
        Self {
            health: 100,
            energy: 100,
            mana: 100
        }
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
        Self {
            constitution: 10,
            fortune: 10,
            agility: 10,
            strength: 10,
            wisdom: 10,
            intelligence: 10
        }
    }
}

// dev

#[derive(Component)]
pub struct Counter {
    number: i64,
}

#[derive(Component)]
struct Movement(f32, bool);

#[derive(Component)]
struct DevText {
    mov_num: f32,
}

// Tags

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct HUD;

// Bundles

// Movement
#[derive(Component)]
struct MovRequest {
    entity: Entity,
    new_pos: Vec3
}

// Grid
#[derive(Component)]
struct ActiveGrid;

#[derive(Component)]
struct GridBox(Vec3);

#[derive(Component)]
struct Grid(String);

#[derive(Component)]
struct GridPositions(Vec<Vec3>);
