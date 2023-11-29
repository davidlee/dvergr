use crate::typical::*;

// TODO support multiple cells
#[derive(Event, Debug)]
pub struct StartMove {
    pub from: IVec3,
    pub to: IVec3,
    pub entity: Entity,
}

impl StartMove {
    pub fn single(from: IVec3, to: IVec3, entity: Entity) -> Self {
        StartMove { from, to, entity }
    }
}

pub fn process_movement(
    mut ev_move: EventReader<StartMove>,
    mut board: ResMut<Board>,
    mut query: Query<(Entity, &Creature, &mut Locus)>,
) {
    for e in ev_move.read() {
        println!("processing movement .. {:?}", e);
        let (entity, _creature, mut locus) = query.get_mut(e.entity).unwrap();
        // first make the changes to the creature
        locus.position = Position::Point(e.to);
        // then reflect the changes on the board's creatures mapping
        board.creature_store.update_single(entity, e.to).unwrap();
    }
}
