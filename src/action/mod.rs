use crate::board::direction::Direction;
use crate::state::TickState;
use crate::time::*;
use crate::Player;
use bevy::prelude::*;
use std::collections::VecDeque;
use std::fmt::Debug;

pub(crate) mod on_success;
pub(crate) mod systems;
pub(crate) use systems::*;

#[derive(Component, Default, Debug)]
pub(crate) struct Actor {
    pub action: Option<Action>,
    pub queue: VecDeque<Action>,
}

#[derive(Event, Debug, PartialEq, Clone, Copy)]
pub(crate) struct Action {
    pub(crate) entity: Entity,
    // player: bool,
    pub(crate) status: ActionStatus,
    pub(crate) detail: ActionDetail,
    pub(crate) duration: u32, // ticks
}

impl Action {
    fn ticks_left(&self) -> Option<u32> {
        if let ActionStatus::Active {
            start_tick: _,
            complete_tick,
        } = self.status
        {
            Some(complete_tick)
        } else {
            None
        }
    }

    fn start(&mut self, current_tick: u32) {
        let start_tick = current_tick;
        let complete_tick = start_tick + self.duration;
        self.status = ActionStatus::Active {
            start_tick,
            complete_tick,
        };
    }

    fn is_running(&self) -> bool {
        if let ActionStatus::Active {
            start_tick: _,
            complete_tick: _,
        } = self.status
        {
            true
        } else {
            false
        }
    }

    fn is_queued(&self) -> bool {
        self.status == ActionStatus::Queued
    }

    // FIXME success? complete?
    fn is_success(&self) -> bool {
        self.status == ActionStatus::Complete
    }

    fn is_failed(&self) -> bool {
        match self.status {
            ActionStatus::Invalid | ActionStatus::Aborted => true,
            _ => false,
        }
    }

    fn should_complete(&self, current_tick: u32) -> bool {
        if let ActionStatus::Active {
            start_tick: _,
            complete_tick,
        } = self.status
        {
            complete_tick <= current_tick
        } else {
            false
        }
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
pub(crate) enum ActionStatus {
    #[default]
    Queued,
    // Validated,
    Active {
        start_tick: u32,
        complete_tick: u32,
    },
    Complete,
    Invalid,
    Aborted,
}

#[derive(Event, Debug, Clone)]
pub(crate) struct PlayerActionInvalidEvent;

#[derive(Component, Debug)]
pub(crate) struct ActorQueueEmptyMarker;

// details

#[allow(dead_code)]
#[derive(Event, Debug, PartialEq, Clone, Copy, Component)]
pub(crate) enum ActionDetail {
    Move(MovementActionDetail),
    Inventory(InventoryActionDetail),
    Attack(MeleeCombatActionDetail),
    Shoot(MissileCombatActionDetail),
    Wait,
    // General(Meta, Verb, GeneralAction),
}

#[derive(Event, Debug, PartialEq, Clone, Copy, Component)]
#[allow(dead_code)]
pub(crate) enum MovementActionDetail {
    Turn(Direction),
    Walk(Direction),
    Run(Direction),
}

impl MovementActionDetail {
    pub(crate) fn direction(&self) -> &Direction {
        match self {
            MovementActionDetail::Turn(dir)
            | MovementActionDetail::Walk(dir)
            | MovementActionDetail::Run(dir) => dir,
        }
    }
}

#[derive(Event, Debug, Eq, PartialEq, Clone, Copy, Component)]
pub(crate) struct InventoryActionDetail {
    // verb: Verb,
    subject: Option<Entity>,
    object: Option<Entity>,
    indirect_object: Option<Entity>,
}

#[derive(Event, Debug, Eq, PartialEq, Clone, Copy, Component)]
pub(crate) struct MeleeCombatActionDetail {
    // verb: Verb,
    subject: Option<Entity>,
    object: Option<Entity>,
    indirect_object: Option<Entity>,
}

#[derive(Event, Debug, Eq, PartialEq, Clone, Copy, Component)]
pub(crate) struct MissileCombatActionDetail {
    // verb: Verb,
    subject: Option<Entity>,
    object: Option<Entity>,
    indirect_object: Option<Entity>,
}
