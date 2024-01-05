use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;

use crate::graphics::typical::*;
use crate::typical::*;

#[derive(Component, Debug, Default)]
pub struct PlayerAvatar;

#[derive(Resource, Debug, Clone)]
pub struct PlayerAvatarRes {
    pub entity: Entity,
}

use super::SPRITESHEET_ASSET_PATH;
const TILE_SIZE_W: f32 = 32.0;
const TILE_SIZE_H: f32 = 32.0;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut ev_writer: EventWriter<AppInitEvent>,
    avatar_ref: Res<PlayerAvatarRes>,
) {
    let texture_handle: Handle<Image> = asset_server.load(SPRITESHEET_ASSET_PATH);
    let vec2 = Vec2::new(TILE_SIZE_W, TILE_SIZE_H);
    let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), vec2, 56, 42, None, None);
    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    commands.insert_resource(DwarfSpritesheet {
        atlas_handle: texture_atlas_handle.clone(),
    });

    let avatar_entity = avatar_ref.entity;
    commands
        .get_entity(avatar_entity)
        .expect("no avatar, nowhere for sprite")
        .with_children(|avatar| {
            avatar.spawn((SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },));

            avatar.spawn(Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., -40.),
                camera: Camera {
                    order: 1,
                    ..default()
                },
                ..default()
            });
        });
    ev_writer.send(AppInitEvent::SetAppState(AppState::Ready));
}
