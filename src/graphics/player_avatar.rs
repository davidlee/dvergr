use super::*;
use crate::typical::*;

pub fn spawn_player_sprite_and_2d_camera(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    player_query: Query<(Entity, &Player)>,
) {
    let texture_handle: Handle<Image> = asset_server.load(SPRITESHEET_ASSET_PATH);
    let vec2 = Vec2::new(TILE_SIZE_W, TILE_SIZE_H);
    let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), vec2, 56, 42, None, None);
    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    commands.insert_resource(DwarfSpritesheet {
        atlas_handle: texture_atlas_handle.clone(),
    });

    let player_entity = player_query.single().0;

    commands
        .get_entity(player_entity)
        .expect("no player for sprite")
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
                },
                transform: Transform::from_xyz(0., 0., -1.).with_scale(Vec3::splat(SPRITE_SCALE)),
                camera: Camera {
                    order: 1,
                    ..default()
                },
                ..default()
            });
        });
}
