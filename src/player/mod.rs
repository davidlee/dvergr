use crate::creature::{Character, CharacterBundle, CharacterLevel, CreatureBundle};
use crate::typical::*;
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Default)]
pub(crate) struct Player;

#[derive(Event, Debug)]
pub(crate) struct SpawnPlayerEvent(pub IVec3);

#[derive(Bundle, Debug)]
pub(crate) struct PlayerBundle {
    player: Player,
    creature: CreatureBundle,
    character: CharacterBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Player,
            creature: CreatureBundle {
                locus: Locus {
                    position: IVec3::new(3, 3, 0),
                    ..default()
                },
                pace: Pace::default(),
                species: Species::Dwarf,
                ..default()
            },
            character: CharacterBundle {
                character: Character {
                    name: None,
                    level: CharacterLevel(1),
                    experience: 0,
                },
                ..default()
            },
        }
    }
}

pub(crate) fn spawn(
    mut commands: Commands,
    mut board: ResMut<Board>,
    // mut ev_writer: EventWriter<AppInitEvent>,
    mut ev_reader: EventReader<SpawnPlayerEvent>,
) {
    warn!("Spawn Player");
    for SpawnPlayerEvent(pos) in ev_reader.read() {
        warn!("Spawn Player {:?}", pos);

        let player_bundle = PlayerBundle {
            creature: CreatureBundle {
                locus: Locus {
                    position: *pos,
                    ..default()
                },
                ..default()
            },
            ..default()
        };
        warn!("humm");
        let player_entity = commands.spawn(player_bundle).id();

        board.creature_store.insert(player_entity, *pos);

        // dbg!("inserting PlayerRes");
        // commands.insert_resource(PlayerRes {
        //     entity: player_entity,
        // });
    }
}
