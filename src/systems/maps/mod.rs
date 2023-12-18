use bevy::math::vec2;
use bevy::prelude::Vec2;
use crate::systems::maps::map_builder::{EventGrowBSPTree, Leaf};

pub mod map_builder;

pub const TILE_SIZE: (i32, i32) = (32, 48);

pub const LEAF_DEV: EventGrowBSPTree = EventGrowBSPTree  {seed: Leaf { x: 0, y: 0, w: 10000, h: 10000}, min: 1500, depth: 7};