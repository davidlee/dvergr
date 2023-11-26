use crate::attributes::Attributes;
use crate::board::Board;
use crate::board::Pos3d;
use crate::creature::*;
use crate::state::AppState;
use bevy::prelude::*;

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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InitPlayer), spawn_player_bundle)
            .add_systems(
                Update,
                movement::validate_directional_input
                    .run_if(state_exists_and_equals(AppState::Game)),
            )
            .add_event::<movement::DirectionalInput>();
    }
}

fn spawn_player_bundle(
    mut commands: Commands,
    mut board: ResMut<Board>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    println!("[AppState::InitPlayer] spawn_player");

    let player_default_position = Pos3d { x: 0, y: 0, z: 0 };
    let player_entity = commands.spawn(PlayerBundle::default()).id();

    board
        .creatures
        // .add(player_entity, vec![player_default_position])
        .add_single(player_entity, player_default_position)
        .unwrap();

    match state.get() {
        AppState::InitPlayer => next_state.set(AppState::InitStage),
        s => panic!("illegal state: {:?}", s),
    }
}

pub mod movement {
    use super::Player;
    use crate::board::{Board, Direction};
    use crate::creature::movement::StartMove;
    use crate::creature::Creature;
    use bevy::prelude::{Entity, Event, EventReader, EventWriter, Query, Res};

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
