use bevy::window::WindowResolution;

// #[doc(inline)]
pub use bevy_ecs_tilemap::helpers::square_grid::neighbors::SquareDirection;
pub use bevy_ecs_tilemap::prelude::*;

pub fn get_tilemap_size(resolution: &WindowResolution, tile_size: &TilemapTileSize) -> TilemapSize {
    let w: u32 = resolution.width() as u32;
    let h: u32 = resolution.height() as u32;

    let x: u32 = w / tile_size.x as u32;
    let y: u32 = h / tile_size.y as u32;
    TilemapSize { x, y }
}
