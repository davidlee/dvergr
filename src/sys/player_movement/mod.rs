use crate::board::{Direction, Pos};
// use crate::Player;
use crate::player::Player;
use bevy::prelude::{Event, EventReader, EventWriter, Input, KeyCode, Query, Res};

#[derive(Event, Debug)]
pub struct PlayerMovementEvent {
    direction: Direction,
}
pub fn player_movement(
    mut ev_player_move: EventReader<PlayerMovementEvent>,
    mut pos_query: Query<(&mut Player, &mut Pos)>,
    // current_board: Res<CurrentBoard>,
    // map_size_query: Query<&Size>,
) {
    let (_player, pos) = pos_query.single_mut();
    // let map_size: &Size = current_board.size();

    for e in ev_player_move.read() {
        let _to = pos.adjacent(e.direction);
        // ...
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
                Direction::NorthWest
            } else {
                Direction::North
            },
        })
    }

    if keys.just_pressed(KeyCode::Down) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                Direction::SouthEast
            } else {
                Direction::South
            },
        })
    }

    if keys.just_pressed(KeyCode::Left) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                Direction::SouthWest
            } else {
                Direction::West
            },
        })
    }

    if keys.just_pressed(KeyCode::Right) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                Direction::NorthEast
            } else {
                Direction::East
            },
        })
    }
}
