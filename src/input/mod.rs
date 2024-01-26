use crate::action::{Action, ActionDetail, ActionStatus, Actor, MovementActionDetail};

use crate::typical::*;

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, States)]
pub(crate) enum PlayerInputState {
    #[default]
    Listen = 1,
    Inactive = 0,
}

pub(crate) fn keybindings(
    mut get_player: Query<(Entity, &Player, &mut Actor)>,
    keys: Res<Input<KeyCode>>,
) {
    dbg!("keybindings");

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
