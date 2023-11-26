use super::AssetsLoading;
use super::Stage;
use super::SPRITESHEET_ASSET_PATH;
use super::*;
use crate::board::Board;
use crate::board::*;
use crate::creature::Creature;
use crate::player::Player;
use crate::state::AppState;
// use bevy::prelude::*;

const TILE_SIZE_W: f32 = 32.0;
const TILE_SIZE_H: f32 = 32.0;

#[derive(Resource, Debug)]
pub struct DwarfSpritesheet {
    #[allow(dead_code)]
    atlas_handle: Handle<TextureAtlas>,
}

pub fn load_spritesheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
) {
    println!("load SPRITESHEET");

    let texture_handle: Handle<Image> = asset_server.load(SPRITESHEET_ASSET_PATH);
    let vec2 = Vec2::new(TILE_SIZE_W, TILE_SIZE_H);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle.clone(), vec2, 56, 42, None, Some(vec2));

    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    commands.insert_resource(DwarfSpritesheet {
        atlas_handle: texture_atlas_handle,
    });

    // TODO
    // this is a bit janky and not very DRY
    // improve the asset loading strategy
    loading.assets.push(texture_handle);
    loading.count += 1;

    match state.get() {
        AppState::InitAssets => next_state.set(AppState::InitBoard),
        s => panic!("illegal state: {:?}", s),
    }
}

#[derive(Component, Debug, Default)]
pub struct PlayerAvatar;

#[derive(Bundle, Debug, Default)]
pub struct PlayerAvatarBundle {
    avatar: PlayerAvatar,
}

pub fn transform_from_tilemap_pos(tile_map: &TileMap, pos: &Pos3d) -> Transform {
    let p = tile_map.tile_offset(pos.x, pos.y);

    Transform::from_xyz(
        p.x + tile_map.center_offset.x,
        p.y + tile_map.center_offset.y,
        1.0,
    )
}

pub fn spawn_player_sprite(
    mut commands: Commands,
    sprites: Res<DwarfSpritesheet>,
    board: Res<Board>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
    mut stage_query: Query<(Entity, &Stage)>,
    player_query: Query<(Entity, &Player, &Creature)>,
    tile_map_query: Query<&TileMap>,
) {
    let transform: Transform;
    {
        let tile_map = tile_map_query.single();
        let (entity, ..) = player_query.single();
        // FIXME this is ugly but :shrug: seems necessary
        let pos: Pos3d = board.creatures.get_pos_for(&entity).unwrap().to_owned();
        transform = transform_from_tilemap_pos(&tile_map, &pos);
    }

    commands
        .get_entity(stage_query.single_mut().0) // Stage entity
        .unwrap()
        .with_children(|s| {
            s.spawn((
                PlayerAvatarBundle::default(),
                SpriteSheetBundle {
                    texture_atlas: sprites.atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    // transform: Transform::from_xyz(0., 0., 1.),
                    transform,
                    ..default()
                },
            ));
        });

    match state.get() {
        AppState::InitMobs => next_state.set(AppState::Game),
        s => panic!("illegal state: {:?}", s),
    }
}

pub fn update_changed() {}
