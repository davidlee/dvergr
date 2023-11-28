use crate::action::Stance;
use crate::action::Tempo;
use crate::board::Area3d;
use crate::board::Direction;
use crate::board::Position;
use bevy::math::UVec3;

use bevy::prelude::{Bundle, Component};

pub mod movement {
    // use super::*;
    // use crate::board::Area3d;
    use crate::board::Board;
    use crate::board::Position;
    use crate::creature::Creature;
    use bevy::math::UVec3;
    use bevy::prelude::EventReader;
    use bevy::prelude::{Entity, Event, Query, ResMut};

    // TODO support multiple cells
    #[derive(Event, Debug)]
    pub struct StartMove {
        pub from: UVec3,
        pub to: UVec3,
        pub entity: Entity,
    }

    impl StartMove {
        pub fn single(from: UVec3, to: UVec3, entity: Entity) -> Self {
            StartMove { from, to, entity }
        }
    }

    pub fn process_movement(
        mut ev_move: EventReader<StartMove>,
        mut board: ResMut<Board>,
        mut query: Query<(Entity, &mut Creature)>,
    ) {
        for e in ev_move.read() {
            println!("processing movement .. {:?}", e);
            let (entity, mut creature) = query.get_mut(e.entity).unwrap();
            // first make the changes to the creature
            creature.locus.position = Position::Point(e.to);
            // then reflect the changes on the board's creatures mapping
            board.creature_entities.update_single(entity, e.to).unwrap();
        }
    }
}

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct Creature {
    pub species: Species,
    pub phenotype: Phenotype,
    pub size: CreatureSize,
    pub base_weight: f64,
    pub condition: CreatureCondition,
    pub locus: Locus,
    pub tempo: Tempo, // tempo? pace?
    pub actions: Actions,

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
            base_weight: 80.0,
            locus: Locus::default(),
            tempo: crate::action::TEMPOS[0].clone(),
            templates: (),
            actions: Actions::default(),
            // gear: Equipment::default(),
            condition: CreatureCondition::default(),
            abilities: (),
            traits: (),
        }
    }

    pub fn set_pos(mut self, pos: UVec3) {
        self.locus.position = Position::Point(pos);
    }
}

#[derive(Bundle, Debug, Clone)]
pub struct CreatureBundle {}

// Locus
//

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Locus {
    pub position: Position,
    pub speed: i32,
    pub direction: Direction,
    pub facing: Direction,
    pub stance: Stance,
    pub weight: f64,
}

impl Locus {
    pub fn set_pos(&mut self, pos: UVec3) {
        self.position = Position::Point(pos);
    }

    pub fn set_area(&mut self, area: Area3d) {
        self.position = Position::Area(area);
    }
}

impl Default for Locus {
    fn default() -> Self {
        Locus {
            position: Position::Point(UVec3::new(0, 0, 0)),
            speed: 0,
            direction: Direction::North,
            facing: Direction::North,
            stance: Stance::Standing,
            weight: 80.0,
        }
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

// Condition
//
#[derive(Component, Debug, Clone, Default, Eq, PartialEq)]
#[allow(dead_code)]
pub struct CreatureCondition {
    needs: (),
    conditions: (),
    injuries: (),
    encumberance: (),
}

impl CreatureCondition {
    fn default() -> Self {
        CreatureCondition {
            needs: (),
            conditions: (),
            injuries: (),
            encumberance: (),
        }
    }
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

// Species
//
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

// Phenotype
//
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

// Equipment
//
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
