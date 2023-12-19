use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::process::Command;
use crate::systems::maps::TILE_SIZE;

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

#[derive(Component)]
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
    Npc,
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
            v.append(&mut self.lchild.unwrap().get_leafs());
            v.append(&mut self.rchild.unwrap().get_leafs());
            v
        }
    }
}

pub fn split_leaf(container: &Leaf, depth: i16, min: (i32, i32), rng: [u8; 32]) -> Tree {
    let mut tree = Tree {
        leaf: *container,
        lchild: None,
        rchild: None,
    };

    if depth > 0 && tree.leaf.w > min.0 && tree.leaf.h > min.1 {
        let (left, right) = random_split(&tree.leaf, min, rng);
        if right.w < min.0 || right.h < min.1 || ChaCha20Rng::from_seed(rng).gen_bool(0.3) {
            return tree;
        };
        
        tree.lchild = Some(Box::from(split_leaf(&left, depth - 1, min, rng)));
        tree.rchild = Some(Box::from(split_leaf(&right, depth - 1, min, rng)));
    }
    tree
}

pub fn generate_room(container: &Leaf, rng: [u8; 32], commands: &mut Commands, color: Color) {
    //if let Some(l_leaf) = container.lchild.as_ref() { generate_room(l_leaf) }
    //if let Some(r_leaf) = container.rchild.as_ref() { generate_room(r_leaf) }
    let max_w = container.w - TILE_SIZE.0 * 3;
    let max_h = container.h - TILE_SIZE.1 * 3;

    let rand_w = ChaCha20Rng::from_seed(rng).gen_range(TILE_SIZE.0..=max_w.max(TILE_SIZE.0));
    let rand_h = ChaCha20Rng::from_seed(rng).gen_range(TILE_SIZE.1..=max_h.max(TILE_SIZE.1));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(vec3(container.x as f32, container.y as f32, 3.)),
            visibility: Visibility::Visible,
            sprite: Sprite {
                color,
                custom_size: Some(vec2(rand_w as f32, rand_h as f32)),
                ..default()
            },
            ..default()
        },
        Room {
            dim: Rect::from_corners(Vec2::ZERO, vec2(rand_w as f32, rand_h as f32)),
        }
    ));
}

pub fn random_split(container: &Leaf, min: (i32, i32), rng: [u8; 32]) -> (Leaf, Leaf) {
    let left: Leaf;
    let right: Leaf;

    if random() {
        let max_width = container.w - min.0;
        let lw = ChaCha20Rng::from_seed(rng).gen_range(min.0..=max_width.max(min.0));
        left = Leaf {
            x: container.x - container.w / 2 + lw / 2,
            y: container.y,
            w: lw,
            h: container.h,
        };
        right = Leaf {
            x: container.x + container.w / 2 - ((container.w - left.w) / 2),
            y: container.y,
            w: container.w - left.w,
            h: container.h,
        };
    } else {
        let max_height = container.h - min.1;
        let lh = thread_rng().gen_range(min.1..=max_height.max(min.1));
        left = Leaf {
            x: container.x,
            y: container.y - container.h / 2 + lh / 2,
            w: container.w,
            h: lh,
        };
        right = Leaf {
            x: container.x,
            y: container.y + container.h / 2 - ((container.h - left.h) / 2),
            w: container.w,
            h: container.h - left.h,
        };
    }

    (left, right)
}

#[derive(Component, Clone, Debug, Copy)]
pub struct Leaf {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Leaf {
    fn paint_box(self, commands: &mut Commands, color: Color, z_index: f32, border: f32) {
        let (x, y, w, h) = (self.x as f32, self.y as f32, self.w as f32, self.h as f32);
        commands.spawn((
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

    pub fn area(&self) -> i32 {
        self.w * self.h
    }
}

pub fn generate_bsp(commands: &mut Commands, seed: &Leaf, depth: i16, min: (i32, i32)) {
    let rng = ChaCha20Rng::from_entropy().get_seed();
    let tree = split_leaf(seed, depth, min, rng);

    let leafs = tree.clone().get_leafs();
    if leafs.len() <= 1 { return generate_bsp(commands, seed, depth, min); }
    
    tree.leaf.paint_box(commands, Color::rgb(0.5, 0.5, 0.5), -1.0, 0.0);
    for leaf in &leafs {
        leaf.paint_box(commands, Color::rgb(0.15, 0.15, 0.15), 0.0, 10.0);
        generate_room(leaf, rng, commands, Color::rgb(0.25, 0.25, 0.25));
    }
}

#[derive(Event)]
pub struct EventGrowBSPTree {
    pub(crate) seed: Leaf,
    pub(crate) depth: i16,
    pub(crate) min: (i32, i32),
}

pub fn redraw_map(
    mut commands: Commands,
    mut ev_tree_grow: EventReader<EventGrowBSPTree>,
    leafs: Query<Entity, With<Leaf>>,
    rooms: Query<Entity, With<Room>>,
) {
    for ev in ev_tree_grow.read() {
        rooms.for_each(|e| commands.entity(e).despawn_recursive());
        leafs.for_each(|e| commands.entity(e).despawn_recursive());
        generate_bsp(&mut commands, &ev.seed, ev.depth, ev.min);
    }
}
