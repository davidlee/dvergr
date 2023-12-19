use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::CameraRenderGraph;
use bevy::render::render_resource::{BlendState, LoadOp};
use bevy::render::view::VisibleEntities;

use crate::graphics::typical::*;
use crate::typical::*;

#[derive(Component, Debug, Default)]
pub struct PlayerAvatar;

#[derive(Bundle, Debug, Default)]
pub struct PlayerAvatarBundle {
    avatar: PlayerAvatar,
}

use super::SPRITESHEET_ASSET_PATH;
const TILE_SIZE_W: f32 = 32.0;
const TILE_SIZE_H: f32 = 32.0;

pub fn spawn(
    mut commands: Commands,
    // sprites: Res<DwarfSpritesheet>,
    board: Res<Board>,
    mut map_query: Query<(Entity, &MapMarker)>,
    player_query: Query<(Entity, &Player)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut ev_writer: EventWriter<AppInitEvent>,
) {
    let transform: Transform;
    let (player_entity, ..) = player_query.single();
    {
        let pos: IVec3 = board
            .creature_store
            .get_pos_for(&player_entity)
            .expect("player sprite needs somewhere to go")
            .to_owned();
        let [x, y, _] = pos.as_vec3().to_array();
        transform = Transform::from_xyz(x, y, 0.);
    }

    let texture_handle: Handle<Image> = asset_server.load(SPRITESHEET_ASSET_PATH);
    let vec2 = Vec2::new(TILE_SIZE_W, TILE_SIZE_H);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle.clone(), vec2, 56, 42, None, Some(vec2));
    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    commands.insert_resource(DwarfSpritesheet {
        atlas_handle: texture_atlas_handle.clone(),
    });

    commands
        .get_entity(map_query.single_mut().0)
        .expect("no map, no player")
        .with_children(|s| {
            s.spawn((
                PlayerAvatarBundle::default(),
                CreatureEntityRef(player_entity),
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(0),
                    transform,
                    ..default()
                },
            ));
            s.spawn(Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., -40.),

                ..default()
            });
        });

    ev_writer.send(AppInitEvent::SetAppState(AppState::Ready));
}
