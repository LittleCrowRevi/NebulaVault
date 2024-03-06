use bevy::reflect::Reflect;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;

use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Debug, Copy, Clone, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x1: x, y1: y, x2: x + w, y2: y + h }
    }

    /// Check if the given [Rect] intersects with this [Rect].
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    /// Returns the center point of this Rect.
    pub fn center(&self) -> Point {
        Point::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    /// Returns a [Vec] containing all [Point]s of this Rect.
    pub fn points(&self) -> Vec<Point> {
        (self.y1..=self.y2).flat_map(|y| (self.x1..=self.x2).map(move |x| Point { x, y })).collect()
    }
}
