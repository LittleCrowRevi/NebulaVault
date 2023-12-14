extern crate core_star;

use core_star::Component;
use godot::bind::{godot_api, GodotClass};
use godot::engine::Node;
use godot::obj::{Base, WithBaseField};
use godot::prelude::{INode, ToGodot};


pub mod entities;

pub trait Component {}

// stats
#[allow(dead_code)]
#[derive(GodotClass, Component)]
#[class(base=Node, init)]
pub struct VitalStats {
    #[base]
    base: Base<Node>,
    
    pub health: i32,
    pub mana: i32,
    pub energy: i32,
}

#[godot_api]
impl INode for VitalStats {
    fn ready(&mut self) {
        self.to_gd().set_name("C_VitalStats".into());
    }
    
    fn exit_tree(&mut self) {
        self.to_gd().free()
    }
}

#[allow(dead_code)]
#[derive(GodotClass, Component)]
#[class(base=Node, init)]
pub struct CoreStats {
    #[base]
    base: Base<Node>,
    
    pub strength: i32,
    pub intelligence: i32,
    pub agility: i32,
    pub constitution: i32,
    pub fortune: i32,
    pub wisdom: i32,
}

#[godot_api]
impl INode for CoreStats {
    fn ready(&mut self) {
        self.to_gd().set_name("C_CoreStats".into());
    }
    
    fn exit_tree(&mut self) {
        self.to_gd().free()
    }
}
