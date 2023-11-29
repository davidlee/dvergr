use crate::action::Stance;
use crate::board::Area3d;
use crate::typical::*;

pub mod movement;
pub use movement::*;

#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub attributes: Attributes,
    pub species: Species,
    pub phenotype: Phenotype,
    pub size: CreatureSize,
    pub base_weight: f64,
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
            species: Species::human(),
            phenotype: Phenotype::default(),
            size: CreatureSize::Medium,
            base_weight: 80.0,
            condition: CreatureCondition::default(),
            locus: Locus::default(),
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

// Locus
//

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Locus {
    pub position: Position,
    pub velocity: Vec3,
    pub direction: Direction,
    pub facing: Direction,
    pub stance: Stance,
    pub weight: f64,
}

impl Locus {
    pub fn set_pos(&mut self, pos: IVec3) {
        self.position = Position::Point(pos);
    }

    pub fn set_area(&mut self, area: Area3d) {
        self.position = Position::Area(area);
    }
}

impl Default for Locus {
    fn default() -> Self {
        Locus {
            position: Position::Point(IVec3::new(0, 0, 0)),
            velocity: Vec3::new(0., 0., 0.),
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
