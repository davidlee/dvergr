use crate::input::PlayerInputState;

use super::*;

pub(crate) fn bootstrap(
    mut ev_plan_req: EventWriter<ActionPlanRequestEvent>,
    mut next_state: ResMut<NextState<ActionSystemState>>,
    mut input_state: ResMut<NextState<PlayerInputState>>,
) {
    next_state.set(ActionSystemState::Plan);
    input_state.set(PlayerInputState::Listen);
    ev_plan_req.send(ActionPlanRequestEvent);
}
/// ??

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

// ...

pub(crate) fn check_player_plan(
    query: Query<(Entity, &Actor), With<Player>>,
    mut input_state: ResMut<NextState<PlayerInputState>>,
    mut events: EventWriter<ActionPlanRequestEvent>,
    // mut next_state: ResMut<NextState<ActionSystemState>>,
) {
    dbg!("check player plan");
    let (_, actor) = query.single();

    if actor.action.is_some() {
        input_state.set(PlayerInputState::Inactive);
        events.send(ActionPlanRequestEvent);
    } else {
        input_state.set(PlayerInputState::Listen);
    }
}

pub(crate) fn check_all_plans(
    query: Query<(Entity, &Actor, Option<&Player>)>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    warn!("check all plans");
    if query
        .iter()
        .all(|(_, actor, _)| actor.action.is_some_and(|x| x.validated))
    {
        ev_tick.send(TickEvent);
    }
}

pub(crate) fn handle_action_invalid(
    mut ev_invalid: EventReader<ActionInvalidEvent>,
    mut commands: Commands,
    mut input_state: ResMut<NextState<PlayerInputState>>,
    mut query: Query<(Entity, &mut Actor, Option<&Player>)>,
) {
    // warn!("HANDLER: handle_action_invalid");

    for ev in ev_invalid.read() {
        let (_, mut actor, is_player) = query.get_mut(ev.entity).unwrap();
        actor.reset();
        if is_player.is_some() {
            input_state.set(PlayerInputState::Listen);
        } else {
            commands.entity(ev.entity).insert(ActionPlanRequestMarker);
        }
    }
}

pub(crate) fn apply_completed_action_markers(
    mut commands: Commands,
    // mut actors: Query<(Entity, &mut Actor)>,
    mut query: Query<(Entity, &mut Actor)>,
    mut next_state: ResMut<NextState<ActionSystemState>>,
    // time: Res<TickCount>,
) {
    for (entity, mut actor) in query.iter_mut() {
        if actor.action.is_some_and(|x| x.is_success()) {
            let action = actor.action.take().unwrap();

            // insert the detail as a marker component
            // we'll look for these in queries inside on_success systems
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
            // cmds.log_components();
        }
    }
    warn!(">> animate");
    next_state.set(ActionSystemState::AwaitAnim);
}

// find the next tick in which an action completes - and advance to it
// this will end up needing to fold into tick_actions to deal with monitors, etc
// ie we won't be able to know the next interesting tick in advance.

pub(crate) fn clock_tick(mut clock: ResMut<TickCount>, query: Query<&Actor>) {
    let mut ts: Vec<u32> = vec![];

    for actor in query.iter() {
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

    dbg!("TICK TOCK! time is: {:?}", clock);
}

pub(crate) fn tick_actions(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Actor, Option<&Player>)>,
    mut ev_start: EventWriter<ActionStartEvent>,
    mut ev_complete: EventWriter<ActionCompleteEvent>,
    mut ev_abort: EventWriter<ActionAbortEvent>,
    mut ev_plan_req: EventWriter<ActionPlanRequestEvent>,
    time: Res<TickCount>,
) {
    // debounce
    let mut plan_req = false;

    for (entity, mut actor, player) in query.iter_mut() {
        let mut action = actor.action.unwrap();
        if action.is_queued() {
            action.start(time.as_u32());
            ev_start.send(ActionStartEvent {
                entity,
                at: time.as_u32(),
            });
        } else if action.is_running() {
            if action.should_complete(time.as_u32()) {
                action.status = ActionStatus::Complete;
                actor.action = Some(action);
                ev_complete.send(ActionCompleteEvent {
                    entity,
                    at: time.as_u32(),
                })
            } else {
                // BAU
                // TODO .. check interrupts, monitors, etc
            };
        } else if action.is_failed() {
            dbg!("failed action, reset queue ... ");
            actor.reset();
            commands.entity(entity).insert(ActionPlanRequestMarker);
            plan_req = true;
            ev_abort.send(ActionAbortEvent {
                entity,
                at: time.as_u32(),
            });
        } else if actor.action.is_none() {
            if let Some(mut next_action) = actor.queue.pop_front() {
                next_action.start(time.as_u32());
                actor.action = Some(next_action);
            } else if player.is_none() {
                commands.entity(entity).insert(ActionPlanRequestMarker);
                plan_req = true;
            }
        } else {
            panic!("unknown action tick scenario");
        }
    }

    if plan_req == true {
        ev_plan_req.send(ActionPlanRequestEvent);
    }
}

pub(crate) fn plan_agent_actions(
    mut actors: Query<(Entity, &mut Actor, Option<&Player>), With<ActionPlanRequestMarker>>,
    mut ev_verify: EventWriter<ActionVerifyAssignsEvent>,
    time: Res<TickCount>,
) {
    for (entity, mut actor, maybe_player) in actors.iter_mut() {
        if maybe_player.is_some() {
            panic!("don't handle this yet ... i guess its the confusion case");
        }

        if actor.action.is_none() {
            //
            // TODO something smarter
            actor.action = Some(Action {
                entity,
                status: ActionStatus::Queued,
                detail: ActionDetail::Wait,
                duration: 10,
                validated: false,
            });
        } else {
            warn!("expected empty action queue but found some");
            panic!("log etc");
        }
    }
    ev_verify.send(ActionVerifyAssignsEvent { at: time.as_u32() });
}
