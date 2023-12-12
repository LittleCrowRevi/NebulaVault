use std::process::Command;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::prelude::*;
use crate::GRID_BOX;

#[derive(Resource)]
pub struct Maps {
    maps: HashMap<String, GridMap>,
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
    dim: Rect,
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
    entity: Option<Entity>,
}

pub enum TileType {
    Wall,
    Player,
    Enemy,
    NPC,
    Item,
    Object,
    Void,
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

pub struct Tree {
    leaf: Leaf,
    lchild: Option<Box<Tree>>,
    rchild: Option<Box<Tree>>,
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
        };
    }
}

pub fn split_leaf(container: Leaf, depth: i16, min: i32) -> Tree {
    let mut tree = Tree {
        leaf: container,
        lchild: None,
        rchild: None,
    };

    if depth > 0 && tree.leaf.w > min as f32 && tree.leaf.h > min as f32 {
        let (left, right) = random_split(&tree.leaf, min);
        if right.w < min as f32 || right.h < min as f32 {
            return tree
        };
        tree.lchild = Some(Box::from(split_leaf(left, depth - 1, min)));
        tree.rchild = Some(Box::from(split_leaf(right, depth - 1, min)));
    }

    return tree;
}

pub fn random_split(container: &Leaf, min: i32) -> (Leaf, Leaf) {
    let left: Leaf;
    let right: Leaf;

    if random() {
        let max_width = container.w - min as f32;
        println!("max_width: {}", max_width);
        let lw = thread_rng().gen_range(min..=max_width.max(min as f32) as i32) as f32;
        left = Leaf {
            x: container.x - container.w / 2f32 + lw / 2f32,
            y: container.y,
            w: lw,
            h: container.h,
        };
        right = Leaf {
            x: container.x + container.w / 2f32 - ((container.w - left.w) / 2f32),
            y: container.y,
            w: container.w - left.w,
            h: container.h,
        };
    } else {
        let max_height = container.h - min as f32;
        let lh = thread_rng().gen_range(min..=max_height.max(min as f32) as i32) as f32;
        left = Leaf {
            x: container.x,
            y: container.y - container.h / 2f32 + lh / 2f32,
            w: container.w,
            h: lh,
        };
        right = Leaf {
            x: container.x,
            y: container.y + container.h / 2f32 - ((container.h - left.h) / 2f32),
            w: container.w,
            h: container.h - left.h,
        };
    }


    return (left, right);
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

    fn paint_box(self, commands: &mut Commands) {
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
                            w - 1.,
                            h - 1.,
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
        9,
        50);

    let leafs = tree.get_leafs();
    println!("Leafs: {}", leafs.iter().count());
    for leaf in leafs {
        leaf.paint_box(commands);
    }
}