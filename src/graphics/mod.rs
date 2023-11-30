pub const TILEMAP_ASSET_PATH: &str = "img/or16w_t.png";
pub const SPRITESHEET_ASSET_PATH: &str = "vettlingr/dwarves.png";

pub mod asset_loading;
pub mod components;
pub mod mobs;
pub mod move_anim;
pub mod player_avatar;
pub mod playground;
pub mod tilemap;

pub mod typical {
    pub use super::asset_loading::AssetsLoading;
    pub use super::components::{GridSize, PixelSize, Stage, TileSize};
    pub use super::mobs::{CreatureEntityRef, DwarfSpritesheet};
    pub use super::player_avatar::PlayerAvatar;
    pub use super::tilemap::{TileMap, TILE_SIZE_H, TILE_SIZE_W};
    pub use super::transform_from_tilemap_pos;
    pub use bevy::prelude::{
        AssetServer, Assets, Color, Handle, Image, SpatialBundle, Sprite, SpriteBundle,
        SpriteSheetBundle, TextureAtlas, TextureAtlasBuilder, TextureAtlasSprite, Transform,
    };
}
use crate::typical::*;
use typical::*;

pub use playground::draw_weird_lines;

pub fn transform_from_tilemap_pos(tile_map: &TileMap, pos: &IVec3) -> Transform {
    let p = tile_map.tile_offset(pos.x, pos.y);

    Transform::from_xyz(
        p.x + tile_map.center_offset.x,
        p.y + tile_map.center_offset.y,
        1.0,
    )
}
