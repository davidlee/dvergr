use super::typical::*;
use crate::typical::*;

#[derive(Component, Debug)]
pub struct MobMoveAnimation {
    pub delta: Vec2,
    pub target: Vec3,
    pub frames: usize,
    pub total_frames: usize,
}

impl MobMoveAnimation {
    fn from_translation(origin: Vec3, target: Vec3, frames: usize) -> Self {
        let delta = Vec2 {
            x: (target.x - origin.x) / frames as f32,
            y: (target.y - origin.y) / frames as f32,
        };
        MobMoveAnimation {
            delta,
            frames,
            target,
            total_frames: frames,
        }
    }
}

pub fn add_changed_creature_mob_move_anim(
    mut commands: Commands,
    tile_map_query: Query<&TileMap>,
    mut sprite_query: Query<(Entity, &CreatureEntityRef, &mut Transform)>,
    changed_query: Query<(Entity, &Creature, &Locus), Changed<Locus>>,
) {
    for (_sprite_entity, CreatureEntityRef(entity), transform) in sprite_query.iter_mut() {
        if changed_query.contains(*entity) {
            let tile_map = tile_map_query.get_single().expect("WHERE IS MY TILEMAP");
            let (_, _creature, locus) = changed_query.get(*entity).unwrap();
            match locus.position {
                Position::Point(pos) => {
                    let target = transform_from_tilemap_pos(tile_map, &pos);

                    let anim = MobMoveAnimation::from_translation(
                        transform.translation,
                        target.translation,
                        6,
                    );
                    commands.entity(_sprite_entity).insert(anim);
                }
                _ => panic!("doesn't support area yet"),
            }
        }
    }
}

pub fn mob_movement(
    mut commands: Commands,
    mut sprite_query: Query<(Entity, &mut MobMoveAnimation, &mut Transform)>,
) {
    for (sprite_entity, mut anim, mut transform) in sprite_query.iter_mut() {
        if anim.frames == 1 {
            transform.translation = anim.target;
            transform.scale = Vec3::new(1.0, 1.0, 1.0);
            commands.entity(sprite_entity).remove::<MobMoveAnimation>();
        } else {
            transform.translation.x += anim.delta.x;
            transform.translation.y += anim.delta.y;
            let k = anim.total_frames as f32 / 2.0;
            let scale_factor = (k - (anim.frames as f32 - k).abs()).abs() / 35.0 + 1.0;
            transform.scale = Vec3::new(scale_factor, scale_factor, scale_factor);
            anim.frames -= 1;
        }
    }
}