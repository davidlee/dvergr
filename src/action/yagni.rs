use crate::creature::pace::*;
use crate::creature::Locus;
use crate::time::*;
use bevy::prelude::*;

pub mod verb;
pub use verb::Verb;
// Actions
//

#[derive(Event, Debug, Eq, PartialEq, Clone)]
pub struct Meta {
    agent: Entity,
    duration: Duration,
    started: Option<TurnTime>,
    completes: Option<TurnTime>,
    // on_start: Option<dyn Event>,
    // on_complete: Option<dyn Event>,
    // on_interrupt: Option<dyn Event>,
}

#[derive(Event, Debug, Eq, PartialEq, Clone)]
pub struct GenericAction {
    // meta: Meta,
    verb: Verb,
    subject: Option<Entity>,
    object: Option<Entity>,
    indirect_object: Option<Entity>,
}

#[derive(Event, Debug, PartialEq, Clone)]
pub struct MovementAction {
    // meta: Meta,
    verb: Verb,
    origin: Locus, // pace etc ignored
    destination: Locus,
}

#[derive(Component, Debug, Clone, Default)]
pub struct ActionList {
    pub current_action: Option<Command>,
    pub queued_commands: Vec<Command>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Command {
    Combat(Meta, CombatAction),
    Move(Meta, MovementAction),
    Item(Meta, ItemAction),
    Wait(Meta, Duration),
    // Sense(Meta),
    // Rest(Meta),
    // Craft(Meta),
    // Social(Meta),
    // UseAbility(Meta),
}

// #[derive(PartialEq, Eq, Debug, Clone)]
// pub enum MoveAction {
//     Turn(Direction),
//     Crawl(Direction),
//     Sneak(Direction),
//     Probe(Direction), // find traps
//     SneakilyProbe(Direction),
//     // usually
//     Walk(Direction),
//     // Jog(Direction),
//     Run(Direction),
//     Sprint(Direction),
//     // Barge(Direction), // breach door
//     Climb(Direction),

//     Prone(),
//     Kneel(),
//     Crouch(),
//     Stand(),
//     Swim(),
//     Jump(),
// }

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub enum ItemAction {
    Retrieve(Entity),
    Get(Entity),
    Wear(Entity),
    Equip(Entity),
    Unequip(Entity),
    Eat(Entity),
    Drink(Entity),
    Use(Entity),
    Drop(Entity),
    Throw(Entity),
    Examine(Entity),
    Stow(Entity, Entity),
    Fill(Entity),
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub enum CombatAction {
    Attack {
        target: Entity, // creature or cell
        weapon: Entity, // weapon
        verb: Verb,     // manoeuvre
        tempo: Option<Tempo>,
    },
    MissileAttack {
        target: Entity,
        weapon: Entity,       // weapon
        ammo: Option<Entity>, // ammo
        verb: Verb,           // manoeuvre
        tempo: Option<Tempo>,
    },
    Defend {
        verb: Verb, // manoeuvre
        tempo: Option<Tempo>,
    },
}
