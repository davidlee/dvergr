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

fn get_cell_for_test_board(pos: &IVec3) -> Cell {
    let [x, y, z] = pos.to_array();
    if (y % 10 == 0 && x % 6 != 0) || (x % 5 == 0 && y % 3 != 0) {
        Cell::wall(IVec3::new(x, y, z))
    } else {
        Cell::empty(IVec3::new(x, y, z))
    }
}

fn populate_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut ev_writer: EventWriter<AppInitEvent>,
) {
    println!("[AppState::InitBoard] populate_board");

    commands.spawn_empty().with_children(|cells_entity| {
        for pos in board.coords().iter() {
            let cell = get_cell_for_test_board(pos);
            let vis = PlayerCellVisibility::new(pos.to_owned());
            let e = cells_entity.spawn((cell, vis)).id();
            board.cell_store.set(*pos, e);
        }
    });

    ev_writer.send(AppInitEvent::SetAppState(AppState::InitPlayer));
}
