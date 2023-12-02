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

fn populate_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut ev_writer: EventWriter<AppInitEvent>,
) {
    info!("[AppState::InitBoard] populate_board");

    commands.spawn_empty().with_children(|cells_entity| {
        for pos in board.coords().iter() {
            let [x, y, z] = pos.to_array();
            let cell = Cell::new(x, y, z);
            let floor = Floor::new(x, y, z, Material::default());
            let vis = PlayerCellVisibility::new(x, y, z);

            if (y % 10 == 0 && x % 6 != 0) || (x % 5 == 0 && y % 3 != 0) {
                let wall = Wall::new(x, y, z, Material::default());
                let cell_entity = cells_entity.spawn((cell, vis, wall, floor)).id();
                board.cell_store.set(*pos, cell_entity);
                board.wall_store.set(*pos, cell_entity);
                board.floor_store.set(*pos, cell_entity);
                board.visibility_store.set(*pos, cell_entity);
            } else {
                let cell_entity = cells_entity.spawn((cell, vis, floor)).id();
                board.cell_store.set(*pos, cell_entity);
                board.floor_store.set(*pos, cell_entity);
                board.visibility_store.set(*pos, cell_entity);
            }
        }
    });

    ev_writer.send(AppInitEvent::SetAppState(AppState::InitPlayer));
}
