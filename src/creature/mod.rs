use crate::action::Stance;
use crate::action::Tempo;
use crate::board::Direction;

// use crate::attributes::Attributes;
use bevy::prelude::{Bundle, Component};

pub mod movement {
    // use super::*;
    use crate::board::Pos3d;
    use bevy::prelude::{Entity, Event};
    // use bevy::prelude::{EventReader, EventWriter};

    // TODO multiple cells
    #[derive(Event, Debug)]
    pub struct StartMove {
        pub from: Pos3d,
        pub to: Pos3d,
        pub entity: Entity,
    }

    impl StartMove {
        pub fn single(from: Pos3d, to: Pos3d, entity: Entity) -> Self {
            StartMove { from, to, entity }
        }
    }
}

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct Creature {
    pub species: Species,
    pub phenotype: Phenotype,
    // pub position: Pos3d,
    pub size: CreatureSize,
    pub condition: CreatureCondition,
    //
    pub facing: Direction,
    pub stance: Stance,
    pub tempo: Tempo,
    pub actions: Actions,
    //

    // age, disease, subspecies, careers, etc
    // a geriatric leprous veteran undead wood-elf pirate
    // a deranged adolescent amputee ex-slave sprite
    templates: (),
    // gear: Equipment,
    // attributes
    abilities: (),
    traits: (),
}

impl Creature {
    pub fn human() -> Self {
        Creature {
            // position,
            phenotype: Phenotype::default(),
            species: Species::human(),
            size: CreatureSize::Medium,
            stance: Stance::default(),
            facing: Direction::North,
            tempo: crate::action::TEMPOS[0].clone(),
            templates: (),
            actions: Actions::default(),
            // gear: Equipment::default(),
            condition: CreatureCondition::default(),
            abilities: (),
            traits: (),
        }
    }
}

#[derive(Bundle, Debug, Clone)]
pub struct CreatureBundle {}

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

#[derive(Component, Debug, Clone, Default, Eq, PartialEq)]
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

#[derive(Component, Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Actions {
    current: (),
    queue: (),
    // behaviour_tree: Option<()>,
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

#[derive(Component, Debug, Clone, Default, Eq, PartialEq)]
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

#[derive(Component, Debug, Clone, Default, Eq, PartialEq)]
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
