mod map_builder;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use godot::prelude::*;

struct NebulaVault;

#[gdextension]
unsafe impl ExtensionLibrary for NebulaVault {}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct WorldManager {
    world: World,
    schedule: Schedule,
    
    #[base]
    node: Base<Node2D>,
    
}

#[godot_api]
impl INode2D for WorldManager {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("Hello, world!");
        
        let mut world = World::new();

        let mut schedule = Schedule::new(MainLoop);
        schedule.add_systems(test_system);
        
        Self {
            node: base,
            world,
            schedule
        }
    }

    fn process(&mut self, delta: f64) {
        self.schedule.run(&mut self.world)
    }
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MainLoop;

pub fn test_system() {
    godot_print!("hallo from bevy!");
}
