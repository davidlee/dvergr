// pub(crate) const TILEMAP_ASSET_PATH: &str = "img/or16w_t.png";
pub(crate) const TILE_SIZE_W: f32 = 32.0;
pub(crate) const TILE_SIZE_H: f32 = 32.0;
pub(crate) const SPRITE_SCALE: f32 = 0.6;

pub(crate) mod anim;
pub(crate) mod init_map;
pub(crate) mod move_anim;
pub(crate) mod player_avatar;
pub(crate) mod sprites;
pub(crate) mod torchlight;

pub(crate) use init_map::spawn_voxel_map;
pub(crate) use sprites::*;
