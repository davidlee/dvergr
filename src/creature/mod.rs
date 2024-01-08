use crate::typical::*;

pub mod attributes;
pub mod condition;
pub mod locus;
pub mod movement;
pub mod phenotype;

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
    pub size: CreatureSize,
    pub condition: CreatureCondition,
    pub locus: Locus,
    pub actions: Actions,
    // tempo: Tempo, // tempo? pace?
    // // age, disease, subspecies, careers, etc
    // // a geriatric leprous veteran undead wood-elf pirate
    // // a deranged adolescent amputee ex-slave sprite
    // templates: (),
    // // gear: Equipment,
    // // attributes
    // abilities: (),
    // traits: (),
}

impl Default for CreatureBundle {
    fn default() -> Self {
        Self {
            creature: Creature::default(),
            attributes: Attributes::new(),
            phenotype: Phenotype::default(),
            condition: CreatureCondition::default(),
            locus: Locus::default(),
            size: CreatureSize::Medium,
            // tempo: Tempo::
            actions: Actions::default(),
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
// Actions
//
#[derive(Component, Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Actions {
    current: (),
    queue: (),
    // behaviour_tree: Option<()>,
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
