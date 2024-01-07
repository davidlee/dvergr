use std::ops::Add;

use bevy::prelude::SpotLight;

use super::{player_avatar::PlayerAvatar, typical::*};
use crate::{typical::*, TorchMarker};

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
                    let [facing_x, facing_y] = locus.facing.offset2df().to_array();
                    let target = Transform::from_xyz(x, y, 0.)
                        .looking_at(Vec3::new(facing_x, facing_y, 0.), Vec3::splat(0.));

                    let anim =
                        LerpVec3::from_translation(transform.translation, target.translation, 6); // 6 frames
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
    mut sprite_query: Query<(
        Entity,
        &TextureAtlasSprite,
        &mut Transform,
        Without<PlayerAvatar>,
    )>,
) {
    if let Ok((_sprite_entity, _sprite, mut sprite_transform, _)) = sprite_query.get_single_mut() {
        for (avatar_entity, _avatar, mut anim, mut player_transform) in avatar_query.iter_mut() {
            if anim.current_frame == 1 {
                player_transform.translation = anim.target;
                sprite_transform.scale = Vec3::new(1.0, 1.0, 1.0);
                commands.entity(avatar_entity).remove::<LerpVec3>();
            } else {
                player_transform.translation.x += anim.delta.x;
                player_transform.translation.y += anim.delta.y;
                let k = anim.total_frames as f32 / 2.0;
                let scale_factor = (k - (anim.current_frame as f32 - k).abs()).abs() / 35.0 + 1.0; // FIXME WTF is 35.0 here??
                sprite_transform.scale = Vec3::new(scale_factor, scale_factor, scale_factor);
                anim.current_frame -= 1;
            }
        }
    }
}

pub fn move_head(
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
