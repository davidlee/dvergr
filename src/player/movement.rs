use crate::events::begin_action::UpdateLocus;
use crate::typical::*;

#[derive(Event, Debug)]
pub struct DirectionalInput {
    pub direction: Direction,
}

// TODO

pub fn validate_directional_input(
    mut ev_input: EventReader<DirectionalInput>,
    mut ev_move: EventWriter<UpdateLocus>,
    player_query: Query<(Entity, &mut Player, &mut Creature, &mut Locus)>,
    board: Res<Board>,
) {
    if let Ok(q) = player_query.get_single() {
        let (entity, .., locus) = q;
        for e in ev_input.read() {
            if let Position::Point(curr_pos_ivec) = locus.position {
                match board.apply_direction(&curr_pos_ivec, &e.direction) {
                    Ok(new_pos) => match board.wall_store.get(&new_pos) {
                        Some(_) => trace!("cell contains a wall"),
                        None => {
                            let ev = UpdateLocus {
                                entity,
                                locus: Locus {
                                    position: Position::Point(new_pos),
                                    direction: e.direction,
                                    facing: e.direction,
                                    velocity: locus.velocity,
                                    stance: locus.stance,
                                    weight: locus.weight,
                                    pace: Pace::default(),
                                },
                                from: locus.position.clone(),
                            };
                            ev_move.send(ev);
                        }
                    },
                    // TODO load bordering map
                    Err(_str) => error!("Out of bounds."),
                }
            }
        }
    }
}
