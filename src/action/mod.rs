use crate::typical::*;

pub(crate) mod on_success;
pub(crate) mod systems;
pub(crate) mod validation;

pub(crate) use systems::*;
// pub(crate) use validation::*;

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, States)]
pub(crate) enum ActionSystemState {
    #[default]
    Plan,
    Run,
    AwaitAnim,
}

#[derive(Component, Debug)]
pub(crate) struct ActionPlanRequestMarker;

#[allow(dead_code)]
pub(crate) mod events {
    use super::*;

    #[derive(Event, Debug)]
    pub(crate) struct TickEvent;

    #[derive(Event, Debug)]
    pub(crate) struct ActionPlanRequestEvent;

    #[derive(Event, Debug, Clone)]
    pub(crate) struct ActionInvalidatedEvent {
        pub(crate) entity: Entity,
    }

    #[derive(Event, Debug, Clone)]
    pub(crate) struct ActionValidatedEvent {
        pub(crate) entity: Entity,
    }

    #[derive(Event, Debug, Clone)]
    pub(crate) struct ActionCompleteEvent {
        pub(crate) entity: Entity,
    }

    #[derive(Event, Debug, Clone)]
    pub(crate) struct ActionAddedEvent {
        pub(crate) entity: Entity,
    }

    #[derive(Event, Debug, Clone)]
    pub(crate) struct ActionStartedEvent {
        pub(crate) entity: Entity,
    }

    #[derive(Event, Debug, Clone)]
    pub(crate) struct ActionAbortedEvent {
        pub(crate) entity: Entity,
    }

    #[derive(Event, Debug)]
    pub(crate) struct StillWaitForAnimEvent;
}

#[derive(Component, Default, Debug)]
pub(crate) struct Actor {
    pub queue: VecDeque<Action>,
}

impl Actor {
    pub(crate) fn clear_queue(&mut self) {
        self.queue = VecDeque::new();
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub(crate) struct ActorAction(pub Action);

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Action {
    pub(crate) entity: Entity,
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

    fn is_idle(&self) -> bool {
        self.status == ActionStatus::Idle
    }

    fn is_ready(&self) -> bool {
        self.status == ActionStatus::Ready
    }

    fn is_runnable(&self) -> bool {
        matches!(
            self.status,
            ActionStatus::Ready | ActionStatus::Active { .. }
        )
    }

    fn is_active(&self) -> bool {
        matches!(self.status, ActionStatus::Active { .. })
    }

    fn is_complete(&self) -> bool {
        self.status == ActionStatus::Complete
    }

    fn is_aborted(&self) -> bool {
        self.status == ActionStatus::Aborted
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
    Idle, // not validated
    Ready, // preconditions satisfied
    Active {
        start_tick: u32,
        complete_tick: u32,
    },
    Complete,
    Aborted,
}

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
    Turn(Dir),
    Walk(Dir),
    Run(Dir),
}

impl MovementActionDetail {
    pub(crate) fn direction(&self) -> &Dir {
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
