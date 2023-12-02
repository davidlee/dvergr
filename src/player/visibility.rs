use crate::board::geometry::*;
use crate::typical::*;

const PLAYER_VISIBILITY_RANGE: f32 = 24.5; // FIXME add light sources

// https://www.albertford.com/shadowcasting/#symmetry
// https://www.roguebasin.com/index.php/FOV_using_recursive_shadowcasting

pub fn mark_player_visible_cells(
    board_mut: Res<Board>,
    mut visibility_query: Query<&mut PlayerCellVisibility>,
    mut player_query: Query<(&mut Player, &Locus)>,
) {
    if let Ok((mut player, locus)) = player_query.get_single_mut() {
        match locus.position {
            Position::Point(pos) => {
                let circle = circle(pos, PLAYER_VISIBILITY_RANGE);
                let new_vis = sector_facing(locus.facing, &pos, circle);
                let old_vis = player.positions_visible.to_owned();

                for arr_pos in new_vis.difference(&old_vis) {
                    let pos = IVec3::from_array(*arr_pos);
                    match board_mut.cell_store.get(&pos) {
                        Some(entity) => match visibility_query.get_mut(*entity) {
                            Ok(mut vis) => (vis.seen, vis.visible) = (true, true),
                            Err(e) => error!("Error: {:?}", e),
                        },
                        None => (),
                    }
                }

                for arr_pos in old_vis.difference(&new_vis) {
                    let pos = IVec3::from_array(*arr_pos);
                    match board_mut.cell_store.get(&pos) {
                        Some(entity) => match visibility_query.get_mut(*entity) {
                            Ok(mut vis) => vis.visible = false,
                            Err(e) => error!("Error: {:?}", e),
                        },
                        None => (),
                    }
                }
                player.positions_visible = new_vis;
            }
            Position::Area(_) => panic!("oops, unimplemented"),
        }
    }
}
