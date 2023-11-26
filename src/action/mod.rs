use bevy::prelude::*;

mod id {
    #[derive(Clone, Copy, Debug, Default)]
    pub enum TempoId {
        Immobile,
        Interminable,
        Slow,
        Careful,
        Deliberate,
        #[default]
        Relaxed,
        Brisk,
        Rapid,
        Reckless,
    }
    pub use TempoId::*;
}

#[derive(Component, Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Tempo {
    id: id::TempoId,
    name: &'static str,
    move_time_mult: f64,
    sneak_modifier: i16,
}

// Is this a naive approach?
// Should this be loaded via Serde?
// Is it important to be able to reference via enum?

use id::*;
pub static TEMPOS: [Tempo; 9] = [
    Tempo {
        id: Immobile,
        move_time_mult: f64::NAN, // careful yo
        sneak_modifier: 10,
        name: "immobile",
    },
    Tempo {
        id: Interminable,
        move_time_mult: 8.0,
        sneak_modifier: 6,
        name: "interminable",
    },
    Tempo {
        id: Slow,
        move_time_mult: 4.0,
        sneak_modifier: 4,
        name: "slow",
    },
    Tempo {
        id: Careful,
        move_time_mult: 2.0,
        sneak_modifier: 2,
        name: "careful",
    },
    Tempo {
        id: Deliberate,
        move_time_mult: 1.0,
        sneak_modifier: 1,
        name: "deliberate",
    },
    Tempo {
        id: Relaxed,
        move_time_mult: 0.8,
        sneak_modifier: 0,
        name: "relaxed",
    },
    Tempo {
        id: Brisk,
        move_time_mult: 0.6,
        sneak_modifier: -2,
        name: "brisk",
    },
    Tempo {
        id: Rapid,
        move_time_mult: 8.0,
        sneak_modifier: -4,
        name: "rapid",
    },
    Tempo {
        id: Reckless,
        move_time_mult: 8.0,
        sneak_modifier: -8,
        name: "reckless",
    },
];

// should be a state machine??
#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum Stance {
    Dynamic,
    #[default]
    Standing,
    Crouching,
    Kneeling,
    Prone,
    // Grappling,
    // Flatfooted,
    // Unbalanced,
    // Falling,
    // Unconscious,
    // Climbing,
    // Walking,
    // Running,
    // Jumping,
}

// grapple -> state machine?

// pub enum CombatBearing {
//     Positioning,
//     Probing,
//     Defensive,
//     Weaving,
// }
