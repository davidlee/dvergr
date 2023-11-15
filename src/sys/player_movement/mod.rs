use crate::map::{SquareDirection, TilePos, TilemapSize};
use crate::Player;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct PlayerMovementEvent {
    direction: SquareDirection,
}

pub fn player_movement(
    mut ev_player_move: EventReader<PlayerMovementEvent>,
    mut pos_query: Query<(&mut Player, &mut TilePos)>,
    map_size_query: Query<&TilemapSize>,
) {
    let (_player, mut pos) = pos_query.single_mut();
    let map_size: &TilemapSize = map_size_query.iter().find(|_x| -> bool { true }).unwrap();

    for e in ev_player_move.iter() {
        if let Some(to) = pos.square_offset(&e.direction, &map_size) {
            TilePos { x: pos.x, y: pos.y } = to;
        }
    }
}

pub fn keybindings(
    mut ev_player_move: EventWriter<PlayerMovementEvent>,
    keys: Res<Input<KeyCode>>,
) {
    let shifted: bool = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    if keys.just_pressed(KeyCode::Up) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                SquareDirection::NorthWest
            } else {
                SquareDirection::North
            },
        })
    }

    if keys.just_pressed(KeyCode::Down) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                SquareDirection::SouthEast
            } else {
                SquareDirection::South
            },
        })
    }

    if keys.just_pressed(KeyCode::Left) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                SquareDirection::SouthWest
            } else {
                SquareDirection::West
            },
        })
    }

    if keys.just_pressed(KeyCode::Right) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                SquareDirection::NorthEast
            } else {
                SquareDirection::East
            },
        })
    }
}
