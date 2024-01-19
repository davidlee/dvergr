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
        warn!("FOUND THIS {:?}", &locus);
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
            let graphical_entity = mapper.graphical_entity(&logical_entity).unwrap();

            let [x, y, _] = pos.as_vec3().to_array();
            let [facing_x, facing_y] = locus.facing.offset2df().to_array();
            let target = Transform::from_xyz(x, y, 0.)
                .looking_at(Vec3::new(facing_x, facing_y, 0.), Vec3::splat(0.));

            let current = query_gfx.component_mut::<Transform>(*graphical_entity);
            let anim = LerpVec3::from_translation(current.translation, target.translation, 6);
            dbg!("ANIMATION::::", &anim);
            commands.entity(*graphical_entity).insert(anim);
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
