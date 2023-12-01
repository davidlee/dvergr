use crate::events::begin_action::UpdateLocus;
use crate::typical::*;

#[derive(Event, Debug)]
pub struct DirectionalInput {
    pub direction: Direction,
}

pub fn validate_directional_input(
    mut ev_input: EventReader<DirectionalInput>,
    mut ev_move: EventWriter<UpdateLocus>,
    cell_query: Query<&Cell>,
    player_query: Query<(Entity, &mut Player, &mut Creature, &mut Locus)>,
    board: Res<Board>,
) {
    if let Ok(q) = player_query.get_single() {
        let (entity, .., locus) = q;
        for e in ev_input.read() {
            match locus.position {
                Position::Point(curr_pos_ivec) => {
                    match board.apply_direction(&curr_pos_ivec, &e.direction) {
                        Ok(new_pos) => match board.cell_store.get(&new_pos) {
                            Some(cell_entity) => {
                                let cell = cell_query
                                    .get_component::<Cell>(*cell_entity)
                                    .expect("missing cell");
                                if cell.passable() {
                                    let ev = UpdateLocus {
                                        entity,
                                        locus: Locus {
                                            position: Position::Point(new_pos),
                                            direction: e.direction,
                                            facing: e.direction,
                                            velocity: locus.velocity,
                                            stance: locus.stance,
                                            weight: locus.weight,
                                        },
                                        from: locus.position.clone(),
                                    };
                                    ev_move.send(ev);
                                } else {
                                    trace!("invalid move to {:?}", cell);
                                }
                            }
                            None => info!("OUT OF BOUNDS"),
                        },
                        Err(_str) => error!("Out of bounds."),
                    }

                    //
                }
                _ => (),
            }
            // let Position::Point(pos) = locus.position;
        }
    }
}
