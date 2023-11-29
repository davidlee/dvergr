// use crate::typical::*;

pub const TILEMAP_ASSET_PATH: &str = "img/or16w_t.png";
pub const SPRITESHEET_ASSET_PATH: &str = "vettlingr/dwarves.png";

pub mod tilemap;
pub use tilemap::TileMap;

pub mod mobs;
pub use mobs::DwarfSpritesheet;

pub mod asset_loading;

pub mod components;
pub use components::*;

pub mod typical {
    pub use super::asset_loading::AssetsLoading;
    pub use crate::graphics::mobs::PlayerAvatar;
    pub use crate::graphics::{GridSize, PixelPos, PixelSize, Stage, TileMap, TileSize};
    // pub use bevy::prelude::get_entity;
    pub use bevy::prelude::{
        AssetServer, Assets, Color, Handle, Image, SpatialBundle, Sprite, SpriteBundle,
        SpriteSheetBundle, TextureAtlas, TextureAtlasBuilder, TextureAtlasSprite, Transform,
    };
}

pub fn draw_weird_lines() {}
