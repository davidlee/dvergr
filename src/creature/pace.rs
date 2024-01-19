#![allow(dead_code)]

use crate::typical::*;

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
    Focused,
    #[default]
    Casual,
    Hurried,
    Desperate,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum PaceId {
    Immobile,
    Interminable, // crawl, listen / spot traps AND sneak
    Slow,         // listen + spot traps
    Cautious,     // sneak; move carefully over trecherous terrain
    Deliberate,   //
    #[default]
    Relaxed, // friends walking and socialising
    Brisk,        // very fast walk
    Rapid,        // jogging
    Running,      // running
    Reckless,     // sprinting
}

pub use PaceId::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
#[allow(dead_code)]
pub struct Pace {
    id: PaceId,
    name: &'static str,
    ticks_per_cell: u32,
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
        ticks_per_cell: u32::MAX,
        sneak_modifier: 12,
        name: "immobile",
    },
    Pace {
        id: Interminable,
        ticks_per_cell: 80,
        sneak_modifier: 6,
        name: "interminable",
    },
    Pace {
        id: Slow,
        ticks_per_cell: 40,
        sneak_modifier: 4,
        name: "slow",
    },
    Pace {
        id: Cautious,
        ticks_per_cell: 20,
        sneak_modifier: 2,
        name: "cautious",
    },
    Pace {
        id: Deliberate,
        ticks_per_cell: 10,
        sneak_modifier: 1,
        name: "deliberate",
    },
    Pace {
        id: Relaxed,
        ticks_per_cell: 8,
        sneak_modifier: 0,
        name: "relaxed",
    },
    Pace {
        id: Brisk,
        ticks_per_cell: 6,
        sneak_modifier: -2,
        name: "brisk",
    },
    Pace {
        id: Rapid,
        ticks_per_cell: 3,
        sneak_modifier: -4,
        name: "rapid",
    },
    Pace {
        id: Reckless,
        ticks_per_cell: 1, // no resolution to go faster without double-moves
        sneak_modifier: -8,
        name: "reckless",
    },
];

#[test]
fn pace_index() {
    assert_eq!(Pace::get(PaceId::Cautious).id, PaceId::Cautious);
    assert_eq!(PACE[PaceId::Cautious as usize].id, PaceId::Cautious);
}
