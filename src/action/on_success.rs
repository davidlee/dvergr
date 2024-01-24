use super::*;
use crate::graphics::typical::LerpVec3;
use crate::{graphics::LogicalGraphicalEntityMapper, typical::*};

pub(crate) fn apply_move(
    mut query_logic: Query<(Entity, &mut Locus, &mut MovementActionDetail)>,
    mut commands: Commands,
    mut board: ResMut<Board>,
    mapper: Res<LogicalGraphicalEntityMapper>,
) {
    warn!("APPLY MOVE");
    for (logical_entity, mut locus, mov) in query_logic.iter_mut() {
        dbg!(&locus, &mov);
        let pos = locus.position;

        // if let Position::Point(pos) = locus.position {
        dbg!("thundertits!!");

        // update the logical model
        let dest: IVec3 = board.apply_direction(&pos, mov.direction()).unwrap();

        locus.facing = *mov.direction();
        locus.position = dest;

        // keep the board's reference up to date
        board.creature_store.update(logical_entity, locus.position);

        // remove marker component
        commands
            .entity(logical_entity)
            .remove::<MovementActionDetail>();

        // then add an animation marker to the graphics
        let anim = LerpVec3::from_translation(pos.as_vec3(), dest.as_vec3(), 6);
        let gfx_entity = mapper.graphical_entity(&logical_entity).unwrap();

        commands.entity(*gfx_entity).insert(anim);
    }
    // }
}

pub(crate) fn apply_attack() {}