//
use crate::graphics::typical::*;
use crate::typical::*;
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn render_gizmos(
    mut gizmos: Gizmos,
    tile_map_q: Query<&TileMap>,
    player_q: Query<(Entity, &Player, &Locus)>,
    clock: Res<Clock>,
) {
    if let Ok((_e, _player, locus)) = player_q.get_single() {
        let tile_map = tile_map_q.get_single().unwrap();

        if let Position::Point(pos) = locus.position {
            let player_centre = tile_map.translate(pos);

            for ray_vector in locus.facing.arc_vectors(1) {
                gizmos.ray_2d(
                    player_centre,
                    ray_vector * TILE_SIZE_H * 17.,
                    Color::rgba_u8(0, 200, 115, 75),
                );
            }

            let alpha = (clock.current_frame() % 120) as f32 / 120.0;

            // FOV
            gizmos
                .arc_2d(
                    player_centre,
                    locus.direction.angular_rotation(),
                    PI / 2.,
                    TILE_SIZE_W * 24. * alpha,
                    Color::GOLD.with_a(0.9),
                )
                .segments(36);

            // peripheral vision arc
            gizmos.arc_2d(
                player_centre,
                locus.direction.angular_rotation(),
                PI,
                TILE_SIZE_W * 6.,
                Color::ORANGE.with_a(0.25),
            );

            // Show the current light radius
            // TODO light sources modelled properly
            gizmos
                .circle_2d(player_centre, TILE_SIZE_W * 24.0, Color::GOLD.with_a(0.1))
                .segments(36);

            // Draw a box around the player
            gizmos.rect_2d(
                player_centre,
                0.0,
                Vec2::new(TILE_SIZE_W * 3.0, TILE_SIZE_H * 3.0),
                Color::ORANGE_RED.with_a(0.35),
            );

            // gizmos.rect_2d(
            //     player_centre,
            //     (clock.current_frame() % 180) as f32 / 180.0 * TAU,
            //     ,
            //     Color::ORANGE_RED.with_a(0.25),
            // );
        }
    }
}

#[allow(dead_code, unused_variables, unused_mut)]
pub fn render_rays(
    mut gizmos: Gizmos,
    tile_map_q: Query<&TileMap>,
    board: Res<Board>,
    player_q: Query<(Entity, &Player, &Locus)>,
    cells_q: Query<(Entity, &Cell)>,
    // clock: Res<Clock>,
) {
    // find all walls
    // determine whether there's an adjacent wall, use this to build vertices
}
