// use crate::creature::pace::*;
// use crate::creature::Locus;

use bevy::prelude::*;
use std::collections::VecDeque;
use std::fmt::Debug;

use crate::time::*;

pub mod verb;

// Plugin
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TickEvent>().add_systems(
            Update,
            process_current_actions.run_if(on_event::<TickEvent>()),
        );
    }
}

#[derive(Component, Default, Debug)]
pub(crate) struct Actor {
    pub action: Option<Action>,
    pub queue: VecDeque<Action>,
}

#[derive(Component, Debug)]
pub(crate) struct ActorQueueEmptyMarker;

fn process_current_actions(
    mut commands: Commands,
    mut actors_q: Query<&mut Actor>,
    time: Res<TickCount>,
) {
    // TODO currently we're iterating over actors in unspecified order
    // we may want to order by initiative, etc

    for mut actor in actors_q.iter_mut() {
        let mut entity: Option<Entity> = None;
        if let Some(current_action) = &actor.action {
            entity = Some(current_action.entity);
            match current_action.status {
                ActionStatus::Queued | ActionStatus::Complete => panic!("invalid state"),
                ActionStatus::Active {
                    start_tick: _,
                    complete_tick,
                } => {
                    if time.as_u32() >= complete_tick {
                        let mut action = actor.action.take().unwrap();
                        action.status = ActionStatus::Complete;

                        // TODO different envelope?
                        commands.add(|world: &mut World| {
                            world.send_event(action);
                        });
                    } else {
                        // TODO validate it
                    }
                }
                ActionStatus::Invalid | ActionStatus::Aborted => {
                    actor.action.take();
                    actor.queue = VecDeque::new();
                    // TODO send event
                }
            }
        }
        let entity = entity;

        if actor.action.is_none() {
            if let Some(mut next_action) = actor.queue.pop_front() {
                next_action.start(time.as_u32());
                actor.action = Some(next_action);
            } else if entity.is_some() {
                let entity = entity.unwrap();
                commands.entity(entity).insert(ActorQueueEmptyMarker);
            } else {
                // panic?
            }
        }
    }
}

// fn process_event(mut events: EventReader<Action>) {}

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
#[allow(dead_code)]
pub(crate) enum TickState {
    #[default]
    AwaitPlayerInput,
    PlayerAction,
    ActorActions,
    AdvanceTime,
    Animate,
}

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
#[allow(dead_code)]
pub(crate) enum ActionStatus {
    #[default]
    Queued,
    Active {
        start_tick: u32,
        complete_tick: u32,
    },
    Complete,
    Invalid,
    Aborted,
}

// Events
#[derive(Event, Debug, Clone)]
pub(crate) struct TickEvent;

#[derive(Event, Debug, PartialEq, Clone)]
pub(crate) struct Action {
    entity: Entity,
    status: ActionStatus,
    detail: ActionDetail,
    duration: u32, // ticks
}

impl Action {
    fn start(&mut self, current_tick: u32) {
        let start_tick = current_tick;
        let complete_tick = start_tick + self.duration;
        self.status = ActionStatus::Active {
            start_tick,
            complete_tick,
        };
    }
}

#[derive(Event, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub(crate) enum ActionDetail {
    Move(MovementActionDetail),
    Inventory(InventoryActionDetail),
    Attack(MeleeCombatActionDetail),
    Shoot(MissileCombatActionDetail),
    Wait(Duration),
    // General(Meta, Verb, GeneralAction),
}

#[derive(Event, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub(crate) enum MovementActionDetail {
    // Crouch,
    // Prone,
    // Stand,
    // Special(Direction, Vec<Flags>) - sneak, find traps, etc
    Turn(Direction),
    Walk(Direction),
    Run(Direction),
    // Sprint(Direction),
    // Climb(Direction),
}

// #[derive(Event, Debug, Eq, PartialEq, Clone)]
// pub struct GeneralAction {
//     // verb: Verb,
//     subject: Option<Entity>,
//     object: Option<Entity>,
//     indirect_object: Option<Entity>,
// }

#[derive(Event, Debug, Eq, PartialEq, Clone)]
pub(crate) struct InventoryActionDetail {
    // verb: Verb,
    subject: Option<Entity>,
    object: Option<Entity>,
    indirect_object: Option<Entity>,
}

#[derive(Event, Debug, Eq, PartialEq, Clone)]
pub(crate) struct MeleeCombatActionDetail {
    // verb: Verb,
    subject: Option<Entity>,
    object: Option<Entity>,
    indirect_object: Option<Entity>,
}

#[derive(Event, Debug, Eq, PartialEq, Clone)]
pub(crate) struct MissileCombatActionDetail {
    // verb: Verb,
    subject: Option<Entity>,
    object: Option<Entity>,
    indirect_object: Option<Entity>,
}
