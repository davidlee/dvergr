use super::super::action::Direction;
use super::super::Player;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Event, Debug)]
pub struct PlayerMovementEvent {
    direction: Direction,
}

pub fn move_to<'a, 'b>(
    &pos: &'a TilePos,
    dir: &'b Direction,
    map_size: &TilemapSize,
) -> Result<TilePos, &'b str> {
    let mut dest = TilePos { x: pos.x, y: pos.y };

    let result = (|| -> Result<TilePos, &str> {
        let (x, y) = dir.as_xy();
        dest.x = (dest.x as i32 + x) as u32;
        dest.y = (dest.y as i32 + y) as u32;
        Ok(dest)
    })()?;

    if result.within_map_bounds(map_size) {
        Ok(dest)
    } else {
        // TODO send invalid command notification
        // println!("Out of bounds! {:?}", dest)
        Err(&"out of bounds")
    }
}

pub fn player_movement(
    mut ev_player_move: EventReader<PlayerMovementEvent>,
    mut pos_query: Query<(&mut Player, &mut TilePos)>,
    map_size_query: Query<&TilemapSize>,
) {
    let (_player, mut pos) = pos_query.single_mut();
    let map_size: &TilemapSize = map_size_query.iter().find(|_x| -> bool { true }).unwrap();

    for e in ev_player_move.iter() {
        if let Ok(to) = move_to(&pos, &e.direction, map_size) {
            TilePos { x: pos.x, y: pos.y } = to;
        } else {
            // invalid command
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
                Direction::UpLeft
            } else {
                Direction::Up
            },
        })
    }

    if keys.just_pressed(KeyCode::Down) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                Direction::DownRight
            } else {
                Direction::Down
            },
        })
    }

    if keys.just_pressed(KeyCode::Left) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                Direction::DownLeft
            } else {
                Direction::Left
            },
        })
    }

    if keys.just_pressed(KeyCode::Right) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                Direction::UpRight
            } else {
                Direction::Right
            },
        })
    }
}
