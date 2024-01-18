use super::*;

#[derive(Event, Debug)]
pub(crate) struct StillWaitForAnimEvent;

pub(crate) fn dead_letters(
    mut letters: EventReader<StillWaitForAnimEvent>,
    mut next_state: ResMut<NextState<TickState>>,
) {
    warn!("dead letters");
    let mut finished = true;
    for _ in letters.read() {
        finished = false;
    }
    if finished {
        warn!(">> ValidatePlayerAction");
        next_state.set(TickState::ValidatePlayerAction);
    }
}

pub(crate) fn apply_completed_action_markers(
    mut commands: Commands,
    mut actors: Query<(Entity, &mut Actor)>,
    mut get_player: Query<(Entity, &mut Player)>,
    mut next_state: ResMut<NextState<TickState>>,
    // time: Res<TickCount>,
) {
    warn!("apply completed, any?");
    // Player action complete?
    let (entity, mut player) = get_player.single_mut();
    warn!("player :: {:?}", player);
    if player.action.is_some_and(|x| x.is_success()) {
        warn!("completed ###");
        // remove the action
        let action = player.action.take().unwrap();
        // insert the detail as a marker component
        // which will trigger the effects
        commands.entity(entity).insert(action.detail);
    }

    for (e, mut actor) in actors.iter_mut() {
        if actor.action.is_some_and(|x| x.is_success()) {
            let action = actor.action.take().unwrap();
            commands.entity(e).insert(action.detail);
        }
    }

    warn!(">> animate");
    next_state.set(TickState::Animate)
}

pub(crate) fn clock_tick(
    mut clock: ResMut<TickCount>,
    mut next_state: ResMut<NextState<TickState>>,
) {
    clock.tick();
    warn!("time is now {:?}", clock);
    next_state.set(TickState::PlayerActionTick);
}

pub(crate) fn tick_player_action(
    // mut commands: Commands,
    mut get_player: Query<(Entity, &mut Player)>,
    mut next_state: ResMut<NextState<TickState>>,
    time: Res<TickCount>,
) {
    let (_entity, mut player) = get_player.single_mut();

    if let Some(mut action) = player.action {
        info!("player action ::({:?}), ", action);

        if action.is_queued() {
            action.start(time.as_u32());
        }

        if !action.is_running() {
            panic!("action not running");
        }

        if action.should_complete(time.as_u32()) {
            action.status = ActionStatus::Complete;
        } else {
            // TODO validate / handle interrupts / etc
        };

        player.action = Some(action);
        next_state.set(TickState::AgentActionsTick);
    } else {
        panic!("missing action");
    }
}

pub(crate) fn tick_agent_actions(
    mut commands: Commands,
    mut actors: Query<(Entity, &mut Actor)>,
    mut next_state: ResMut<NextState<TickState>>,
    time: Res<TickCount>,
) {
    // NOTE processing actors in unspecified order

    info!(">> tick_agent_actions");

    for (entity, mut actor) in actors.iter_mut() {
        if let Some(mut action) = &actor.action {
            if action.is_queued() {
                action.start(time.as_u32());
            }

            if action.is_running() {
                if action.should_complete(time.as_u32()) {
                    action.status = ActionStatus::Complete;

                    // commands.add(|world: &mut World| {
                    //     world.send_event(a);
                    // });
                } else {
                    // TODO check for invalid conditions, interrupt, etc
                }
            } else if action.is_failed() {
                warn!("action not running: {:?}", action);
                actor.action = None;
                actor.queue = VecDeque::new();
            } else {
                panic!("what state dis? {:?}", actor);
            }
        }

        if actor.action.is_none() {
            if let Some(mut next_action) = actor.queue.pop_front() {
                next_action.start(time.as_u32());
                actor.action = Some(next_action);
            } else {
                commands.entity(entity).insert(ActorQueueEmptyMarker);
            }
        }
    }

    warn!(">> apply completed actions");
    next_state.set(TickState::ApplyCompletedActions);
}

// planner
pub(crate) fn prepare_agent_actions(
    mut actors: Query<(Entity, &mut Actor)>,
    mut next_state: ResMut<NextState<TickState>>,
) {
    warn!("prepare_agent_actions");

    for (entity, mut actor) in actors.iter_mut() {
        if let Some(current_action) = &actor.action {
            info!("agent action exists {:?}", current_action);
            // match current_action.status {
            // }
        } else {
            actor.action = Some(Action {
                entity,
                status: ActionStatus::Queued,
                detail: ActionDetail::Wait,
                duration: 10,
            });
        }
    }

    warn!("next state ..");
    next_state.set(TickState::ClockTick);
}
