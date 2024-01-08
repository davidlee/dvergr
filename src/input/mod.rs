use crate::graphics::anim::LerpVec3;
use crate::player::movement::DirectionalInput;
use crate::typical::*;

use bevy::prelude::{Entity, EventWriter, Input, KeyCode, Query, Res, Transform};

// TODO
//
// Define a common grammar for commands
// issue a movement command
// have it progress time

pub fn keybindings(
    mut ev_player_move: EventWriter<DirectionalInput>,
    keys: Res<Input<KeyCode>>,
    sprite_query: Query<(Entity, &LerpVec3, &Transform)>,
) {
    // ignore any player movement input while animation is in progress
    if sprite_query.get_single().is_ok() {
        return;
    }

    let shifted: bool = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    let direction = if keys.just_pressed(KeyCode::Up) {
        Some(if shifted {
            Direction::NorthWest
        } else {
            Direction::North
        })
    } else if keys.just_pressed(KeyCode::Down) {
        Some(if shifted {
            Direction::SouthEast
        } else {
            Direction::South
        })
    } else if keys.just_pressed(KeyCode::Left) {
        Some(if shifted {
            Direction::SouthWest
        } else {
            Direction::West
        })
    } else if keys.just_pressed(KeyCode::Right) {
        Some(if shifted {
            Direction::NorthEast
        } else {
            Direction::East
        })
    } else {
        None
    };

    if let Some(direction) = direction {
        ev_player_move.send(DirectionalInput { direction })
    }
}
