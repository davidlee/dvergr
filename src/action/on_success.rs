use super::*;
use crate::graphics::typical::LerpVec3;
use crate::{graphics::LogicalGraphicalEntityMapper, typical::*};

pub(crate) fn apply_move(
    mut query_logic: Query<(Entity, &mut Locus, &mut MovementActionDetail)>,
    mut query_gfx: Query<(Entity, &mut Transform)>,
    mut commands: Commands,
    board: ResMut<Board>,
    mapper: Res<LogicalGraphicalEntityMapper>,
) {
    warn!("APPLY MOVE");
    for (logical_entity, mut locus, mov) in query_logic.iter_mut() {
        dbg!(&locus, &mov);

        if let Position::Point(pos) = locus.position {
            dbg!("thundertits!!");

            // update the logical model
            let dest: IVec3 = board.apply_direction(&pos, mov.direction()).unwrap();

            locus.facing = *mov.direction();
            locus.position = Position::Point(dest);

            // remove marker component
            commands
                .entity(logical_entity)
                .remove::<MovementActionDetail>();

            // then add an animation marker to the graphics

            // let [facing_x, facing_y] = locus.facing.offset2df().to_array();
            //     .looking_at(Vec3::new(facing_x, facing_y, 0.), Vec3::splat(0.));

            // let current = query_gfx.component_mut::<Transform>(*gfx_entity);
            // let anim = LerpVec3::from_translation(current.translation, target.translation, 6);
            // dbg!("ANIMATION::::", &anim);

            let anim = LerpVec3::from_translation(pos.as_vec3(), dest.as_vec3(), 6);
            let gfx_entity = mapper.graphical_entity(&logical_entity).unwrap();

            commands.entity(*gfx_entity).insert(anim);
        }
    }
}

pub(crate) fn movement_anim_tick(//
) {
    //
}

pub(crate) fn attack() {
    //
}
