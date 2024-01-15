use crate::creature::pace::*;
use crate::creature::Locus;
use crate::time::*;
use bevy::prelude::*;

pub mod verb;
pub use verb::Verb;
// Actions
//

#[derive(Component, Debug, Clone, Default)]
pub struct ActionList {
    pub current_action: Option<Command>,
    pub queued_commands: Vec<Command>,
}

#[derive(Event, Debug, Eq, PartialEq, Clone)]
pub struct Meta {
    agent: Entity,
    duration: Duration,
    started: Option<TurnTime>,
    completes: Option<TurnTime>,
}

#[derive(Event, Debug, PartialEq, Clone)]
pub enum Command {
    Move(Meta, Verb, MovementCommand),
    Inventory(Meta, Verb, InventoryCommand),
    Attack(Meta, Verb, MeleeCombatCommand),
    Shoot(Meta, Verb, MissileCombatCommand),
    Wait(Meta, Verb, Duration),
    General(Meta, Verb, GeneralCommand),
}

#[derive(Event, Debug, PartialEq, Clone)]
pub struct MovementCommand {
    // verb: Verb,
    origin: Locus, // pace etc ignored
    destination: Locus,
}

#[derive(Event, Debug, Eq, PartialEq, Clone)]
pub struct GeneralCommand {
    // verb: Verb,
    subject: Option<Entity>,
    object: Option<Entity>,
    indirect_object: Option<Entity>,
}

#[derive(Event, Debug, Eq, PartialEq, Clone)]
pub struct InventoryCommand {
    // verb: Verb,
    subject: Option<Entity>,
    object: Option<Entity>,
    indirect_object: Option<Entity>,
}

#[derive(Event, Debug, Eq, PartialEq, Clone)]
pub struct MeleeCombatCommand {
    // verb: Verb,
    subject: Option<Entity>,
    object: Option<Entity>,
    indirect_object: Option<Entity>,
}

#[derive(Event, Debug, Eq, PartialEq, Clone)]
pub struct MissileCombatCommand {
    // verb: Verb,
    subject: Option<Entity>,
    object: Option<Entity>,
    indirect_object: Option<Entity>,
}
