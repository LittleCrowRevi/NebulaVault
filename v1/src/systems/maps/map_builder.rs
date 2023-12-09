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
                        GRID_BOX.x - GRID_BOX.x / GRID_BOX.x,
                        GRID_BOX.y - GRID_BOX.y / GRID_BOX.y,
                    )),
                    ..default()
                },
                ..default()
            }); 
        });
}

pub struct Tree {
    leaf: Leaf,
    lchild: Option<Box<Tree>>,
    rchild: Option<Box<Tree>>
}

impl Tree {
    pub fn get_leafs(self) -> Vec<Leaf> {
        return if self.lchild.is_none() && self.rchild.is_none() {
            vec![self.leaf]
        } else {
            let mut v = vec![];
            //v.push(self.leaf);
            v.append(&mut self.lchild.unwrap().get_leafs());
            v.append(&mut self.rchild.unwrap().get_leafs());
            v
        }
    }
}

pub fn split_leaf(container: Leaf, depth: i16, min: i32) -> Tree {
    let mut tree = Tree {
        leaf: container,
        lchild: None,
        rchild: None
    };

    if depth > 0 && tree.leaf.w > min as f32 && tree.leaf.h > min as f32 {
        let (left, right) = random_split(&tree.leaf, min);
        println!("Splitted Leafs with depth: {} and sizes: lx {} ly {} lw {} lh {}, rx {} ry {} rw {} rh {}", depth, left.x, left.y, left.w, left.h, right.x, right.y, right.w, right.h);
        tree.lchild = Some(Box::from(split_leaf(left, depth - 1, min)));
        tree.rchild = Some(Box::from(split_leaf(right, depth - 1, min)));
    }

    return tree
}

pub fn random_split(container: &Leaf, min: i32) -> (Leaf, Leaf) {
    let left: Leaf;
    let right: Leaf;

    if random() {
        let lx = thread_rng().gen_range(min..container.w as i32) as f32;
        left = Leaf {
            x: container.x - container.w / 2f32 + lx / 2f32,
            y: container.y,
            w: lx,
            h: container.h
        };
        right = Leaf {
            x: container.x + container.w / 2f32 - ((container.w - left.w) / 2f32), 
            y: container.y,
            w: container.w - left.w,
            h: container.h
        };
    } else {
        let ly = thread_rng().gen_range(min..container.h as i32) as f32;
        left = Leaf {
            x: container.x,
            y: container.y - container.h / 2f32 + ly / 2f32,
            w: container.w,
            h: ly,
        };
        right = Leaf {
            x: container.x,
            y: container.y + container.h / 2f32 - ((container.h - left.h) / 2f32),
            w: container.w,
            h: container.h - left.h
        };
    }

    return (left, right)
}

#[derive(Component)]
pub struct Leaf {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Leaf {
    pub fn paint(self, command: &mut Commands) {
        command.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.1, 0.10),
                    custom_size: Some(vec2(self.x, self.y)),
                    ..default()
                },
                ..default()
            },
            self
        ));
    }

    fn spawn_box(self, commands: &mut Commands) {
        let (x, y, w, h) = (self.x, self.y, self.w, self.h);
        commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(vec3(x, y, 1.0)),
                    visibility: Visibility::Visible,
                    ..default()
                },
                self
            ))
            .with_children(|parent| {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.2, 0.2, 0.2),
                        custom_size: Some(vec2(w, h)),
                        ..default()
                    },
                    ..default()
                });
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.15, 0.15, 0.15),
                        custom_size: Some(vec2(
                            w - 10.,
                            h - 10.,
                        )),
                        ..default()
                    },
                    ..default()
                }); 
            });
    }
}

pub fn generate_bsp(commands: &mut Commands) {
    let tree = split_leaf(
        Leaf { 
            x: 0f32, 
            y: 0f32,
            h: 1000f32,
            w: 1000f32,
        },
        5, 
        100);
    
    let leafs = tree.get_leafs();
    println!("Leafs: {}", leafs.iter().count());
    for leaf in leafs {
        leaf.spawn_box(commands);
    }
}