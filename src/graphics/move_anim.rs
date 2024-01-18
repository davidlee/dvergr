use std::ops::Add;

use bevy::prelude::SpotLight;

use super::{player_avatar::PlayerAvatar, typical::*};
use crate::{action::StillWaitForAnimEvent, typical::*, TorchMarker};

pub(crate) fn player_movement(
    mut commands: Commands,
    mut ev_wr: EventWriter<StillWaitForAnimEvent>,
    mut sprite_query: Query<(
        Entity,
        &TextureAtlasSprite,
        &mut Transform,
        Without<PlayerAvatar>,
    )>,
    mut avatar_query: Query<(Entity, &PlayerAvatar, &mut LerpVec3, &mut Transform)>,
) {
    warn!("player mov");
    let mut still_animating = false;

    if let Ok((_sprite_entity, _sprite, mut sprite_transform, _)) = sprite_query.get_single_mut() {
        warn!("found ..");
        for (avatar_entity, _avatar, mut anim, mut player_transform) in avatar_query.iter_mut() {
            warn!("yeah gotem");
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
                still_animating = true;
            }
        }
    }

    if still_animating {
        warn!("still animating");
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
