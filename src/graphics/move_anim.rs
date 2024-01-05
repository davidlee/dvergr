use super::{player_avatar::PlayerAvatar, typical::*};
use crate::typical::*;

pub fn add_changed_creature_mob_move_anim(
    mut commands: Commands,
    mut mob_query: Query<(Entity, &CreatureEntityRef, &mut Transform)>,
    changed_query: Query<(Entity, &Creature, &Locus), Changed<Locus>>,
) {
    for (_mob_entity, CreatureEntityRef(entity), transform) in mob_query.iter_mut() {
        if changed_query.contains(*entity) {
            let (_, _creature, locus) = changed_query.get(*entity).unwrap();
            match locus.position {
                Position::Point(pos) => {
                    let [x, y, _] = pos.as_vec3().to_array();
                    let target = Transform::from_xyz(x, y, 0.);

                    let anim =
                        LerpVec3::from_translation(transform.translation, target.translation, 6); // 6 frames
                    dbg!(&anim);
                    commands.entity(_mob_entity).insert(anim);
                }
                _ => panic!("doesn't support area yet"),
            }
        }
    }
}

pub fn player_movement(
    mut commands: Commands,
    mut avatar_query: Query<(Entity, &PlayerAvatar, &mut LerpVec3, &mut Transform)>,
) {
    for (avatar_entity, _avatar, mut anim, mut transform) in avatar_query.iter_mut() {
        commands.entity(avatar_entity).log_components();
        dbg!(&anim);
        if anim.current_frame == 1 {
            transform.translation = anim.target;
            transform.scale = Vec3::new(1.0, 1.0, 1.0);
            commands.entity(avatar_entity).remove::<LerpVec3>();
        } else {
            transform.translation.x += anim.delta.x;
            transform.translation.y += anim.delta.y;
            let k = anim.total_frames as f32 / 2.0;
            let scale_factor = (k - (anim.current_frame as f32 - k).abs()).abs() / 35.0 + 1.0; // FIXME WTF is 35.0 here??
            transform.scale = Vec3::new(scale_factor, scale_factor, scale_factor);
            anim.current_frame -= 1;
        }
    }
}
