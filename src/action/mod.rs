// use crate::creature::pace::*;
// use crate::creature::Locus;

use bevy::prelude::*;
use std::any::Any;
use std::collections::VecDeque;
use std::fmt::Debug;

use crate::time::*;
use crate::Locus;
use crate::Player;

pub use verb::Verb;

pub mod verb;

// Plugin
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            // .add_event::<ActionCompleteEvent>()
            // .add_event::<ActionAbortEvent>()
            // .add_event::<ActionRequiredEvent>()
            .add_event::<ActorQueueEmptyEvent>()
            // .add_event::<InvalidPlayerActionEvent>()
            .add_systems(Update, process_action_queue.run_if(on_event::<TickEvent>()));
    }
}

pub fn process_action_queue(world: &mut World) {
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else {
        return;
    };
    let Some(entity) = queue.0.pop_front() else {
        world.send_event(ActorQueueEmptyEvent);
        return;
    };
    let Some(mut actor) = world.get_mut::<Actor>(entity) else {
        return;
    };
    let Some(action) = actor.current_action.take() else {
        return;
    };

    let is_player = world.get::<Player>(entity).is_some();

    // match action.execute(world) {
    //     ActionState::Planned => panic!("Action should only be Planned until executed"),
    //     ActionState::Started => (),
    //     ActionState::Complete => {
    //         actor.current_action.take();
    //     } // send event
    //     ActionState::Invalid => {
    //         if world.get::<Player>(entity).is_some() {
    //             world.send_event(InvalidPlayerActionEvent);
    //             return;
    //         }
    //     } //
    //     ActionState::Aborted => {
    //         actor.current_action.take();
    //     } // send event
    // }

    // actor.tick(world);

    // if actor.current_action.is_some() {
    //     let action = actor.current_action.unwrap();
    //     let result_state = action.as_ref().execute(world);

    //     match result_state {
    //     }
    //     world.send_event(NextActorEvent);
    // };
}

// #[derive(Default, Debug)]
// pub enum ActionState {
//     None,
//     #[default]
//     Planned,
//     Started,
//     Complete,
//     Invalid,
//     Aborted,
// }

// Trait
pub trait Action: Send + Sync + Debug {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()>;
    fn as_any(&self) -> &dyn Any; // { self }
}

pub trait Strategy: Send + Sync + Debug {
    fn scheme(&self, world: &mut World) -> dyn Action;
}

#[derive(Component, Default, Debug)]
pub struct Actor {
    pub current_action: Option<Box<dyn Action>>,
    pub queue: VecDeque<Box<dyn Action>>,
    pub strategy: Option<Box<dyn Strategy>>,
}

impl Actor {
    // pub fn tick(&mut self, world: &mut World) {
    //     if self.current_action.is_some() {
    //         match self.current_action.as_mut().unwrap().execute(world) {
    //             ActionState::Planned => panic!("Action should only be Planned until executed"),
    //             ActionState::Started => (),
    //             ActionState::Complete => {
    //                 self.current_action = self.queue.pop_front();
    //             }
    //             ActionState::Invalid | ActionState::Aborted | ActionState::None => {
    //                 self.current_action = None;
    //             }
    //         }
    //     }
    // }
}

#[derive(Default, Resource, Debug)]
pub struct ActorQueue(pub VecDeque<Entity>);

// Events
#[derive(Event, Debug, Clone)]
pub struct TickEvent;

#[derive(Event, Debug, Clone)]
pub struct NextActorEvent;

// #[derive(Event, Debug)]
// pub struct ActionCompleteEvent;
// #[derive(Event, Debug, Clone)]
// pub struct ActionAbortEvent;
// #[derive(Event, Debug, Clone)]
// pub struct ActionRequiredEvent;

#[derive(Event, Debug, Clone)]
pub struct InvalidPlayerActionEvent;

#[derive(Event, Debug, Clone)]
pub struct ActorQueueEmptyEvent;

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
