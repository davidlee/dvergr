//
use crate::graphics::typical::*;
use crate::typical::*;
use bevy::prelude::*;

// use super::tilemap;

pub fn draw_weird_lines(
    mut gizmos: Gizmos,
    tile_map_q: Query<&TileMap>,
    player_q: Query<(Entity, &Player, &Locus)>,
    clock: Res<Clock>,
) {
    if let Ok((_e, _player, locus)) = player_q.get_single() {
        let tile_map = tile_map_q.get_single().unwrap();

        if let Position::Point(pos) = locus.position {
            let a = (clock.current_frame() % 120) as f32 / 120.0;

            // // Draw a box around the player
            let origin = tile_map.translate(pos);
            let size = Vec2::new(TILE_SIZE_W * 3.0, TILE_SIZE_H * 3.0);
            gizmos.rect_2d(origin, 0.0, size, Color::ORANGE_RED.with_a(a));

            // draw a line to somewhere
            //
            // gizmos.line_2d(
            //     tile_map.translate(pos),
            //     Vec2::new(0., 0.),
            //     Color::RED.with_a(a),
            // );
        }
    }
}
