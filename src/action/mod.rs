use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Component, Debug, Copy, Clone)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn as_xy(self) -> (i32, i32) {
        match self {
            Self::Up => (0, 1),
            Self::UpRight => (1, 1),
            Self::Right => (1, 0),
            Self::DownRight => (1, -1),
            Self::Down => (0, -1),
            Self::DownLeft => (-1, -1),
            Self::Left => (-1, 0),
            Self::UpLeft => (-1, 1),
        }
    }
}

#[allow(dead_code)]
#[derive(Component, Debug)]
pub enum Pace {
    Inactive,    // 0.0
    Painstaking, // 0.25
    Deliberate,  // 0.5
    Relaxed,     // 1.0 * stride
    Brisk,       // 1.5
    Rapid,       // 3.0
    Reckless,    // 6.0
}

#[allow(dead_code)]
#[derive(Component, Debug)]
pub enum Stance {
    Grappling, // other
    Clinch,    // other
    OnGuard,
    Standing,
    Flatfooted,
    Unbalanced,
    Falling,
    Prone,
    Kneeling,
    Jumping,
    Climbing,
}
