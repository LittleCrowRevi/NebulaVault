use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use godot::engine::{AtlasTexture, CompressedTexture2D, InputEvent, Sprite2D, Texture, Texture2D};
use godot::prelude::*;

use crate::entities::{ANGEL_SPRITE, PLAYER_SPRITE};
use crate::entities::entity::{create_entity, Entity};

struct NebulaVault;

#[gdextension]
unsafe impl ExtensionLibrary for NebulaVault {}

#[derive(GodotClass)]
#[class(base=Node2D, init)]
pub struct WorldManager {
    pub schedule: Schedule,
    pub world: World,
    pub tilemap: Gd<Texture2D>,
    pub state: GameState,
    
    #[base]
    pub node: Base<Node2D>,
    
}

#[godot_api]
impl INode2D for WorldManager {

    fn ready(&mut self) {
        godot_print!("Hello, world!");

        let mut world = World::new();
        
        self.world = world;

        let mut schedule = Schedule::new(MainLoop);
        schedule.add_systems(test_system);
        self.schedule = schedule;
        
        let mut tilemap = load::<Texture2D>("res://resources/tilemap/curses-vector.png");
        self.tilemap = tilemap;

    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if self.state == GameState::StartUp {
            if event.is_action_pressed("space".into()) {
                self.create_world();
            }
        }
    }
}

impl WorldManager {
    
    pub fn create_world(&mut self) {
        let mut pc = create_entity(&self, PLAYER_SPRITE);
        pc.set_name("PC".to_godot());
        self.node.add_child(pc.upcast());

        let mut angel = create_entity(&self, ANGEL_SPRITE);
        angel.set_name("angel".to_godot());
        angel.set_position(Vector2 { x:40.0, y:40.0});
        self.node.add_child(angel.upcast());
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    InGame,
    Pause,
    #[default]
    StartUp
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MainLoop;

pub fn test_system() {
    godot_print!("hallo from bevy!");
}
