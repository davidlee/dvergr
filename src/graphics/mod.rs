pub const TILEMAP_ASSET_PATH: &str = "img/or16w_t.png";
pub const SPRITESHEET_ASSET_PATH: &str = "vettlingr/dwarves.png";

pub mod anim;
pub mod components;
// pub mod mobs;
pub mod move_anim;
pub mod player_avatar;
// pub mod player_decoration;
use bevy::prelude::*;

pub mod typical {
    pub use super::anim::{LerpVec3, Lerpf32, SimpleTimer};
    pub use super::components::{GridSize, PixelSize, TileSize};
    pub use super::{CreatureEntityRef, DwarfSpritesheet};
    pub use bevy::prelude::{
        AssetServer, Assets, Color, Handle, Image, SpatialBundle, Sprite, SpriteBundle,
        SpriteSheetBundle, TextureAtlas, TextureAtlasBuilder, TextureAtlasSprite, Transform,
    };
}

#[derive(Resource, Debug)]
pub struct DwarfSpritesheet {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Component, Debug)]
pub struct CreatureEntityRef(pub Entity);
