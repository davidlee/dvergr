use super::*;

use crate::graphics::anim::LerpVec3;

pub(crate) fn apply_move(
    mut query_logic: Query<(Entity, &mut Locus, &mut MovementActionDetail)>,
    mut commands: Commands,
    mut board: ResMut<Board>,
) {
    for (entity, mut locus, mov) in query_logic.iter_mut() {
        dbg!(&locus, &mov);
        let pos = locus.position;

        // update the logical model
        let dest: IVec3 = board.apply_direction(&pos, mov.direction()).unwrap();

        locus.facing = *mov.direction();
        locus.position = dest;

        // keep the board's reference up to date
        board.creature_store.update(entity, locus.position);

        // then add an animation marker to the graphics
        let anim = LerpVec3::from_translation(pos.as_vec3(), dest.as_vec3(), 6);
        // remove marker component
        commands.entity(entity).remove::<MovementActionDetail>();
        commands.entity(entity).insert(anim);
    }
}

pub(crate) fn apply_attack() {}
