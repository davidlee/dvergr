use crate::graphics::anim::LerpVec3;
use crate::player::movement::DirectionalInput;
use crate::typical::*;

use bevy::prelude::{Entity, EventWriter, Input, KeyCode, Query, Res, Transform};

pub fn keybindings(
    mut ev_player_move: EventWriter<DirectionalInput>,
    keys: Res<Input<KeyCode>>,
    sprite_query: Query<(Entity, &LerpVec3, &Transform)>,
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
use crate::graphics::typical::*;
// use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::{MouseButton, Window};
use bevy::window::PrimaryWindow;

// FIXME cool, now we have to account for the offset from panning.
//
pub fn mousey_mousey(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    // board: Res<Board>,
    tm_query: Query<&TileMap>,
    // mut cursor_evr: EventReader<CursorMoved>,
    // mut scroll_evr: EventReader<MouseWheel>,
) {
    if buttons.pressed(MouseButton::Left) {
        info!("left mouse currently pressed");
        if let Some(cursor_position) = q_windows.single().cursor_position() {
            // TODO
            // let corrected_cursor = cursor_position - pancam_thingy.translation ..
            //
            info!("Mousey cursor: {:?}", cursor_position);
            let pos = tm_query.single().from_pixels(&cursor_position);
            warn!("MOUSE CLICK: {:?}", pos);
        }
    }
}
