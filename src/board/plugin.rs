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
    let mut walls: Vec<(Entity, [i32; 3])> = vec![];

    commands.spawn_empty().with_children(|cells_entity| {
        for pos in board.coords().iter() {
            let [x, y, z] = pos.to_array();
            let cell = Cell::new(x, y, z);
            let floor = Floor::new(x, y, z, Material::default());
            let vis = PlayerCellVisibility::new(x, y, z);

            let cell_entity: Entity = cells_entity.spawn((cell, vis, floor)).id();

            board.cell_store.set(*pos, cell_entity);
            board.floor_store.set(*pos, cell_entity);
            board.visibility_store.set(*pos, cell_entity);

            if (y % 10 == 0 && x % 6 != 0) || (x % 5 == 0 && y % 3 != 0) {
                walls.push((cell_entity, [x, y, z]));
            }
        }
    });

    for (entity, [x, y, z]) in walls.iter() {
        let wall = Wall::new(*x, *y, *z, Material::default());
        commands.entity(*entity).insert(wall.clone()); // would be nice to avoid cloning
        board.wall_store.set(wall.position, *entity);
    }

    ev_writer.send(AppInitEvent::SetAppState(AppState::InitPlayer));
}
