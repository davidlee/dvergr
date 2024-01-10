use crate::typical::*;

pub mod attributes;
pub mod character;
pub mod condition;
pub mod locus;
pub mod movement;
pub mod pace;
pub mod phenotype;

pub use crate::anatomy::Gender;
pub use character::*;
pub use pace::*;

pub use attributes::*;
pub use condition::*;
pub use locus::*;
pub use movement::*;
pub use phenotype::*;

#[allow(dead_code)]
#[derive(Bundle, Debug, Clone)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub attributes: Attributes,
    pub phenotype: Phenotype,
    pub species: Species,
    pub gender: Gender,
    pub age: Age,
    pub size: CreatureSize,
    pub pace: Pace,
    pub tempo: Tempo,
    pub stance: Stance,
    pub approach: Approach,
    pub condition: ConditionList,
    pub locus: Locus,
    pub actions: ActionList,
    // // age, disease, subspecies, careers, etc
    // // a geriatric leprous veteran undead wood-elf pirate
    // // a deranged adolescent amputee ex-slave wood sprite
    // templates: (),
    // gear: Equipment,
    // traits: (),
}

impl Default for CreatureBundle {
    fn default() -> Self {
        Self {
            creature: Creature::default(),
            attributes: Attributes::new(),
            phenotype: Phenotype::default(),
            condition: ConditionList::default(),
            locus: Locus::default(),
            size: CreatureSize::Medium,
            // tempo: Tempo::
            actions: ActionList::default(),
            ..default()
        }
    }
}

impl CreatureBundle {
    pub fn human() -> Self {
        Self::default()
    }
}

// CREATURE
//

#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub struct Creature {
    dry_weight: f32, // kg
    height: i32,     // cm
                     // age:
}

impl Default for Creature {
    fn default() -> Self {
        Creature {
            dry_weight: 80.,
            height: 178,
        }
    }
}

impl Creature {
    pub fn human() -> Self {
        Self::default()
    }
}
// Size
//
#[derive(Component, Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum CreatureSize {
    Insect,
    Tiny,  // kitten
    Small, // Human child; fox
    #[default]
    Medium, // Human adult
    Large, // horse; ogre
    Giant, // two story humanoid; war elephant
    Leviathan(), // show me map tiles
}

pub enum Size {
    Bug,    // diminutive
    Cat,    // tiny
    Monkey, // small
    Wolf,   //
    Man,    // Medium
    Tiger,  //
    Bear,   // Large
    Horse,  //
    Bison,  // Larger
    Hippopotamus,
    Elephant,  //
    Leviathan, // expressed in map tiles
}
// Actions
//
#[derive(Component, Debug, Clone, Default)]
#[allow(dead_code)]
pub struct ActionList {
    pub current: (),
    pub queue: (),
    // behaviour_tree: Option<()>,
}
#[derive(Component, Debug, Clone, Default)]
pub struct NeedList {
    pub hunger: u8,
    pub thirst: u8,
    pub sleep: u8,
    pub morale: u8,
    pub breath: u8,
    pub blood: f32, // liters
    pub hope: u8,
    pub safety: u8,
}

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
