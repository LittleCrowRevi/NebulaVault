use crate::systems::maps::map_builder::{EventGrowBSPTree, Leaf};

pub mod map_builder;


pub const LEAF_DEV: EventGrowBSPTree = EventGrowBSPTree  {seed: Leaf { x: 0., y: 0., w: 1000., h: 1000.}, min: 200, depth: 5};