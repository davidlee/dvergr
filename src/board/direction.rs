use bevy::math::IVec3;

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
    Up,
    Down,
}

impl Direction {
    pub fn offset(self) -> IVec3 {
        DIRECTION_OFFSETS[self as usize]
    }
}

pub const DIRECTIONS: [Direction; 10] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
    Direction::Up,
    Direction::Down,
];

pub const CARDINAL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub const DIRECTION_OFFSETS: [IVec3; 10] = [
    IVec3 { x: 0, y: 1, z: 0 },
    IVec3 { x: 1, y: 1, z: 0 },
    IVec3 { x: 1, y: 0, z: 0 },
    IVec3 { x: 1, y: -1, z: 0 },
    IVec3 { x: 0, y: -1, z: 0 },
    IVec3 { x: -1, y: -1, z: 0 },
    IVec3 { x: -1, y: 0, z: 0 },
    IVec3 { x: -1, y: 1, z: 0 },
    IVec3 { x: 0, y: 0, z: 1 },
    IVec3 { x: 0, y: 0, z: -1 },
];

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
