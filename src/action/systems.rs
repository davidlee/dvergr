use super::*;

#[derive(Event, Debug)]
pub(crate) struct StillWaitForAnimEvent;

pub(crate) fn skip_player_input_if_action_exists(
    query: Query<&Player>,
    mut next_state: ResMut<NextState<TickState>>,
) {
    let player = query.get_single().unwrap();
    if player.action.is_some() {
        info!("no need for player input, have action. go to ClockTick");
        // dbg!(player);
        next_state.set(TickState::ClockTick);
    }
}

pub(crate) fn dead_letters(
    mut letters: EventReader<StillWaitForAnimEvent>,
    mut next_state: ResMut<NextState<TickState>>,
) {
    // warn!("dead letters");
    let mut finished = true;
    for _ in letters.read() {
        finished = false;
    }
    if finished {
        warn!(">> Loop Over ...");
        next_state.set(TickState::PlayerInput);
    }
}

pub(crate) fn apply_completed_action_markers(
    mut commands: Commands,
    // mut actors: Query<(Entity, &mut Actor)>,
    mut get_player: Query<(Entity, &mut Player)>,
    mut next_state: ResMut<NextState<TickState>>,
    // time: Res<TickCount>,
) {
    // warn!("apply completed action markers:");
    // Player action complete?
    let (entity, mut player) = get_player.single_mut();
    // warn!("player :: {:?}", player);

    if player.action.is_some_and(|x| x.is_success()) {
        // warn!("completed ###");

        // remove it
        let action = player.action.take().unwrap();

        // insert the detail as a marker component
        // we'll look for these in the on_success systems

        let mut cmds = commands.entity(entity);
        match action.detail {
            ActionDetail::Move(x) => {
                cmds.insert(x);
            }
            ActionDetail::Inventory(x) => {
                cmds.insert(x);
            }
            ActionDetail::Attack(x) => {
                cmds.insert(x);
            }
            ActionDetail::Shoot(x) => {
                cmds.insert(x);
            }
            ActionDetail::Wait => {} // noop
        }

        cmds.log_components();
    }

    // for (e, mut actor) in actors.iter_mut() {
    //     if actor.action.is_some_and(|x| x.is_success()) {
    //         let action = actor.action.take().unwrap();
    //         // TODO / fixme
    //         // commands.entity(e).insert(action.detail).log_components();
    //     }
    // }

    warn!(">> animate");
    next_state.set(TickState::Animate)
}

// find the next "interesting" clock tick
// for now, that is the next tick in which an action completes -
// then advance to it

pub(crate) fn clock_tick(
    mut clock: ResMut<TickCount>,
    qp: Query<&Player>,
    qa: Query<&Actor>,
    mut next_state: ResMut<NextState<TickState>>,
) {
    let mut ts: Vec<u32> = vec![];

    let player = qp.get_single().unwrap();
    if player.action.is_some() {
        if let Some(t) = player.action.unwrap().ticks_left() {
            ts.push(t);
        }
    }

    for actor in qa.iter() {
        if actor.action.is_some() {
            if let Some(t) = actor.action.unwrap().ticks_left() {
                ts.push(t);
            }
        }
    }

    if ts.is_empty() {
        clock.tick();
    } else {
        ts.sort();
        clock.advance(ts[0]);
    }
    dbg!("Time is: {:?}", clock);

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
            warn!("SHOULD COMPLETE");
            action.status = ActionStatus::Complete;
        } else {
            // TODO validate / handle interrupts / etc
        };

        player.action = Some(action);
        dbg!("tick_player_action:", &player.action);
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
