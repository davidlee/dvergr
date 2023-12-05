// use bevy::reflect::List;

use crate::board::geometry::*;
use crate::board::shadowcast::*;
use crate::typical::*;

// const PLAYER_VISIBILITY_RANGE: f32 = 24.5; // FIXME add light sources

// https://www.albertford.com/shadowcasting/#symmetry
// https://www.roguebasin.com/index.php/FOV_using_recursive_shadowcasting

/*
TODO:
-----

+ separate lighting from player visibility
+ show lit but obscured cells as geometric gold
+ implement peripheral vision


*/

pub fn mark_player_visible_cells(
    board: Res<Board>,
    mut visibility_query: Query<&mut PlayerCellVisibility>,
    mut player_query: Query<(&mut Player, &Locus)>,
) {
    if let Ok((mut player, locus)) = player_query.get_single_mut() {
        match locus.position {
            Position::Point(pos) => {
                let unobscured =
                    shadowcast_visibility_2d([pos.x, pos.y], &board.wall_store.as_hashset2d())
                        .into_iter()
                        .collect::<HashSet<[i32; 2]>>();

                let visible: HashSet<[i32; 2]> = fov_facing(&pos, locus.facing, 28.)
                    .intersection(&unobscured)
                    .into_iter()
                    .cloned()
                    .collect();

                let prev_visible = player.positions_visible.to_owned(); // FIXME

                // these cells are newly visible
                for arr_pos in visible.difference(&prev_visible) {
                    let pos = IVec3::from_array([arr_pos[0], arr_pos[1], pos.z]);
                    match board.cell_store.get(&pos) {
                        Some(entity) => match visibility_query.get_mut(*entity) {
                            Ok(mut vis) => (vis.seen, vis.visible) = (true, true),
                            Err(e) => error!("Error: {:?}", e),
                        },
                        None => (),
                    }
                }

                // these cells are newly obscured
                for arr_pos in prev_visible.difference(&visible) {
                    let pos = IVec3::from_array([arr_pos[0], arr_pos[1], pos.z]);
                    match board.cell_store.get(&pos) {
                        Some(entity) => match visibility_query.get_mut(*entity) {
                            Ok(mut vis) => vis.visible = false,
                            Err(e) => error!("Error: {:?}", e),
                        },
                        None => (),
                    }
                }

                player.positions_visible = visible;
            }
            Position::Area(_) => panic!("oops, unimplemented"),
        }
    }
}
