use std::process::Command;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::prelude::*;
use crate::GRID_BOX;

#[derive(Resource)]
pub struct Maps {
    maps: HashMap<String, GridMap>
}

pub struct GridMap {
    size: Option<Vec2>,
    tiles: HashMap<Vec3, Tile>,
    rooms: Vec<Room>,
    name: String,
    
}

impl GridMap {
    fn move_to_unoccupied() {}
}

pub struct Room {
    dim: Rect
}

impl Room {
    pub fn default_room() -> Self {
        Self {
            dim: Rect::from_corners(vec2(0., 0.), vec2(10., 10.))
        }
    }
    pub fn rectange_room() -> Self {
        Self {
            dim: Rect::from_corners(vec2(0., 0.), vec2(10., 20.))
        }
    }
    
    pub fn thiny_room() -> Self {
        Self {
            dim: Rect::from_corners(vec2(0., 0.), vec2(5.0, 10.0))
        }
    }
    
    pub fn thinx_room() -> Self {
        Self {
            dim: Rect::from_corners(vec2(0., 0.), vec2(10., 5.))
        }
    }
}

pub struct Tile {
    tile_type: TileType,
    entity: Option<Entity>
}

pub enum TileType {
    Wall,
    Player,
    Enemy,
    NPC,
    Item,
    Object,
    Void
}

pub struct Tree {
    leaf: Container,
    lchild: Option<Box<Tree>>,
    rchild: Option<Box<Tree>>
}

impl Tree {
    pub fn get_leafs(self) -> Vec<Container> {
        return if self.lchild.is_none() && self.rchild.is_none() {
            vec![self.leaf]
        } else {
            let mut v = vec![];
            v.append(&mut self.lchild.unwrap().get_leafs());
            v.append(&mut self.rchild.unwrap().get_leafs());
            v
        }
    }
}

pub fn split_container(container: Container, depth: u32) -> Tree {
    let mut tree = Tree {
        leaf: container,
        lchild: None,
        rchild: None
    };
    
    if depth != 0 {
        let (left, right) = random_split(&tree.leaf, thread_rng());
        tree.lchild = Some(Box::from(split_container(left, depth)));
        tree.rchild = Some(Box::from(split_container(right, depth)));
    }
    
    return tree
}

pub fn random_split(container: &Container, mut rng: ThreadRng) -> (Container, Container) {
    let left: Container; 
    let right: Container;
    
    if rng.gen() {
        left = Container {
            x: rng.gen_range(1..container.x),
            y: container.y
        };
        right = Container {
            x: container.x - left.x,
            y: container.y
        };
    } else {
        left = Container {
            x: container.x,
            y: rng.gen_range(1..container.y),
        };
        right = Container {
            x: container.x,
            y: container.y - left.y,
        };
    }
    
    return (left, right)
}

pub struct Container {
    pub x: i32,
    pub y: i32,
}

impl Container {
    pub fn paint(self, command: Command) {

    }
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



fn create_square_grid(
    commands: &mut Commands,
    grid_box: Vec2,
    boundary_box: Vec2,
    grid_name: String,
) -> Entity {
    let grid_x = grid_box.x as usize;
    let grid_y = grid_box.y as usize;

    let mut grid = commands.spawn((Grid(grid_name), SpatialBundle { ..default() }));

    grid.with_children(|parent| {
        for x in ((boundary_box.x - grid_box.x) as i32 / -2..=boundary_box.x as i32 / 2).step_by(grid_x) {
            for y in ((boundary_box.y - grid_box.y) as i32 / -2..=boundary_box.y as i32 / 2).step_by(grid_y) {
                spawn_box(parent, (x) as f32, (y) as f32);
            }
        }
    });

    grid.id()
}

fn spawn_box(commands: &mut ChildBuilder, x: f32, y: f32) {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(vec3(x, y, 1.0)),
                visibility: Visibility::Visible,
                ..default()
            },
            GridBox(vec3((x / GRID_BOX.x).round(), (y / GRID_BOX.y).round(), 1.0)),
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.2, 0.2),
                    custom_size: Some(vec2(GRID_BOX.x, GRID_BOX.y)),
                    ..default()
                },
                ..default()
            });
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.15, 0.15, 0.15),
                    custom_size: Some(vec2(
                        GRID_BOX.x - GRID_BOX.x / GRID_BOX.y,
                        GRID_BOX.y - GRID_BOX.y / GRID_BOX.y,
                    )),
                    ..default()
                },
                ..default()
            });
        });
}