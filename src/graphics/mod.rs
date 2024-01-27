pub const TILEMAP_ASSET_PATH: &str = "img/or16w_t.png";
pub const SPRITESHEET_ASSET_PATH: &str = "vettlingr/dwarves.png";

pub(crate) mod anim;
pub(crate) mod init_map;
pub(crate) mod move_anim;
pub(crate) mod player_avatar;
pub(crate) mod torchlight;

use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct DwarfSpritesheet {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Component, Debug)]
pub struct CreatureEntityRef(pub Entity);
//
// marker components
//

#[derive(Component, Debug)]
pub struct MapMarker;

#[derive(Component, Debug)]
pub struct TorchMarker;

#[derive(Component, Debug)]
pub struct TorchSecondaryLightMarker;

#[derive(Component, Debug)]
pub struct CameraMarker;
