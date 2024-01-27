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

pub(crate) fn plan_init_check_or_tick(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Actor, Option<&ActorAction>), With<Player>>,
    idle_actors: Query<(Entity, &Actor), (Without<Player>, Without<ActorAction>)>,
    actors: Query<(Entity, &Actor, &ActorAction), Without<Player>>,
    mut input_state: ResMut<NextState<PlayerInputState>>,
    mut ev_planner: EventWriter<ActionPlanRequestEvent>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    info!(">> INIT/CHECK/TICK");

    let mut ready = false;

    if let (entity, mut actor, Some(action)) = player.single_mut() {
        info!("player has action ({:?})", &action.0);

        match action.0.status {
            ActionStatus::Ready | ActionStatus::Active { .. } => {
                ready = true;
                input_state.set(PlayerInputState::Inactive);
            }
            ActionStatus::Complete => {
                warn!("ignoring: needs to apply completed action markers");
            }
            ActionStatus::Idle => panic!("needs validation"),
            ActionStatus::Aborted => {
                // remove & clean up. If we send an event, it'll trigger this function again
                warn!("removing aborted command & clearing queue");
                commands.entity(entity).remove::<ActorAction>();
                actor.clear_queue();

                // then prepare for player input
                input_state.set(PlayerInputState::Listen);
            }
        }
    } else {
        input_state.set(PlayerInputState::Listen);
    }

    if idle_actors.iter().count() > 0 {
        dbg!("idle actors ..");
        // state.set(ActionSystemState::Plan);
        // TODO insert markers for them
        ev_planner.send(ActionPlanRequestEvent);
    } else if actors.iter().all(|x| x.2 .0.is_runnable()) {
        if ready {
            ev_tick.send(TickEvent);
            dbg!("TICK");
        } else {
            dbg!("agents ready, waiting on player");
            // TODO something
        }
    }
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
    warn!("APPLY COMPLETED");
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
            commands.entity(entity).remove::<ActorAction>();
        } else {
            // anything else to clean up?
            dbg!("....", action.0);
        }
    }
    warn!("anim");
    next_state.set(ActionSystemState::AwaitAnim);
}

// find the next tick in which an action completes - and advance to it
// this makes everything feel a lot more responsive ...
// but I'd love not to need it because it might not be workable when eg.
// actions require monitor validations each tick

#[allow(dead_code)]
pub(crate) fn clock_tick_eager(
    mut clock: ResMut<TickCount>,
    frame: Res<FrameCount>,
    query: Query<(&Actor, &ActorAction)>,
) {
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

    warn!(
        "TICK TOCK! time is: {:?} at frame count: {:?}",
        clock.0, frame.0,
    );
}

// just advance by 1 tick
pub(crate) fn clock_tick(mut clock: ResMut<TickCount>, frame: Res<FrameCount>) {
    clock.tick();
    warn!(
        "TICK! currently # {:?} at frame count: {:?}",
        clock.0, frame.0,
    );
}

pub(crate) fn tick_actions(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Actor, &mut ActorAction, Option<&Player>)>,
    mut ev_start: EventWriter<ActionStartedEvent>,
    mut ev_complete: EventWriter<ActionCompleteEvent>,
    mut ev_abort: EventWriter<ActionAbortedEvent>,
    mut ev_input: EventWriter<PlayerInputRequestEvent>,
    time: Res<TickCount>,
) {
    // dbg!("tick actions");

    for (entity, mut actor, mut a_action, player) in query.iter_mut() {
        let a = a_action.0;
        if a.is_ready() {
            dbg!("starting:", &a.status);
            a_action.0.start(time.0);
            // warn!(
            //     "started: {:?} {:?}",
            //     &a_action.0.status,
            //     &a_action.0.ticks_remaining(time.0)
            // );

            // commands.insert()
            ev_start.send(ActionStartedEvent { entity });
        } else if a.is_active() {
            dbg!("status: active", &a.status);
            // warn!("remaining: {:?}", &a.ticks_remaining(time.0));
            if a.should_complete(time.0) {
                // warn!("COMPLETED");
                a_action.0.status = ActionStatus::Complete;
                ev_complete.send(ActionCompleteEvent { entity });

                if let Some(mut next_action) = actor.queue.pop_front() {
                    next_action.start(time.0);
                    commands.entity(entity).insert(ActorAction(next_action));
                } else if player.is_some() {
                    ev_input.send(PlayerInputRequestEvent);
                    info!("player has no queued action to make active");
                } else {
                    commands.entity(entity).insert(ActionPlanRequestMarker);
                }
            } else {
                // dbg!("not complete yet at ", time.0);
            }
        } else if a.is_aborted() {
            warn!("!!!! failed action, reset queue ... ");
            commands.entity(entity).remove::<ActorAction>();
            actor.clear_queue();
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
            dbg!("in plan agent actions: this is a player, NOOP");
            continue;
        }

        commands.entity(entity).insert(ActorAction(Action {
            entity,
            status: ActionStatus::Ready, //no validation reqd
            detail: ActionDetail::Wait,
            duration: 10,
        }));

        dbg!("added a wait action in planner");
        ev_added.send(ActionAddedEvent { entity });
    }
}
