use crate::action::{Action, ActionDetail, ActionStatus, Actor, ActorAction, MovementActionDetail};

use crate::typical::*;

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, States)]
pub(crate) enum PlayerInputState {
    #[default]
    Listen = 1,
    Inactive = 0,
}

pub(crate) fn keybindings(
    mut get_player: Query<(Entity, &Player, &mut Actor), Without<ActorAction>>,
    mut ev_added: EventWriter<ActionAddedEvent>,
    mut commands: Commands,
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
        let (entity, _player, _actor) = get_player.single_mut();
        let movement = MovementActionDetail::Walk(direction);
        let action = Action {
            entity,
            status: ActionStatus::Idle,
            detail: ActionDetail::Move(movement),
            duration: 10,
        };
        dbg!("key command:", action);
        commands.entity(entity).insert(ActorAction(action));
        ev_added.send(ActionAddedEvent { entity });
    }
}
