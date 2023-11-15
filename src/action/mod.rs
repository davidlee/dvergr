use bevy::prelude::*;

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
