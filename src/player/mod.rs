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

    println!("WARNING: this should be a Position::Point(Pos3d) in spawn_player_bundle");

    let player_default_position = Pos3d { x: 0, y: 0, z: 0 };
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
                let new_pos = pos.adjacent(e.direction);
                match board.cells.get(&new_pos) {
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
                }
            }
        }
    }
}

// https://www.redblobgames.com/grids/circle-drawing/
//
pub mod visibility {
    use crate::graphics::tilemap::DARK_MAP_Z;
    use crate::typical::*;

    pub fn mark_player_visible_cells(
        mut board_mut: ResMut<Board>,
        player_query: Query<(Entity, &Player, &Creature)>,
    ) {
        if let Ok((_entity, _, creature)) = player_query.get_single() {
            match creature.locus.position {
                Position::Point(centre) => {
                    let r: i32 = 6;
                    let top = centre.y - r;
                    let bot = centre.y + r;

                    for y in top..bot {
                        let dy: i32 = y - centre.y;
                        let dx: f32 = f32::sqrt((r * r - dy * dy) as f32);
                        let left: i32 = f32::ceil(centre.x as f32 - dx) as i32;
                        let right: i32 = f32::floor(centre.x as f32 + dx) as i32;

                        for x in left..right {
                            let pos = Pos3d::new(x, y, DARK_MAP_Z);
                            println!("pos light:  {:?}", pos);
                            if let Some(cell) = board_mut.cells.get(&pos) {
                                let mut cell = cell.clone();
                                cell.visibility = CellVisibility::Visible;
                                board_mut.cells.set(pos, cell);
                            }
                            // pew pew
                        }
                    }
                }
                _ => panic!("oops",),
            }
        }
    }
}
