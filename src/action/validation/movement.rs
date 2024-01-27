use super::*;

pub(crate) fn validate_move(
    mut ev_invalid: EventWriter<ActionInvalidatedEvent>,
    mut ev_valid: EventWriter<ActionValidatedEvent>,
    mut query: Query<(Entity, &mut Actor, &mut ActorAction, &Locus)>,
    board: Res<Board>,
) {
    info!("validating move");
    for (entity, _actor, mut action, locus) in query.iter_mut() {
        dbg!("validating move", &action.0.status);

        if action.0.is_runnable() {
            dbg!("runnable");
            continue;
        } else {
            dbg!("not runny");
        }

        let valid = match action.0.detail {
            // TODO check for issues other than collisions with walls
            ActionDetail::Move(MovementActionDetail::Walk(dir)) => board
                .apply_direction(&locus.position, &dir)
                .is_ok_and(|dir| board.is_unoccupied(&dir)),
            ActionDetail::Move(_) => false,
            _ => continue,
        };

        action.0.status = if valid {
            ActionStatus::Ready
        } else {
            ActionStatus::Aborted
        };

        dbg!("determined validity: ", valid, action);

        if valid {
            ev_valid.send(ActionValidatedEvent { entity });
        } else {
            ev_invalid.send(ActionInvalidatedEvent { entity });
        }
    }
}
