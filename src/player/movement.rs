use crate::creature::movement::StartMove;
use crate::typical::*;

#[derive(Event, Debug)]
pub struct DirectionalInput {
    pub direction: Direction,
}

pub fn validate_directional_input(
    mut ev_input: EventReader<DirectionalInput>,
    mut ev_move: EventWriter<StartMove>,
    cell_query: Query<&Cell>,
    player_query: Query<(Entity, &mut Player, &mut Creature, &mut Locus)>,
    board: Res<Board>,
) {
    if let Ok(q) = player_query.get_single() {
        let (entity, ..) = q;
        let pos = board.creature_store.get_pos_for(&entity).unwrap();
        for e in ev_input.read() {
            match board.apply_direction(pos, &e.direction) {
                Ok(new_pos) => match board.cell_store.get(&new_pos) {
                    Some(cell_entity) => {
                        if let Ok(cell) = cell_query.get_component::<Cell>(*cell_entity) {
                            if cell.passable() {
                                let ev = StartMove::single(*pos, new_pos, entity);
                                trace!("Cell unobstructed ... moving Player: {:?}", ev);
                                ev_move.send(ev);
                            } else {
                                trace!("invalid move to {:?}", cell);
                            }
                        }
                    }
                    None => info!("OUT OF BOUNDS"),
                },
                Err(_str) => error!("Out of bounds."),
            }
        }
    }
}
