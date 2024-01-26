use crate::input::PlayerInputState;

use super::*;

// delete?
pub(crate) fn bootstrap(
    mut ev_plan_req: EventWriter<ActionPlanRequestEvent>,
    mut next_state: ResMut<NextState<ActionSystemState>>,
    mut input_state: ResMut<NextState<PlayerInputState>>,
) {
    next_state.set(ActionSystemState::Plan);
    input_state.set(PlayerInputState::Listen);
    ev_plan_req.send(ActionPlanRequestEvent);
}

//
pub(crate) fn init_or_check_plan(
    player: Query<(Entity, &Actor, Option<&ActorAction>), With<Player>>,
    idle_actors: Query<(Entity, &Actor), (Without<Player>, Without<ActorAction>)>,
    actors: Query<(Entity, &Actor, &ActorAction), Without<Player>>,
    mut input_state: ResMut<NextState<PlayerInputState>>,
    mut ev_planner: EventWriter<ActionPlanRequestEvent>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    let mut ready = false;

    if let (_, _, Some(action)) = player.single() {
        dbg!("init, player action ({:?})", action.0);

        input_state.set(PlayerInputState::Inactive);
        if action.0.is_ready() {
            ready = true;
        } else if action.0.is_idle() {
            dbg!("idle, ", action);
            // let's make validation happen
        }
    } else {
        input_state.set(PlayerInputState::Listen);
    }

    if idle_actors.iter().count() > 0 {
        dbg!("idle actors ..");
        ev_planner.send(ActionPlanRequestEvent);
    } else if ready && actors.iter().all(|x| x.2 .0.is_runnable()) {
        ev_tick.send(TickEvent);
        dbg!("TICK");
    } // else fix it
}

pub(crate) fn set_state_plan(
    mut next_state: ResMut<NextState<ActionSystemState>>,
    mut input_state: ResMut<NextState<PlayerInputState>>,
) {
    next_state.set(ActionSystemState::Plan);
    input_state.set(PlayerInputState::Listen);
}

pub(crate) fn set_state_run(
    mut next_state: ResMut<NextState<ActionSystemState>>,
    mut input_state: ResMut<NextState<PlayerInputState>>,
) {
    next_state.set(ActionSystemState::Run);
    input_state.set(PlayerInputState::Inactive);
}
pub(crate) fn set_state_await_anim(
    mut next_state: ResMut<NextState<ActionSystemState>>,
    mut input_state: ResMut<NextState<PlayerInputState>>,
) {
    next_state.set(ActionSystemState::AwaitAnim);
    input_state.set(PlayerInputState::Listen);
}

// pub(crate) fn tick_if_conditions_met(
//     query: Query<(Entity, &Actor, Option<&Player>)>,
//     mut ev_tick: EventWriter<TickEvent>,
// ) {
//     warn!("checking if we can tick");
//     //
//     // check if any actions are missing
//     // then if any are not valid
//     //

//     // FIXME

//     if query
//         .iter()
//         .all(|(_, actor, _)| actor.action.is_some_and(|x| x.validated))
//     {
//         info!("seems legit, sending Tick event");
//         ev_tick.send(TickEvent);
//     } else {
//         warn!("nah, let's see");
//         for (_, actor, is_player) in query.iter() {
//             info!(
//                 "?? -- {:?},{:?}, {:?}",
//                 actor,
//                 actor.action.is_some(),
//                 is_player
//             );
//         }
//     }
// }

pub(crate) fn handle_action_invalid(
    mut ev_invalid: EventReader<ActionInvalidatedEvent>,
    mut commands: Commands,
    mut input_state: ResMut<NextState<PlayerInputState>>,
    mut query: Query<(Entity, &mut Actor, Option<&Player>)>,
) {
    // warn!("HANDLER: handle_action_invalid");

    for ev in ev_invalid.read() {
        let (_, mut actor, is_player) = query.get_mut(ev.entity).unwrap();
        actor.clear_queue();
        if is_player.is_some() {
            input_state.set(PlayerInputState::Listen);
        } else {
            commands.entity(ev.entity).insert(ActionPlanRequestMarker);
        }
    }
}

pub(crate) fn apply_completed_action_markers(
    mut commands: Commands,
    query: Query<(Entity, &Actor, &ActorAction)>,
    mut next_state: ResMut<NextState<ActionSystemState>>,
) {
    for (entity, _actor, action) in query.iter() {
        if action.0.is_complete() {
            // insert the detail as a marker component
            // we'll look for these in queries inside on_success systems
            let mut cmds = commands.entity(entity);
            match action.0.detail {
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
        } else {
            // anything else to clean up?
            dbg!("....", action.0);
        }
    }
    next_state.set(ActionSystemState::AwaitAnim);
}

// find the next tick in which an action completes - and advance to it
// this will end up needing to fold into tick_actions to deal with monitors, etc
// ie we won't be able to know the next interesting tick in advance.

pub(crate) fn clock_tick(mut clock: ResMut<TickCount>, query: Query<(&Actor, &ActorAction)>) {
    let mut ts: Vec<u32> = vec![];

    for (_, action) in query.iter() {
        if let Some(t) = action.0.ticks_left() {
            ts.push(t);
        }
    }

    if ts.is_empty() {
        clock.tick();
    } else {
        ts.sort();
        clock.advance(ts[0]);
    }

    dbg!("TICK TOCK! time is: {:?}", clock);
}

pub(crate) fn tick_actions(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Actor, &ActorAction, Option<&Player>)>,
    mut ev_start: EventWriter<ActionStartedEvent>,
    mut ev_complete: EventWriter<ActionCompleteEvent>,
    mut ev_abort: EventWriter<ActionAbortedEvent>,
    // ut ev_plan_req: EventWriter<ActionPlanRequestEvent>,
    time: Res<TickCount>,
) {
    // debounce
    dbg!("tick actions");

    for (entity, mut actor, action, player) in query.iter_mut() {
        let mut action = action.0;
        if action.is_ready() {
            dbg!("starting:", action);
            action.start(time.as_u32());
            ev_start.send(ActionStartedEvent { entity });
        } else if action.is_active() {
            dbg!("active", action);

            if action.should_complete(time.as_u32()) {
                action.status = ActionStatus::Complete;
                ev_complete.send(ActionCompleteEvent { entity });

                if let Some(mut next_action) = actor.queue.pop_front() {
                    next_action.start(time.as_u32());
                    commands.entity(entity).insert(ActorAction(next_action));
                } else if player.is_none() {
                    commands.entity(entity).insert(ActionPlanRequestMarker);
                } else {
                    // TODO set input state?
                }
            }
        } else if action.is_aborted() {
            warn!("failed action, reset queue ... ");
            actor.clear_queue();
            commands.entity(entity).remove::<ActorAction>();
            ev_abort.send(ActionAbortedEvent { entity });
        }
    }
}

pub(crate) fn plan_agent_actions(
    mut actors: Query<(Entity, &mut Actor, Option<&Player>), Without<ActorAction>>,
    mut ev_added: EventWriter<ActionAddedEvent>,
    mut commands: Commands,
) {
    for (entity, _actor, maybe_player) in actors.iter_mut() {
        if maybe_player.is_some() {
            dbg!("don't handle this case");
            continue;
        }

        commands.entity(entity).insert(ActorAction(Action {
            entity,
            status: ActionStatus::Idle,
            detail: ActionDetail::Wait,
            duration: 10,
        }));

        ev_added.send(ActionAddedEvent { entity });
    }
}
