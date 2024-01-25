use super::*;

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
