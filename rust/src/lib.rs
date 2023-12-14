use bevy_ecs::component::Component;
use godot::prelude::*;
use crate::components::entities::Entities::Human;

extern crate core_star;

mod map_builder;
mod components;

struct NebulaVault;

#[gdextension]
unsafe impl ExtensionLibrary for NebulaVault {}

#[derive(GodotClass)]
#[class(base=Node2D, init)]
struct Galaxy {
    #[base]
    base: Base<Node2D>
}

#[godot_api]
impl INode2D for Galaxy {
    fn ready(&mut self) {
        let human = Human::default();
        self.base.add_child(human.upcast());
    }
}

