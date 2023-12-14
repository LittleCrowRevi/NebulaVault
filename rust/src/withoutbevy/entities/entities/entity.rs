use bevy_ecs::prelude::*;
use godot::builtin::{Vector2, Vector3};
use godot::engine::{AtlasTexture, ISprite2D, Sprite2D};
use godot::prelude::*;
use crate::WorldManager;

#[derive(GodotClass)]
#[class(init, base=Sprite2D)]
pub struct Entity {
    #[base]
    base: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Entity {

    fn exit_tree(&mut self) {

        self.to_gd().free()
    }
}

pub fn create_entity(wm: &WorldManager, sprite_coord: Rect2) -> Gd<Entity>
{
    let mut texture = AtlasTexture::new();
    texture.set_atlas(wm.tilemap.to_godot());
    texture.set_region(sprite_coord);

    let mut e = Entity::alloc_gd();
    e.set_texture(texture.upcast());

    e
}

#[derive(Bundle)]
pub struct EntityBase {
    pub vital: VitalStats,
    pub core: CoreStats,
    pub position: Position,
    pub sprite_coords: SpriteCoords
}

// Coordinates of the Sprite on the Tilemap
#[derive(Component)]
pub struct SpriteCoords(pub Rect2);

// In World position
#[derive(Component)]
pub struct Position(pub Vector3);

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