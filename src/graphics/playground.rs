//
use crate::graphics::typical::*;
use crate::typical::*;
use bevy::prelude::*;
// use std::f32::consts::PI;

pub fn draw_weird_lines(
    //
    mut gizmos: Gizmos,
    mut commands: Commands,
    board: Res<Board>,
    tile_map_q: Query<&TileMap>,
    player_q: Query<(Entity, &Player)>,
) {
    if let Ok((e, player)) = player_q.get_single() {
        println!("??? {:?}", player);
        commands.entity(e).log_components();

        // match locus.position {
        //     Position::Point(pos) => {
        //         let tile_map = tile_map_q.get_single().unwrap();
        //         let start = tile_map.tile_offset(pos.x, pos.y);
        //         // let start_vec2 = Vec2::new(50., 150.);
        //         let end_vec2 = Vec2::new(500., 450.);
        //         gizmos.line_2d(start, end_vec2, Color::RED);
        //     }
        //     Position::Area(_) => {}
        // }
    }
}
