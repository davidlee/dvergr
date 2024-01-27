// pub(crate) const TILEMAP_ASSET_PATH: &str = "img/or16w_t.png";
pub(crate) const SPRITESHEET_ASSET_PATH: &str = "vettlingr/dwarves.png";
pub(crate) const TILE_SIZE_W: f32 = 32.0;
pub(crate) const TILE_SIZE_H: f32 = 32.0;
pub(crate) const SPRITE_SCALE: f32 = 0.6;

pub(crate) mod anim;
pub(crate) mod init_map;
pub(crate) mod move_anim;
pub(crate) mod player_avatar;
pub(crate) mod torchlight;

use bevy::prelude::*;
pub(crate) use init_map::spawn_voxel_map;
pub(crate) use player_avatar::spawn_player_sprite_and_2d_camera;

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
