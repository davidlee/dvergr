use crate::action::{
    events::*, Action, ActionDetail, ActionPlanRequestMarker, ActionStatus, Actor,
    MovementActionDetail,
};

use crate::typical::*;

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, States)]
pub(crate) enum PlayerInputState {
    #[default]
    Listen = 1,
    Inactive = 0,
}

pub(crate) fn keybindings(
    mut get_player: Query<(Entity, &Player, &mut Actor)>,
    // mut next_state: ResMut<NextState<TickState>>,
    keys: Res<Input<KeyCode>>,
) {
    let shifted: bool = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    let direction = if keys.just_pressed(KeyCode::Up) {
        Some(if shifted { Dir::NorthWest } else { Dir::North })
    } else if keys.just_pressed(KeyCode::Down) {
        Some(if shifted { Dir::SouthEast } else { Dir::South })
    } else if keys.just_pressed(KeyCode::Left) {
        Some(if shifted { Dir::SouthWest } else { Dir::West })
    } else if keys.just_pressed(KeyCode::Right) {
        Some(if shifted { Dir::NorthEast } else { Dir::East })
    } else {
        None
    };

    if let Some(direction) = direction {
        let (entity, _player, mut actor) = get_player.single_mut();
        if actor.action.is_some() {
            panic!("Player unexpectedly already has an action");
        }
        let movement = MovementActionDetail::Walk(direction);
        let action = Action {
            entity,
            status: ActionStatus::Queued,
            detail: ActionDetail::Move(movement),
            duration: 10,
            validated: false,
        };
        actor.action = Some(action);
    }
}

// should we immediately call Actor#reset, or use this action to reset?
// atmo we're doing the latter
pub(crate) fn handle_action_invalid(
    mut ev_invalid: EventReader<ActionInvalidEvent>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Actor)>,
) {
    warn!("HANDLER: handle_action_invalid");

    let (entity, mut actor) = query.single_mut();
    for _ in ev_invalid.read() {
        actor.reset();
        commands.entity(entity).insert(ActionPlanRequestMarker);
    }

    // TODO something
}

//
pub(crate) fn validate_move(
    mut ev_invalid: EventWriter<ActionInvalidEvent>,
    mut query: Query<(Entity, &mut Actor, &Locus)>,
    time: Res<TickCount>,
    board: Res<Board>,
) {
    for (entity, mut actor, locus) in query.iter_mut() {
        if let Some(action) = &mut actor.action {
            if action.validated {
                continue;
            }

            match action.detail {
                // TODO
                ActionDetail::Move(MovementActionDetail::Walk(dir)) => {
                    if board
                        .apply_direction(&locus.position, &dir)
                        .is_ok_and(|dir| board.is_unoccupied(&dir))
                    {
                        action.validated = true;
                    } else {
                        ev_invalid.send(ActionInvalidEvent {
                            entity,
                            at: time.as_u32(),
                        });
                    };
                }
                ActionDetail::Move(_) => panic!("not implemented"),
                _ => (), // it's not a movement action, ignore
            }
            // TODO check for issues other than collisions with walls
        } else {
            // no action, should we care?
        }
    }
}
