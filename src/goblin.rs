#![allow(dead_code)]
use crate::typical::{graphics::GoblinSpritesheet, *};

#[derive(Event, Debug)]
pub(crate) struct SpawnGoblinEvent(pub IVec3);

// https://bevy-cheatbook.github.io/cookbook/cursor2world.html

pub(crate) fn spawn_goblins(
    cam_query: Query<(Entity, &Camera3d)>,
    other_query: Query<(Entity, &BoardMarker)>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut ev_gobs: EventReader<SpawnGoblinEvent>,
    sprite: Res<GoblinSpritesheet>,
) {
    return;

    let (e, _) = other_query.single();
    let (ce, cam) = cam_query.single();

    commands.entity(e).with_children(|on_board| {
        for SpawnGoblinEvent(position) in ev_gobs.read() {
            let goblin_id = on_board
                .spawn(goblin_bundle(position))
                .with_children(|gobbo| {
                    gobbo.spawn((SpriteSheetBundle {
                        texture_atlas: sprite.atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(0),
                        transform: Transform::from_translation(Vec3::splat(0.)), // (position.as_vec3()),
                        ..default()
                    },));
                })
                .id();
            board.creature_store.insert(goblin_id, *position);
            warn!("goblin spawned at {:?}", position);
        }
    });
}

fn goblin_bundle(position: &IVec3) -> CreatureBundle {
    CreatureBundle {
        creature: Creature {
            dry_weight: 55.,
            height: 125,
        },
        locus: Locus {
            position: *position,
            ..default()
        },

        spatial: SpatialBundle {
            transform: Transform::from_translation(position.as_vec3()),
            ..default()
        },
        species: Species::Goblin,
        size: CreatureSize::Small,
        // rest
        ..default()
    }
}
