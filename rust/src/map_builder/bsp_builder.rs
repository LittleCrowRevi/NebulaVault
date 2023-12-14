use bevy_ecs::prelude::*;
use rand::{random, Rng, thread_rng};

#[derive(Event)]
pub struct EventGrowBSPTree {
    seed: Leaf,
    depth: i16,
    min: i32
}

#[derive(Component, Clone)]
pub struct Leaf {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Leaf {

    fn paint_box(self, commands: &mut Commands) {
        
    }
}

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

