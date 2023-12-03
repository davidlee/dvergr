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

pub fn spawn_player(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut ev_writer: EventWriter<AppInitEvent>,
) {
    let player_position = IVec3 { x: 3, y: 3, z: 0 };
    let position = Position::Point(player_position);
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
        .add_single(player_entity, player_position)
        .unwrap();

    ev_writer.send(AppInitEvent::SetAppState(AppState::InitStage))
}
