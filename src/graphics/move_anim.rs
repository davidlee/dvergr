use crate::action::StillWaitForAnimEvent;
use crate::graphics::anim::LerpVec3;
use crate::graphics::TorchMarker;
use crate::typical::*;

pub(crate) fn lerp_vec3_translation(
    mut commands: Commands,
    mut ev_wr: EventWriter<StillWaitForAnimEvent>,
    mut query: Query<(Entity, &mut Transform, &mut LerpVec3)>,
) {
    let mut still_animating = false;

    for (entity, mut transform, mut anim) in query.iter_mut() {
        if anim.is_done() {
            transform.translation = anim.target;
            commands.entity(entity).remove::<LerpVec3>();
            //
            commands.entity(entity).log_components();
        } else {
            transform.translation += anim.delta;
            anim.next();

            still_animating = true;
        }
    }

    if still_animating {
        ev_wr.send(StillWaitForAnimEvent);
    }
}

pub(crate) fn animate_player_fov(
    get_locus: Query<&Locus, With<Player>>,
    mut query: Query<&mut Transform, With<TorchMarker>>,
) {
    let locus = get_locus.single();
    let mut tr = query.single_mut();
    let t = Transform::from_xyz(0., 0., 0.).looking_at(locus.facing.offset().as_vec3(), Vec3::Z);
    tr.rotation = tr.rotation.lerp(t.rotation, 0.2);
}
