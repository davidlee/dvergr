use crate::board::Direction;
use crate::graphics::mobs::MobMoveAnimation;
use crate::player::movement::DirectionalInput;

use bevy::prelude::{Entity, EventWriter, Input, KeyCode, Query, Res, Transform};

pub fn keybindings(
    mut ev_player_move: EventWriter<DirectionalInput>,
    keys: Res<Input<KeyCode>>,
    sprite_query: Query<(Entity, &MobMoveAnimation, &Transform)>,
) {
    if sprite_query.get_single().is_ok() {
        // ignore any player movement input while animation is in progress
        // FIXME we probably want a more robust approach to preventing movement issues
        return;
    }

    let shifted: bool = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    if keys.just_pressed(KeyCode::Up) {
        ev_player_move.send(DirectionalInput {
            direction: if shifted {
                Direction::NorthWest
            } else {
                Direction::North
            },
        })
    }

    if keys.just_pressed(KeyCode::Down) {
        ev_player_move.send(DirectionalInput {
            direction: if shifted {
                Direction::SouthEast
            } else {
                Direction::South
            },
        })
    }

    if keys.just_pressed(KeyCode::Left) {
        ev_player_move.send(DirectionalInput {
            direction: if shifted {
                Direction::SouthWest
            } else {
                Direction::West
            },
        })
    }

    if keys.just_pressed(KeyCode::Right) {
        ev_player_move.send(DirectionalInput {
            direction: if shifted {
                Direction::NorthEast
            } else {
                Direction::East
            },
        })
    }
}
