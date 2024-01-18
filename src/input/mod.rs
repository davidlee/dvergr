use crate::action::{
    Action, ActionDetail, ActionStatus, MovementActionDetail, PlayerActionInvalidEvent,
};
use crate::board::direction::Direction;
use crate::state::TickState;
use crate::typical::*;
use bevy::prelude::{Entity, EventWriter, Input, KeyCode, Query, Res};
use bevy::utils::tracing::*;

#[derive(Event, Debug)]
pub(crate) struct UpdateLocus {
    pub entity: Entity,
    pub locus: Locus,
    pub from: Position,
}

pub(crate) fn keybindings(
    mut get_player: Query<(Entity, &mut Player)>,
    mut next_state: ResMut<NextState<TickState>>,
    // mut commands: Commands,
    keys: Res<Input<KeyCode>>,
) {
    let shifted: bool = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    let direction = if keys.just_pressed(KeyCode::Up) {
        Some(if shifted {
            Direction::NorthWest
        } else {
            Direction::North
        })
    } else if keys.just_pressed(KeyCode::Down) {
        Some(if shifted {
            Direction::SouthEast
        } else {
            Direction::South
        })
    } else if keys.just_pressed(KeyCode::Left) {
        Some(if shifted {
            Direction::SouthWest
        } else {
            Direction::West
        })
    } else if keys.just_pressed(KeyCode::Right) {
        Some(if shifted {
            Direction::NorthEast
        } else {
            Direction::East
        })
    } else {
        None
    };

    if let Some(direction) = direction {
        let (entity, mut player) = get_player.single_mut();
        if player.action.is_some() {
            panic!("Player unexpectedly already has an action");
        }
        // let entity = player_ref.entity;
        let movement = MovementActionDetail::Walk(direction);
        let action = Action {
            entity,
            status: ActionStatus::Queued,
            detail: ActionDetail::Move(movement),
            duration: 10,
        };
        player.action = Some(action);
        next_state.set(TickState::ValidatePlayerAction);
    }
}

//
pub(crate) fn handle_ev_player_action_invalid(
    mut ev_invalid: EventReader<PlayerActionInvalidEvent>,
    mut player_query: Query<&mut Player>,
    mut next_state: ResMut<NextState<TickState>>,
) {
    warn!("HANDLER: handle_ev_player_action_invalid");

    let mut player = player_query.get_single_mut().unwrap();
    for _ in ev_invalid.read() {
        player.action = None;
    }

    next_state.set(TickState::PlayerInput);
}

//
pub(crate) fn validate_player_move(
    mut next_state: ResMut<NextState<TickState>>,
    mut ev_invalid: EventWriter<PlayerActionInvalidEvent>,
    mut player_query: Query<(&mut Player, &Locus)>,
    board: Res<Board>,
) {
    if let Ok((player, locus)) = player_query.get_single_mut() {
        let mut direction: Option<Direction> = None;
        let mut valid = false;

        if let Some(action) = &player.action {
            direction = match action.detail {
                // hrurr
                ActionDetail::Move(MovementActionDetail::Walk(dir)) => Some(dir),
                _ => None,
            };
        }

        if direction.is_some() {
            if let Position::Point(origin) = locus.position {
                if let Ok(destination) = board.apply_direction(&origin, &direction.unwrap()) {
                    // TODO check for things other than walls - statues, pillars, creatures, doors ...
                    valid = board.is_unoccupied(&destination);
                } else {
                    warn!("out of bounds");
                    valid = false;
                }
            }
        } else {
            return;
        }

        if valid {
            next_state.set(TickState::PrepareAgentActions);
        } else {
            ev_invalid.send(PlayerActionInvalidEvent);
            // player.action = None;
            // warn!("Invalid player input");
            // next_state.set(TickState::PlayerInput);
        }
    }
}
