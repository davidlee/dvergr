use super::LogicalGraphicalEntityMapper;
use crate::typical::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_turborand::prelude::*;
use bevy_turborand::GlobalChaChaRng;
use bevy_turborand::RngComponent;

use crate::graphics::typical::*;
// use crate::TorchMarker;
use crate::TorchSecondaryLightMarker;

#[derive(Component, Debug, Default)]
pub struct PlayerAvatar;

#[derive(Resource, Debug, Clone)]
pub struct PlayerAvatarRes {
    pub entity: Entity,
}

use super::SPRITESHEET_ASSET_PATH;
const TILE_SIZE_W: f32 = 32.0;
const TILE_SIZE_H: f32 = 32.0;
const SPRITE_SCALE: f32 = 0.6;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut ev_writer: EventWriter<AppInitEvent>,
    avatar_ref: Res<PlayerAvatarRes>,
    mut mapper: ResMut<LogicalGraphicalEntityMapper>,
    player_query: Query<(Entity, &Player)>,
) {
    let texture_handle: Handle<Image> = asset_server.load(SPRITESHEET_ASSET_PATH);
    let vec2 = Vec2::new(TILE_SIZE_W, TILE_SIZE_H);
    let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), vec2, 56, 42, None, None);
    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    commands.insert_resource(DwarfSpritesheet {
        atlas_handle: texture_atlas_handle.clone(),
    });

    let player_entity = player_query.get_single().unwrap().0;
    let avatar_entity = avatar_ref.entity;

    mapper.insert(&player_entity, &avatar_entity);

    commands
        .get_entity(avatar_entity)
        .expect("no avatar, nowhere for sprite")
        .with_children(|avatar| {
            avatar.spawn((SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_xyz(0., 0., 0.),
                //.with_scale(Vec3::new(10.,10.,10.)),
                ..default()
            },));

            avatar.spawn(Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., -1.).with_scale(Vec3::splat(SPRITE_SCALE)),
                camera: Camera {
                    order: 1,
                    ..default()
                },
                ..default()
            });
        });
    ev_writer.send(AppInitEvent::SetAppState(AppState::Ready));
}

// TODO get parent, use attributes for intensity range
#[allow(unused, unreachable_code)]
pub fn flicker_torches(
    mut commands: Commands,
    // primary_query: Query<(Entity, &TorchMarker, &PointLight)>,
    mut secondary_query: Query<(
        Entity,
        &TorchSecondaryLightMarker,
        &Parent,
        Option<&mut PointLight>,
    )>,
    mut global_rng: ResMut<GlobalChaChaRng>,
) {
    // return;
    let mut rng = RngComponent::from(&mut global_rng);

    for x in secondary_query.iter_mut() {
        if rng.usize(0..10) < 6 {
            return;
        }

        let a = rng.usize(250..850) as f32;
        let b = rng.usize(250..850) as f32;

        let intensity = a + b;

        if let Some(mut light) = x.3 {
            light.intensity = intensity;
        } else {
            commands.entity(x.0).insert((SpotLightBundle {
                spot_light: SpotLight {
                    intensity,
                    range: 120.,
                    shadows_enabled: true,
                    color: Color::GOLD,
                    outer_angle: 1.5,
                    inner_angle: 0.2,
                    ..default()
                },
                ..default()
            },));
        }
    }
}
