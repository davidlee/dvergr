use super::AssetsLoading;
use super::Stage;
use super::SPRITESHEET_ASSET_PATH;
// use crate::board::BoardRes;
use crate::state::AppState;
use bevy::prelude::*;

const TILE_SIZE_W: f32 = 32.0;
const TILE_SIZE_H: f32 = 32.0;

pub struct MobsPlugin;

impl Plugin for MobsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InitMobs), spawn_player_sprite);
    }
}

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

pub fn spawn_player_sprite(
    mut commands: Commands,
    sprites: Res<DwarfSpritesheet>,
    // board: Res<BoardRes>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
    mut stage_query: Query<(Entity, &Stage)>,
) {
    // TODO we need to spawn -- and perhaps, to separately maintain -- a logical Player
    // distinct from the graphical representation
    // who should also have a presence in the Board .. probably in the Cell.Creature struct

    // find the x,y based on position on the Board
    // then translate & render above

    // ..
    let (e, _stage) = stage_query.single_mut();
    let mut stage_entity = commands.get_entity(e).unwrap();

    stage_entity.with_children(|s| {
        s.spawn((
            PlayerAvatarBundle::default(),
            SpriteSheetBundle {
                texture_atlas: sprites.atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },
        ));
    });

    match state.get() {
        AppState::InitMobs => next_state.set(AppState::Game),
        s => panic!("illegal state: {:?}", s),
    }
}
