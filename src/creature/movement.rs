use crate::input::UpdateLocus;
use crate::typical::*;

pub fn process_movement(
    mut ev_move: EventReader<UpdateLocus>,
    mut board: ResMut<Board>,
    mut query: Query<(Entity, &Creature, &mut Locus)>,
) {
    for e in ev_move.read() {
        trace!("processing movement .. {:?}", e);

        let (entity, _creature, mut locus) = query.get_mut(e.entity).unwrap();

        locus.position = e.locus.position.clone();
        locus.direction = e.locus.direction;
        locus.facing = e.locus.facing;
        locus.stance = e.locus.stance;

        // reflect the changes on the board's creatures mapping
        if let Position::Point(ivec) = locus.position {
            board.creature_store.update_single(entity, ivec).unwrap();
        } else {
            panic!("Area support not implemented");
        }
    }
}
