#![allow(dead_code)]
use crate::typical::*;

#[derive(Event, Debug)]
pub(crate) struct SpawnGoblinEvent(pub IVec3);

pub(crate) fn spawn_goblins(
    map_query: Query<Entity, With<MapMarker>>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut ev_gobs: EventReader<SpawnGoblinEvent>,
) {
    //
    let entity = map_query.single();
    for SpawnGoblinEvent(position) in ev_gobs.read() {
        commands.entity(entity).with_children(|inside_map| {
            let goblin_id = inside_map.spawn(goblin_bundle(position)).id();
            board.creature_store.insert(goblin_id, *position);
            warn!("goblin spawned at {:?}", position);
        });
    }
}

fn goblin_bundle(position: &IVec3) -> CreatureBundle {
    let [x, y, z] = position.to_array();
    let position = IVec3 { x, y, z };
    CreatureBundle {
        creature: Creature {
            dry_weight: 55.,
            height: 125,
        },
        locus: Locus {
            position,
            ..default()
        },

        spatial: SpatialBundle {
            transform: Transform::from_xyz(position.x as f32, position.y as f32, 0.),
            ..default()
        },
        species: Species::Goblin,
        size: CreatureSize::Small,
        // rest
        ..default()
    }
}
