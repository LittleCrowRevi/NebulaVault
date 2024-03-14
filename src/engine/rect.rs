use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Sub;

use bevy::reflect::Reflect;
use bevy::utils::HashSet;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;

use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn from_vec(x: i32, y: i32, vec: (i32, i32)) -> Self {
        Self { x: x + vec.0, y: y + vec.1 }
    }

    pub fn neighbors(self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
            // Add diagonal neighbors if needed
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x + 1, self.y + 1),
        ]
    }

    pub fn line_of_sight(from: Point, to: Point) -> HashSet<Point> {
        let mut line = HashSet::new();
        let (dx, dy) = ((to.x - from.x), (to.y - from.y));
        let (nx, ny) = (dx.abs(), dy.abs());
        let sx = if dx > 0 { 1 } else { -1 };
        let sy = if dy > 0 { 1 } else { -1 };

        let mut p = from;
        line.insert(p);
        let mut ix = 0;
        let mut iy = 0;
        while ix < nx || iy < ny {
            let decision = (1 + 2 * ix) * ny - (1 + 2 * iy) * nx;
            if decision == 0 {
                // next step is diagonal
                p.x += sx;
                p.y += sy;
                ix += 1;
                iy += 1;
            } else if decision < 0 {
                // next step is horizontal
                p.x += sx;
                ix += 1;
            } else {
                // next step is vertical
                p.y += sy;
                iy += 1;
            }
            line.insert(p);
        }

        line
    }

    pub fn can_see(self, target: Point, map: &Map) -> bool {
        // Check if line of sight from self to target is unobstructed
        let line_of_sight_from_self = Point::line_of_sight(self, target);
        let line_of_sight_from_target = Point::line_of_sight(target, self);

        let equal = line_of_sight_from_self == line_of_sight_from_target;
        let no_obstraction = line_of_sight_from_self.iter().all(|&p| !map.is_opaque(map.xy_idx(p)));

        equal && no_obstraction
    }
}

impl AddAssign<(i32, i32)> for Point {
    fn add_assign(&mut self, other: (i32, i32)) {
        *self = Self { x: self.x + other.0, y: self.y + other.1 }
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub<(i32, i32)> for Point {
    type Output = Point;

    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        Self { x: self.x - rhs.0, y: self.y - rhs.1 }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl From<Point> for IVec2 {
    fn from(value: Point) -> Self {
        IVec2::new(value.x, value.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Reflect, InspectorOptions)]
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
    pub fn intersect(&self, other: &Rect, border: i32) -> bool {
        self.x1 < other.x2 + border && self.x2 > other.x1 + border && self.y1 < other.y2 && self.y2 > other.y1 + border
    }

    /// Returns the center point of this Rect.
    pub fn center(&self) -> Point {
        Point::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    /// Returns a [Vec] containing all [Point]s of this Rect.
    pub fn points(&self) -> Vec<Point> {
        (self.y1..=self.y2).flat_map(|y| (self.x1..=self.x2).map(move |x| Point { x, y })).collect()
    }

    pub fn outer_rim(&self) -> Vec<Point> {
        let points: Vec<Point> = (self.x1..=self.x2)
            .flat_map(|x| vec![Point::new(x, self.y1), Point::new(x, self.y2)])
            .chain((self.y1..self.y2).flat_map(|y| vec![Point::new(self.x1, y), Point::new(self.x2, y)]))
            .collect();

        points
    }
}
