pub const TILEMAP_ASSET_PATH: &str = "img/or16w_t.png";
pub const SPRITESHEET_ASSET_PATH: &str = "vettlingr/dwarves.png";

pub(crate) mod anim;
pub(crate) mod init_map;
pub(crate) mod move_anim;
pub(crate) mod player_avatar;

use bevy::{prelude::*, utils::HashMap};

pub(crate) mod typical {
    pub use super::anim::LerpVec3;
    pub use super::{CreatureEntityRef, DwarfSpritesheet, PlayerAvatar, PlayerAvatarRes};
    pub use bevy::prelude::{
        AssetServer, Assets, Color, Handle, Image, SpatialBundle, Sprite, SpriteBundle,
        SpriteSheetBundle, TextureAtlas, TextureAtlasBuilder, TextureAtlasSprite, Transform,
    };
}

#[derive(Component, Debug, Default)]
pub struct PlayerAvatar;

#[derive(Resource, Debug, Clone)]
pub struct PlayerAvatarRes {
    pub entity: Entity,
}

#[derive(Component, Debug)]
pub struct MapMarker;

#[derive(Component, Debug)]
pub struct TorchMarker;

#[derive(Component, Debug)]
pub struct TorchSecondaryLightMarker;

#[derive(Component, Debug)]
pub struct CameraMarker;

#[derive(Resource, Debug)]
pub struct DwarfSpritesheet {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Component, Debug)]
pub struct CreatureEntityRef(pub Entity);

#[derive(Resource, Debug, Default)]
pub struct LogicalGraphicalEntityMapper {
    logical_to_graphical: HashMap<Entity, Entity>,
    graphical_to_logical: HashMap<Entity, Entity>,
}

impl LogicalGraphicalEntityMapper {
    pub fn new() -> Self {
        LogicalGraphicalEntityMapper {
            logical_to_graphical: HashMap::new(),
            graphical_to_logical: HashMap::new(),
        }
    }

    pub fn graphical_entity(&self, logical_entity: &Entity) -> Option<&Entity> {
        self.logical_to_graphical.get(logical_entity)
    }

    pub fn logical_entity(&self, sprite_entity: &Entity) -> Option<&Entity> {
        self.graphical_to_logical.get(sprite_entity)
    }

    pub fn insert(&mut self, logical_entity: &Entity, sprite_entity: &Entity) {
        self.logical_to_graphical
            .insert(*logical_entity, *sprite_entity);

        self.graphical_to_logical
            .insert(*sprite_entity, *logical_entity);
    }
}
