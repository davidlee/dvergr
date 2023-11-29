//
use crate::graphics::typical::*;
use crate::typical::*;
use bevy::prelude::*;
// use std::f32::consts::PI;

pub fn draw_weird_lines(
    //
    mut gizmos: Gizmos,
    mut commands: Commands,
    // board: Res<Board>,
    tile_map_q: Query<&TileMap>,
    player_q: Query<(Entity, &Player, &Locus)>,
) {
    if let Ok((e, _player, locus)) = player_q.get_single() {
        // println!("??? {:?}", player);
        commands.entity(e).log_components();

        let tile_map = tile_map_q.get_single().unwrap();

        match locus.position {
            Position::Point(pos) => {
                let c = tile_map.center_offset;
                let t = tile_map.tile_offset(pos.x, pos.y);
                let start = Vec2::new(c.x + t.x, c.y + t.y);
                let end_vec2 = Vec2::new(500., 450.);
                gizmos.line_2d(start, end_vec2, Color::RED);
            }
            Position::Area(_) => {}
        }
    }
}
