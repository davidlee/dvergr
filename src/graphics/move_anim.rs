use bevy::prelude::SpotLight;
use std::ops::Add; // for transform.rotation.add

use crate::graphics::anim::{LerpVec3, SimpleFrameTimer};
use crate::typical::*;
use crate::{action::StillWaitForAnimEvent, TorchMarker};

pub(crate) fn lerp_vec3_translation(
    mut commands: Commands,
    mut ev_wr: EventWriter<StillWaitForAnimEvent>,
    mut query: Query<(Entity, &mut Transform, &mut LerpVec3)>,
) {
    warn!("lerpvec3");
    let mut still_animating = false;

    for (entity, mut transform, mut anim) in query.iter_mut() {
        dbg!(entity, &transform, &anim);

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
        warn!("still animating ..");
        ev_wr.send(StillWaitForAnimEvent);
    }
}

pub(crate) fn move_head(
    player_query: Query<(Entity, &Player, &Locus)>,
    // mut avatar_query: Query<(Entity, &PlayerAvatar, &mut LerpVec3, &mut Transform)>,
    mut query: Query<(Entity, &TorchMarker, &SpotLight, &mut Transform)>,
) {
    const K: f32 = 6.0;
    let (_, _, locus) = player_query.get_single().unwrap();
    // FIXME this'll do for now but 135' angle transitions are goofy
    let target = match locus.direction {
        Direction::North | Direction::South => Transform::from_xyz(0., 0., 0.)
            .looking_at(locus.direction.offset().as_vec3(), Vec3::new(-1., -1., 0.)),
        _ => Transform::from_xyz(0., 0., 0.)
            .looking_at(locus.direction.offset().as_vec3(), Vec3::new(0., 0., 0.)),
    };

    for (_, _, _, mut transform) in query.iter_mut() {
        let delta = (target.rotation - transform.rotation) / K;
        transform.rotation = transform.rotation.add(delta);
    }
}
