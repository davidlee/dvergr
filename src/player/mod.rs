use crate::creature::CreatureBundle;
use crate::typical::*;
use bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Component, Debug, Clone)]
pub struct Player {
    positions_visible: HashSet<[i32; 3]>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            positions_visible: HashSet::new(),
            // movement delta?
        }
    }
}

#[derive(Bundle, Debug, Clone)]
pub struct PlayerBundle {
    player: Player,
    creature: CreatureBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Player::default(),
            creature: CreatureBundle {
                locus: Locus {
                    position: Position::Point(IVec3::new(0, 0, 0)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut ev_writer: EventWriter<AppInitEvent>,
) {
    let player_position = IVec3 { x: 0, y: 0, z: 0 };
    let player_bundle = PlayerBundle::default();
    let player_entity = commands.spawn(player_bundle).id();

    board
        .creature_store
        .add_single(player_entity, player_position)
        .unwrap();

    ev_writer.send(AppInitEvent::SetAppState(AppState::InitStage))
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
        cell_query: Query<&Cell>,
        player_query: Query<(Entity, &mut Player, &mut Creature, &mut Locus)>,
        board: Res<Board>,
    ) {
        if let Ok(q) = player_query.get_single() {
            println!("yea?");
            let (entity, ..) = q;
            let pos = board.creature_store.get_pos_for(&entity).unwrap();
            println!("Pos?? {:?}", pos);
            for e in ev_input.read() {
                println!("input");
                match board.apply_direction(pos, &e.direction) {
                    Ok(new_pos) => match board.cell_store.get(&new_pos) {
                        Some(cell_entity) => {
                            println!("some cell {:?}", cell_entity);
                            if let Ok(cell) = cell_query.get_component::<Cell>(*cell_entity) {
                                if cell.passable() {
                                    let ev = StartMove::single(*pos, new_pos, entity);
                                    // println!("Cell unobstructed ... moving Player: {:?}", ev);
                                    ev_move.send(ev);
                                } else {
                                    // println!("invalid move to {:?}", cell);
                                }
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
    use crate::board::geometry::circle_hash_set;
    use crate::typical::*;

    const PLAYER_VISIBILITY_RANGE: i32 = 6; // FIXME add light sources

    pub fn mark_player_visible_cells(
        board_mut: Res<Board>,
        mut cell_query: Query<(&Cell, &mut PlayerCellVisibility)>,
        mut player_query: Query<(Entity, &mut Player, &Creature, &Locus)>,
    ) {
        if let Ok((_, mut player, _creature, locus)) = player_query.get_single_mut() {
            match locus.position {
                Position::Point(pos) => {
                    let new_vis = circle_hash_set(pos, PLAYER_VISIBILITY_RANGE);
                    let old_vis = player.positions_visible.to_owned();

                    for arr_pos in new_vis.difference(&old_vis) {
                        let pos = IVec3::from_array(*arr_pos);
                        match board_mut.cell_store.get(&pos) {
                            Some(cell_entity) => match cell_query.get_mut(*cell_entity) {
                                Ok((_cell, mut vis)) => (vis.seen, vis.visible) = (true, true),
                                Err(e) => println!("Error: {:?}", e),
                            },
                            None => (),
                        }
                    }

                    for arr_pos in old_vis.difference(&new_vis) {
                        let pos = IVec3::from_array(*arr_pos);
                        match board_mut.cell_store.get(&pos) {
                            Some(cell_entity) => match cell_query.get_mut(*cell_entity) {
                                Ok((_cell, mut vis)) => vis.visible = false,
                                Err(e) => println!("Error: {:?}", e),
                            },
                            None => (),
                        }
                    }
                    player.positions_visible = new_vis;
                }
                _ => panic!("oops",),
            }
        }
    }
}
