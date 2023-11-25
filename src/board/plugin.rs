use super::*;
use bevy::prelude::{App, NextState, OnEnter, Plugin, ResMut};

// Plugin & Resource
//

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_systems(OnEnter(AppState::InitBoard), populate_board);
    }
}
// systems

fn populate_board(mut board: ResMut<Board>, mut state: ResMut<NextState<AppState>>) {
    println!("[AppState::InitBoard] populate_board");
    board.fill(|x, y, _z| {
        if (y % 10 == 0 && x % 6 != 0) || (x % 5 == 0 && y % 3 != 0) {
            Some(Cell::default())
        } else {
            Some(Cell::empty())
        }
    });
    state.set(AppState::InitPlayer);
}
