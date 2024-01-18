use crate::typical::*;

pub(crate) mod anatomy;
pub(crate) mod attributes;
pub(crate) mod character;
pub(crate) mod condition;
pub(crate) mod locus;
pub(crate) mod movement;
pub(crate) mod pace;

pub(crate) use character::*;
pub(crate) use pace::*;

pub(crate) use attributes::*;
pub(crate) use condition::*;
pub(crate) use locus::*;

#[derive(Bundle, Debug, Clone)]
pub(crate) struct CreatureBundle {
    pub creature: Creature,
    pub attributes: Attributes,
    pub species: Species,
    pub gender: Gender,
    pub needs: NeedList,
    pub size: CreatureSize,
    pub skills: SkillList,
    pub abilities: AbilityList,
    pub pace: Pace,
    pub tempo: Tempo,
    pub stance: Stance,
    pub approach: Approach,
    pub conditions: ConditionList,
    pub locus: Locus,
}

impl Default for CreatureBundle {
    fn default() -> Self {
        Self {
            creature: Creature::default(),
            attributes: Attributes::new(),
            species: Species::Dwarf,
            gender: Gender::Male,
            needs: NeedList::default(),
            size: CreatureSize::default(),
            skills: SkillList::default(),
            abilities: AbilityList::default(),
            pace: Pace::default(),
            tempo: Tempo::default(),
            stance: Stance::default(),
            approach: Approach::default(),
            conditions: ConditionList::default(),
            locus: Locus::default(),
        }
    }
}

// CREATURE
//

#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub struct Creature {
    dry_weight: f32, // kg
    height: i32,     // cm
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

#[derive(Component, Debug, Clone, Default)]
pub struct Age(pub u16);

#[derive(Component, Debug, Clone, Default)]
#[allow(dead_code)]
pub(crate) struct SkillList {
    pub boxing: u8,
    pub striking: u8,
    pub grappling: u8,
    pub dagger: u8,
    pub sword: u8,
    pub axe: u8,
    pub staff: u8,
    pub greatsword: u8,
    pub bow: u8,
    pub sling: u8,
    pub javelin: u8,
    pub marksman: u8,
    pub pistol: u8,
    pub dueling: u8,
    pub melee: u8,
    pub fencing: u8,
    pub shield: u8,
    pub two_weapon: u8,

    pub listen: u8,
    pub notice: u8,
    pub sneak: u8,
    pub ambush: u8,
    pub climb: u8,
    pub athletics: u8,
    pub survival: u8,
    pub alcoholism: u8,
    pub herbalism: u8,
    pub mycology: u8,
    pub zoology: u8,
    pub cooking: u8,
    pub husbandry: u8,
    pub alchemy: u8,
    pub poisons: u8,
    pub apothecary: u8,
    pub diagnosis: u8,
    pub first_aid: u8,
    pub surgery: u8,
    pub mining: u8,
    pub smithing: u8,
    pub carpentry: u8,
    pub masonry: u8,
    pub leatherwork: u8,
    pub gemcutting: u8,
    pub locks: u8,
    pub mechanics: u8,
    pub trapping: u8,
    pub attunement: u8,
    pub ritual: u8,
    pub divination: u8,
    pub summoning: u8,
    pub enchantment: u8,
    pub bargain: u8,
    pub law: u8,
    pub runes: u8,
    pub trance: u8,
}

#[derive(Component, Debug, Clone, Default)]
pub(crate) struct AbilityList {}

// #[derive(Component, Debug, Clone, Default)]
// pub struct SpellList {}

// grapple -> state machine?

// pub enum CombatBearing {
//     Positioning,
//     Probing,
//     Defensive,
//     Weaving,
// }
