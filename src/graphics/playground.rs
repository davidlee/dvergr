//
use crate::graphics::typical::*;
use crate::typical::*;
use bevy::prelude::*;
// use std::f32::consts::PI;
use bevy_turborand::prelude::*;

pub fn draw_weird_lines(
    mut gizmos: Gizmos,
    tile_map_q: Query<&TileMap>,
    mut global_rng: ResMut<GlobalRng>,
    player_q: Query<(Entity, &Player, &Locus)>,
) {
    if let Ok((_e, player, locus)) = player_q.get_single() {
        let tile_map = tile_map_q.get_single().unwrap();
        // choose a random visible tile
        let visible_tiles: Vec<[i32; 3]> =
            player.positions_visible.to_owned().into_iter().collect();
        if visible_tiles.len() == 0 {
            return;
        }
        let n: usize = visible_tiles.len();
        let i = global_rng.usize(0..n);
        let rand_pos = IVec3::from(visible_tiles[i]);
        let c = tile_map.center_offset;
        let t = tile_map.tile_offset(rand_pos.x, rand_pos.y);
        let end = Vec2::new(c.x + t.x, c.y + t.y);

        match locus.position {
            Position::Point(pos) => {
                let c = tile_map.center_offset;
                let t = tile_map.tile_offset(pos.x, pos.y);
                let start = Vec2::new(c.x + t.x, c.y + t.y);
                gizmos.line_2d(start, end, Color::RED.with_a(0.3));
            }
            Position::Area(_) => {}
        }
    }
}
