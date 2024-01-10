use crate::typical::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum PaceId {
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

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Structure {
    // structure, in fencing lingo : distance / range?
    #[default]
    Normal,
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Measure {
    #[default]
    Normal, //
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tempo {
    // the combat equivalent
    #[default]
    Normal,
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Approach {
    // the task equivalent
    Painstaking,
    Careful,
    #[default]
    Casual,
    Hurried,
    Desperate,
}

pub use PaceId::*;

#[derive(Component, Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Pace {
    id: PaceId,
    name: &'static str,
    move_time_mult: f64,
    sneak_modifier: i16,
}

impl Default for Pace {
    fn default() -> Self {
        Pace::get(PaceId::default())
    }
}

impl Pace {
    fn get(id: PaceId) -> Pace {
        PACE[id as usize]
    }
}

const PACE: [Pace; 9] = [
    Pace {
        id: Immobile,
        move_time_mult: f64::NAN, // careful yo
        sneak_modifier: 10,
        name: "immobile",
    },
    Pace {
        id: Interminable,
        move_time_mult: 8.0,
        sneak_modifier: 6,
        name: "interminable",
    },
    Pace {
        id: Slow,
        move_time_mult: 4.0,
        sneak_modifier: 4,
        name: "slow",
    },
    Pace {
        id: Careful,
        move_time_mult: 2.0,
        sneak_modifier: 2,
        name: "careful",
    },
    Pace {
        id: Deliberate,
        move_time_mult: 1.0,
        sneak_modifier: 1,
        name: "deliberate",
    },
    Pace {
        id: Relaxed,
        move_time_mult: 0.8,
        sneak_modifier: 0,
        name: "relaxed",
    },
    Pace {
        id: Brisk,
        move_time_mult: 0.6,
        sneak_modifier: -2,
        name: "brisk",
    },
    Pace {
        id: Rapid,
        move_time_mult: 8.0,
        sneak_modifier: -4,
        name: "rapid",
    },
    Pace {
        id: Reckless,
        move_time_mult: 8.0,
        sneak_modifier: -8,
        name: "reckless",
    },
];

#[test]
fn pace_index() {
    assert_eq!(Pace::get(PaceId::Careful).id, PaceId::Careful);
    assert_eq!(PACE[PaceId::Careful as usize].id, PaceId::Careful);
}
