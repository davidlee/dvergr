use crate::typical::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Player;

#[derive(Bundle, Debug, Clone)]
pub struct PlayerBundle {
    player: Player,
    creature: Creature,
    attributes: Attributes,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Player,
            creature: Creature::human(),
            attributes: Attributes::new(),
        }
    }
}

pub fn spawn_player_bundle(
    mut commands: Commands,
    mut board: ResMut<Board>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    println!("[AppState::InitPlayer] spawn_player");

    println!("WARNING: this should be a Position::Point(UVec3) in spawn_player_bundle");

    let player_default_position = UVec3 { x: 0, y: 0, z: 0 };
    let player_entity = commands.spawn(PlayerBundle::default()).id();

    board
        .creatures
        .add_single(player_entity, player_default_position)
        .unwrap();

    match state.get() {
        AppState::InitPlayer => next_state.set(AppState::InitStage),
        s => panic!("illegal state: {:?}", s),
    }
}

pub mod movement {
    use crate::creature::movement::StartMove;
    use crate::typical::*;

    #[derive(Event, Debug)]
    pub struct DirectionalInput {
        pub direction: Direction,
    }

    pub fn validate_directional_input(
        mut ev_input: EventReader<DirectionalInput>,
        mut ev_move: EventWriter<StartMove>,
        player_query: Query<(Entity, &mut Player, &mut Creature)>,
        board: Res<Board>,
    ) {
        if let Ok(q) = player_query.get_single() {
            let (entity, ..) = q;
            let pos = board.creatures.get_pos_for(&entity).unwrap();
            for e in ev_input.read() {
                match board.apply_direction(pos, &e.direction) {
                    Ok(new_pos) => match board.cells.get(&new_pos) {
                        Some(cell) => {
                            if cell.passable() {
                                let ev = StartMove::single(pos.clone(), new_pos, entity);
                                println!("Cell unobstructed ... moving Player: {:?}", ev);
                                ev_move.send(ev);
                            } else {
                                println!("invalid move to {:?}", cell);
                            }
                        }
                        None => println!("OUT OF BOUNDS"),
                    },
                    Err(_str) => println!("Out of bounds."),
                }
            }
        }
    }
}

pub mod visibility {
    use crate::board::geometry::circle;
    // use crate::graphics::tilemap::DARK_MAP_Z;
    use crate::typical::*;

    pub fn mark_player_visible_cells(
        mut board_mut: ResMut<Board>,
        player_query: Query<(Entity, &Player, &Creature)>,
    ) {
        if let Ok((_entity, _, creature)) = player_query.get_single() {
            match creature.locus.position {
                Position::Point(pos) => {
                    for v in circle(pos, 6) {
                        match board_mut.cells.get(&v) {
                            Some(cell) => {
                                let mut cell = cell.clone();
                                cell.visibility = CellVisibility::Visible;
                                board_mut.cells.set(pos, cell);
                            }
                            None => println!("circle has missing cells"),
                        }
                    }
                }
                _ => panic!("oops",),
            }
        }
    }
}
