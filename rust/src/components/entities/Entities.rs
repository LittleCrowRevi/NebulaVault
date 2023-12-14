use godot::prelude::*;
use crate::components::entities::*;
use crate::components::{CoreStats, VitalStats};

pub struct Human;

impl Human {
    pub fn default() -> Gd<Entity> {
        let mut e = Entity::alloc_gd();
        e.add_child(VitalStats::alloc_gd().upcast());
        e.add_child(CoreStats::alloc_gd().upcast());
        
        e.set_name("Human".into());
        
        e
    }
}