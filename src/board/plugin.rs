use crate::typical::*;

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

fn get_cell_for_test_board(pos: &UVec3) -> Cell {
    let [x, y, _z] = pos.to_array();
    if (y % 10 == 0 && x % 6 != 0) || (x % 5 == 0 && y % 3 != 0) {
        Cell::default()
    } else {
        Cell::empty()
    }
}

fn populate_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut state: ResMut<NextState<AppState>>,
) {
    println!("[AppState::InitBoard] populate_board");

    commands.spawn_empty().with_children(|cells_entity| {
        for pos in board.coords().iter() {
            let cell = get_cell_for_test_board(pos);
            let e = cells_entity.spawn(cell).id();
            board.cell_entities.set(*pos, e);
        }
    });

    state.set(AppState::InitPlayer);
}
