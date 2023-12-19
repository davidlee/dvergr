use crate::creature::CreatureBundle;
use crate::typical::*;
use bevy::prelude::*;
use bevy::utils::HashSet;

pub mod movement;
pub mod visibility;

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub positions_visible: HashSet<[i32; 2]>,
}

#[derive(Event, Debug)]
pub struct SpawnPlayerEvent(pub IVec3);

impl Default for Player {
    fn default() -> Self {
        Player {
            positions_visible: HashSet::new(),
            // movement delta?
        }
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
                ..default()
            },
            character: CharacterBundle {
                character: Character,
                equipment: Equipment::default(),
                pace: Pace::default(),
            },
        }
    }
}

pub fn spawn(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut ev_writer: EventWriter<AppInitEvent>,
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
        let player_entity = commands.spawn(player_bundle).id();

        board
            .creature_store
            .add_single(player_entity, *pos)
            .unwrap();
        ev_writer.send(AppInitEvent::SetAppState(AppState::SpawnPlayer));
    }
}
