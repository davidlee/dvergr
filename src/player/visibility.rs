
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
                            Err(e) => error!("Error: {:?}", e),
                        },
                        None => (),
                    }
                }

                for arr_pos in old_vis.difference(&new_vis) {
                    let pos = IVec3::from_array(*arr_pos);
                    match board_mut.cell_store.get(&pos) {
                        Some(cell_entity) => match cell_query.get_mut(*cell_entity) {
                            Ok((_cell, mut vis)) => vis.visible = false,
                            Err(e) => error!("Error: {:?}", e),
                        },
                        None => (),
                    }
                }
                player.positions_visible = new_vis;
            }
            _ => panic!("oops, unimplemented",),
        }
    }
}
