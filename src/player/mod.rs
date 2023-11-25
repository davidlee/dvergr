use crate::attributes::Attributes;
use crate::board::Board;
use crate::board::Direction;
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
            creature: Creature::human(Pos3d { x: 0, y: 0, z: 0 }),
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
                player_movement.run_if(state_exists_and_equals(AppState::Game)),
            )
            .add_event::<PlayerMovementEvent>();
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
        .add(player_entity, vec![player_default_position])
        .unwrap();

    match state.get() {
        AppState::InitPlayer => next_state.set(AppState::InitStage),
        s => panic!("illegal state: {:?}", s),
    }
}

#[derive(Event, Debug)]
pub struct PlayerMovementEvent {
    pub direction: Direction,
}

pub fn player_movement(
    mut ev_player_move: EventReader<PlayerMovementEvent>,
    mut player_query: Query<(&mut Player, &mut Creature)>,
    board: ResMut<Board>,
) {
    if let Ok(q) = player_query.get_single_mut() {
        let (_, creature) = q;
        let pos = creature.position;

        for e in ev_player_move.read() {
            let new_pos = pos.adjacent(e.direction);
            // println!("we want to move Player to: {:?}", to);
            // let cell = board.cells.get(&pos.adjacent(e.direction));
            match board.cells.get(&new_pos) {
                Some(cell) => {
                    if cell.passable() {
                        println!("Player is moving ...");
                        // make the change to the logical player position ...
                        // creature.position = new_pos;
                        // br.
                    } else {
                        println!("invalid move to {:?}", cell);
                    }
                }
                None => println!("OUT OF BOUNDS"),
            }
            // if cell.unwrap().passable() {
            //     // TODO other checks ..
            // }
            // check the board to see if that move's legal ...
            // println!("that'd be into this cell: {:?}", cell);
            // ...
        }
    }
}
