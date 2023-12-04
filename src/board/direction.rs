use crate::typical::*;

use std::f32::consts::TAU;

// Direction
//
#[derive(Eq, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    const DIRECTION_NUM: usize = 8;

    pub fn offset(self) -> IVec3 {
        DIRECTION_OFFSETS[self as usize]
    }

    pub fn offset2d(self) -> IVec2 {
        let o = self.offset();
        IVec2::new(o.x, o.y)
    }

    pub fn offset2df(self) -> Vec2 {
        let v = self.offset2d();
        Vec2::new(v.x as f32, v.y as f32)
    }

    pub fn arc_vectors(self, n: usize) -> [Vec2; 2] {
        [
            Self::ivec3_to_vec2(DIRECTION_OFFSETS[self.counter_clockwise_neigbour(n) as usize]),
            Self::ivec3_to_vec2(DIRECTION_OFFSETS[self.clockwise_neighbour(n) as usize]),
        ]
    }

    fn ivec3_to_vec2(ivec3: IVec3) -> Vec2 {
        let [x, y, _] = ivec3.to_array();
        Vec2::new(x as f32, y as f32)
    }

    pub fn clockwise_neighbour(self, n: usize) -> Self {
        DIRECTIONS[(self as usize + n) % Self::DIRECTION_NUM]
    }

    pub fn counter_clockwise_neigbour(self, n: usize) -> Self {
        DIRECTIONS[(Self::DIRECTION_NUM + self as usize - n) % Self::DIRECTION_NUM]
    }

    pub fn to_degrees(self) -> f32 {
        DIRECTION_RADIANS[self as usize] * TAU
    }

    pub fn to_radians(self) -> f32 {
        DIRECTION_RADIANS[self as usize]
    }
}

pub const DIRECTIONS: [Direction; 8] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

pub const CARDINAL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub const DIRECTION_OFFSETS: [IVec3; 8] = [
    IVec3 { x: 0, y: 1, z: 0 },
    IVec3 { x: 1, y: 1, z: 0 },
    IVec3 { x: 1, y: 0, z: 0 },
    IVec3 { x: 1, y: -1, z: 0 },
    IVec3 { x: 0, y: -1, z: 0 },
    IVec3 { x: -1, y: -1, z: 0 },
    IVec3 { x: -1, y: 0, z: 0 },
    IVec3 { x: -1, y: 1, z: 0 },
];

// * TAU = degrees
pub const DIRECTION_RADIANS: [f32; 8] = [0.0, 0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875];

pub const COMPASS_DEGREES: [f32; 8] = [0., 45., 90., 135., 180., 225., 270., 315.];
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
