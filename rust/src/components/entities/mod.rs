use godot::engine::{ISprite2D, Sprite2D};
use godot::prelude::*;

use crate::components::Component;

pub mod entities;

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

// Coordinates of the Sprite on the Tilemap
pub struct SpriteCoords(pub Rect2);

// In World position
pub struct Position(pub Vector3);