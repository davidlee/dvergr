// use crate::character::*;
use crate::creature::*;
use crate::typical::*;
use bevy::prelude::*;

pub mod movement;

#[derive(Component, Debug, Clone)]
pub struct Player {}

#[derive(Resource, Debug, Clone)]
pub struct PlayerRes {
    pub entity: Entity,
}

#[derive(Event, Debug)]
pub struct SpawnPlayerEvent(pub IVec3);

impl Default for Player {
    fn default() -> Self {
        Player {}
    }
}

#[derive(Bundle, Debug, Clone)]
pub struct PlayerBundle {
    player: Player,
    creature: CreatureBundle,
    character: CharacterBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Player::default(),
            creature: CreatureBundle {
                locus: Locus {
                    position: Position::Point(IVec3::new(3, 3, 0)),
                    ..default()
                },
                pace: Pace::default(),
                age: Age(22),
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

pub fn spawn(
    mut commands: Commands,
    mut board: ResMut<Board>,
    // mut ev_writer: EventWriter<AppInitEvent>,
    mut ev_reader: EventReader<SpawnPlayerEvent>,
) {
    warn!("Spawn Player");
    for SpawnPlayerEvent(pos) in ev_reader.read() {
        let position = Position::Point(*pos);
        warn!("Spawn Player {:?}", position);
        let player_bundle = PlayerBundle {
            creature: CreatureBundle {
                locus: Locus {
                    position,
                    ..default()
                },
                ..default()
            },
            ..default()
        };
        warn!("humm");
        let player_entity = commands.spawn(player_bundle).id();

        board
            .creature_store
            .add_single(player_entity, *pos)
            .unwrap();

        dbg!("inserting PlayerRes");
        commands.insert_resource(PlayerRes {
            entity: player_entity,
        });

        // ev_writer.send(AppInitEvent::SetAppState(AppState::BuildMap));
    }
}
