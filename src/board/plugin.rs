use super::*;

// Plugin & Resource
//

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardRes>()
            .add_systems(OnEnter(AppState::LoadAssets), populate_board);
    }
}
// systems

fn populate_board(mut current: ResMut<BoardRes>) {
    current.board.fill(|x, y, _z| {
        if (y % 10 == 0 && x % 6 != 0) || (x % 5 == 0 && y % 3 != 0) {
            Some(Cell::default())
        } else {
            Some(Cell::empty())
        }
    });
}
