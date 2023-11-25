use super::Direction;
use bevy::prelude::Component;
use std::cmp::Ordering;
use std::ops::Add;

// Pos3d
//
#[derive(Copy, Clone, Debug, Hash, Component)]
pub struct Pos3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// NOTE: Order is by ( Z, Y, X )
//
impl Ord for Pos3d {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_zyx().cmp(&other.to_zyx())
    }
}

impl PartialOrd for Pos3d {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pos3d {
    fn eq(&self, other: &Self) -> bool {
        self.to_zyx() == other.to_zyx()
    }
}

impl Eq for Pos3d {}

impl Pos3d {
    pub fn to_zyx(&self) -> (i32, i32, i32) {
        (self.z, self.y, self.x)
    }

    pub fn to_xyz(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn adjacent(self, direction: Direction) -> Pos3d {
        direction.offset() + self
    }
}

impl Add for Pos3d {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl From<(i32, i32, i32)> for Pos3d {
    fn from(tuple: (i32, i32, i32)) -> Self {
        Pos3d {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

// Size3d
#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Size3d {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
}

// RectPrism
#[allow(dead_code)]
pub struct RectPrism {
    pub origin: Pos3d,
    pub size: Size3d,
}

// Pos2d
//
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Component)]
pub struct Pos2d {
    pub x: i32,
    pub y: i32,
}

impl Pos2d {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

// Size2d
#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Size2d {
    pub width: i32,
    pub height: i32,
}

// Rect
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Rect {
    pub origin: Pos2d,
    pub size: Size2d,
}
