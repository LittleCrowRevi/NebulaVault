use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::process::Command;

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
            dim: Rect::from_corners(vec2(0., 0.), vec2(10., 10.)),
        }
    }
    pub fn rectange_room() -> Self {
        Self {
            dim: Rect::from_corners(vec2(0., 0.), vec2(10., 20.)),
        }
    }

    pub fn thiny_room() -> Self {
        Self {
            dim: Rect::from_corners(vec2(0., 0.), vec2(5.0, 10.0)),
        }
    }

    pub fn thinx_room() -> Self {
        Self {
            dim: Rect::from_corners(vec2(0., 0.), vec2(10., 5.)),
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

#[derive(Clone, Debug)]
pub struct Tree {
    leaf: Leaf,
    lchild: Option<Box<Tree>>,
    rchild: Option<Box<Tree>>,
}

impl Tree {
    pub fn get_leafs(self) -> Vec<Leaf> {
        if self.lchild.is_none() && self.rchild.is_none() {
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

pub fn split_leaf(container: &Leaf, depth: i16, min: i32, rng: ChaCha20Rng) -> Tree {
    let mut tree = Tree {
        leaf: *container,
        lchild: None,
        rchild: None,
    };

    if depth > 0 && tree.leaf.w > min as f32 && tree.leaf.h > min as f32 {
        let (left, right) = random_split(&tree.leaf, min, rng.clone());
        if right.w < min as f32 || right.h < min as f32 {
            return tree;
        };
        tree.lchild = Some(Box::from(split_leaf(&left, depth - 1, min, rng.clone())));
        tree.rchild = Some(Box::from(split_leaf(&right, depth - 1, min, rng)));
    }

    tree
}

pub fn random_split(container: &Leaf, min: i32, mut rng: ChaCha20Rng) -> (Leaf, Leaf) {
    let left: Leaf;
    let right: Leaf;

    if random() {
        let max_width = container.w - min as f32;
        println!("max_width: {}", max_width);
        let lw = rng.gen_range(min..=max_width.max(min as f32) as i32) as f32;
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

    (left, right)
}

#[derive(Component, Clone, Debug, Copy)]
pub struct Leaf {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Leaf {
    fn paint_box(self, commands: &mut Commands, color: Color, z_index: f32, border: f32) {
        let (x, y, w, h) = (self.x, self.y, self.w, self.h);
        let mut leaf = commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(vec3(x, y, z_index)),
                visibility: Visibility::Visible,
                sprite: Sprite {
                    color,
                    custom_size: Some(vec2(w - border, h - border)),
                    ..default()
                },
                ..default()
            },
            self,
        ));
    }

    pub fn area(&self) -> f32 {
        self.w * self.h
    }
}

pub fn generate_bsp(commands: &mut Commands, seed: &Leaf, depth: i16, min: i32) {
    let rng = ChaCha20Rng::from_entropy();
    let tree = split_leaf(seed, depth, min, rng);

    let leafs = tree.clone().get_leafs();
    tree.leaf.paint_box(commands, Color::rgb(0.5, 0.5, 0.5), -1.0, 0.0);
    leafs.iter().for_each(|leaf| leaf.paint_box(commands, Color::rgb(0.15, 0.15, 0.15), 0.0, 5.0));
}

#[derive(Event)]
pub struct EventGrowBSPTree {
    pub(crate) seed: Leaf,
    pub(crate) depth: i16,
    pub(crate) min: i32,
}

pub fn redraw_map(
    mut commands: Commands,
    mut ev_tree_grow: EventReader<EventGrowBSPTree>,
    leafs: Query<Entity, With<Leaf>>,
) {
    for ev in ev_tree_grow.read() {
        leafs.for_each(|e| commands.entity(e).despawn_recursive());
        generate_bsp(&mut commands, &ev.seed, ev.depth, ev.min);
    }
}
