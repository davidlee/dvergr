use crate::typical::graphics::*;

const DWARF_SPRITESHEET_ASSET_PATH: &str = "vettlingr/dwarves.png";
const GOBLIN_SPRITESHEET_ASSET_PATH: &str = "vettlingr/goblins.png";

#[derive(Resource, Debug)]
pub struct DwarfSpritesheet {
    pub atlas_handle: Handle<TextureAtlas>,
}
#[derive(Resource, Debug)]
pub struct GoblinSpritesheet {
    pub atlas_handle: Handle<TextureAtlas>,
}

pub fn load_spritesheets(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let dwarf_texture_handle: Handle<Image> = asset_server.load(DWARF_SPRITESHEET_ASSET_PATH);
    let dwarf_texture_atlas = TextureAtlas::from_grid(
        dwarf_texture_handle.clone(),
        Vec2::new(TILE_SIZE_W, TILE_SIZE_H),
        56,
        42,
        None,
        None,
    );
    let dwarf_texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(dwarf_texture_atlas);
    commands.insert_resource(DwarfSpritesheet {
        atlas_handle: dwarf_texture_atlas_handle,
    });

    let goblin_texture_handle: Handle<Image> = asset_server.load(GOBLIN_SPRITESHEET_ASSET_PATH);
    let goblin_texture_atlas = TextureAtlas::from_grid(
        goblin_texture_handle.clone(),
        Vec2::new(TILE_SIZE_W, TILE_SIZE_H),
        56,
        42,
        None,
        None,
    );
    let goblin_texture_atlas_handle: Handle<TextureAtlas> =
        texture_atlases.add(goblin_texture_atlas);
    commands.insert_resource(GoblinSpritesheet {
        atlas_handle: goblin_texture_atlas_handle,
    });
    warn!("wat");
}

pub fn spawn_player_sprite_and_2d_camera(
    mut commands: Commands,
    dwarf: Res<DwarfSpritesheet>,
    player_query: Query<(Entity, &Player)>,
) {
    let player_entity = player_query.single().0;

    commands
        .get_entity(player_entity)
        .expect("no player for sprite")
        .with_children(|avatar| {
            avatar.spawn((SpriteSheetBundle {
                texture_atlas: dwarf.atlas_handle.clone(),
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
