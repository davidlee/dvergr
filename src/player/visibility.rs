// use bevy::reflect::List;

use crate::board::geometry::*;
use crate::board::shadowcast::*;
use crate::typical::*;

const PLAYER_VISIBILITY_RANGE: f32 = 24.5; // FIXME add light sources

// https://www.albertford.com/shadowcasting/#symmetry
// https://www.roguebasin.com/index.php/FOV_using_recursive_shadowcasting

/*
TODO:
-----
// +++ symmetric shadowcasting

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
                // lighting:
                //
                let circle = circle(pos, PLAYER_VISIBILITY_RANGE);

                // field of view:
                //
                let visible_sector: HashSet<[i32; 2]> = sector_facing(locus.facing, &pos, circle);

                // line of sight:
                //
                let mut los_vec: Vec<[i32; 2]> =
                    compute_fov_2d_recursive([pos.x, pos.y], &board.wall_store.as_hashset2d());

                // warn!("{:?}", &board.wall_store.as_hashset2d());

                let mut los_set: HashSet<[i32; 2]> = HashSet::new();
                los_set
                    .try_reserve(los_vec.len())
                    .expect("failed allocation");
                while los_vec.len() > 0 {
                    // warn!("{:?}", los_vec);
                    los_set.insert(los_vec.pop().expect("should not panic"));
                }

                let new_vis = los_set;
                //let mut new_vis: HashSet<[i32; 2]> = visible_sector.clone();

                // for xy in los_set.difference(&visible_sector) {
                //     let res = new_vis.remove(xy);
                //     warn!("{:?} => {:?}", xy, res);
                // }

                let old_vis = player.positions_visible.to_owned(); // FIXME

                for arr_pos in new_vis.difference(&old_vis) {
                    let pos = IVec3::from_array([arr_pos[0], arr_pos[1], pos.z]);
                    match board.cell_store.get(&pos) {
                        Some(entity) => match visibility_query.get_mut(*entity) {
                            Ok(mut vis) => (vis.seen, vis.visible) = (true, true),
                            Err(e) => error!("Error: {:?}", e),
                        },
                        None => (),
                    }
                }

                for arr_pos in old_vis.difference(&new_vis) {
                    let pos = IVec3::from_array([arr_pos[0], arr_pos[1], pos.z]);
                    match board.cell_store.get(&pos) {
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
