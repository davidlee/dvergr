use super::*;

pub(crate) fn validate_move(
    mut ev_invalid: EventWriter<ActionInvalidatedEvent>,
    mut ev_valid: EventWriter<ActionValidatedEvent>,
    mut query: Query<(Entity, &mut Actor, &Locus)>,
    board: Res<Board>,
) {
    info!("validating move");
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
                        dbg!("yeah its good");
                        action.validated = true;
                        ev_valid.send(ActionValidatedEvent { entity });
                    } else {
                        dbg!("nah its bad");
                        ev_invalid.send(ActionInvalidatedEvent { entity });
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
