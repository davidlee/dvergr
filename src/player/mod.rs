use crate::action::Stance;
use crate::action::Tempo;
use crate::board::Direction;
use crate::board::Pos;
use bevy::sprite::Anchor;

use crate::attributes::Attributes;
use bevy::prelude::{Bundle, Component};

#[derive(Component, Debug, Clone, Copy)]
pub struct Player;

#[derive(Component, Debug, Clone, Default)]
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

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct Actor {
    current_action: (),
    action_queue: (),
    behaviour_tree: Option<()>,
}

#[derive(Component, Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Default)]
#[allow(dead_code)]
pub struct Species {
    name: String,
    anatomy_template: (),
    subtype: Option<()>,
}

impl Species {
    fn default() -> Self {
        Species {
            name: String::from("human"),
            anatomy_template: (),
            subtype: None,
        }
    }

    pub fn humanoid(name: &str) -> Self {
        Species {
            name: String::from(name),
            anatomy_template: (),
            subtype: None,
        }
    }

    pub fn human() -> Self {
        Species::humanoid("human")
    }
}

#[derive(Component, Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Phenotype {
    species: Species,
    size: CreatureSize,
    anatomy_template: (),

    natural_weapons: (),
    natural_armour: (),
    natural_inventory: (),

    innate_abilities: (),
    traits: (),
    // metabolism
    // needs
    // thoughts ..
}

impl Phenotype {
    fn default() -> Self {
        Phenotype {
            species: Species::default(),
            size: CreatureSize::default(),
            anatomy_template: (),

            natural_weapons: (),
            natural_armour: (),
            natural_inventory: (),
            innate_abilities: (),
            traits: (),
        }
    }
}

#[derive(Component, Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Equipment {
    worn_armour: (),
    equipped: (),
    wearing: (),
    carrying: (),
}

#[allow(dead_code)]
impl Equipment {
    fn default() -> Self {
        Equipment {
            worn_armour: (),
            equipped: (),
            wearing: (),
            carrying: (),
        }
    }
}

#[derive(Component, Debug, Clone, Default)]
#[allow(dead_code)]
pub struct CreatureCondition {
    needs: (),
    conditions: (),
    injuries: (),
}

impl CreatureCondition {
    fn default() -> Self {
        CreatureCondition {
            needs: (),
            conditions: (),
            injuries: (),
        }
    }
}

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct Creature {
    position: Pos,
    anchor: Anchor,
    stance: Stance,
    facing: Direction,
    tempo: Tempo,

    species: Species,
    phenotype: Phenotype,
    // age, disease, subspecies, careers, etc
    // a geriatric leprous veteran undead wood-elf pirate
    // a deranged adolescent amputee ex-slave sprite
    templates: (),
    // gear: Equipment,
    // attributes
    condition: CreatureCondition,
    abilities: (),
    traits: (),
}

impl Creature {
    pub fn human(position: Pos) -> Self {
        Creature {
            position,
            anchor: Anchor::default(),
            stance: Stance::default(),
            facing: Direction::North,
            tempo: crate::action::TEMPOS[0].clone(),
            species: Species::human(),
            phenotype: Phenotype::default(),
            templates: (),
            // gear: Equipment::default(),
            condition: CreatureCondition::default(),
            abilities: (),
            traits: (),
        }
    }
}

#[derive(Bundle, Debug, Clone)]
pub struct CreatureBundle {}

#[derive(Bundle, Debug, Clone)]
pub struct PlayerBundle {
    player: Player,
    attributes: Attributes,
}

#[allow(dead_code)]
impl PlayerBundle {
    fn new() -> Self {
        PlayerBundle::default()
    }

    fn default() -> Self {
        PlayerBundle {
            player: Player,
            attributes: Attributes::new(),
        }
    }
}
