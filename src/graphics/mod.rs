use crate::typical::*;

pub mod tilemap;
pub use tilemap::TileMap;

pub mod mobs;
pub use mobs::DwarfSpritesheet;

pub mod typical {
    pub use crate::graphics::mobs::PlayerAvatar;
    pub use crate::graphics::{
        AssetsLoading, GridSize, PixelPos, PixelSize, Stage, TileMap, TileSize,
    };
    // pub use bevy::prelude::get_entity;
    pub use bevy::prelude::{
        AssetServer, Assets, Color, Handle, Image, SpatialBundle, Sprite, SpriteBundle,
        SpriteSheetBundle, TextureAtlas, TextureAtlasBuilder, TextureAtlasSprite, Transform,
    };
}
use typical::*;

#[derive(Component, Debug, Copy, Clone)]
pub struct PixelSize {
    pub width: f32,
    pub height: f32,
}
pub type TileSize = PixelSize;

#[derive(Component, Debug, Copy, Clone)]
pub struct GridSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Component, Debug, Copy, Clone)]
pub struct PixelPos {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource, Debug)]
pub struct AssetsLoading {
    pub assets: Vec<Handle<Image>>,
    pub count: usize,
}

impl AssetsLoading {
    pub fn init_done(&self) -> bool {
        return self.count == 2; // hax
    }
}

#[derive(Component, Debug, Default)]
pub struct Stage;

#[derive(Bundle, Debug, Default)]
pub struct StageBundle {
    stage: Stage,
}

pub fn spawn_stage(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,

    state: Res<State<AppState>>,
) {
    println!("[AppState::InitStage] spawn_stage");
    commands.spawn((StageBundle::default(), SpatialBundle::default()));

    match state.get() {
        AppState::InitStage => next_state.set(AppState::LoadAssets),
        s => panic!("illegal state: {:?}", s),
    }
}

pub const TILEMAP_ASSET_PATH: &str = "img/or16w_t.png";
pub const SPRITESHEET_ASSET_PATH: &str = "vettlingr/dwarves.png";

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetsLoading {
            assets: vec![],
            count: 0,
        })
        .add_systems(
            OnEnter(AppState::InitAssets),
            (
                tilemap::load_tileset,
                mobs::load_spritesheet.after(tilemap::load_tileset),
            ),
        )
        .add_systems(
            PostUpdate,
            ensure_assets_loaded.run_if(state_exists_and_equals(AppState::LoadAssets)),
        );
    }
}

// TODO actually check asset loading
pub fn ensure_assets_loaded(
    mut _commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
) {
    match state.get() {
        AppState::LoadAssets => next_state.set(AppState::InitUI),
        s => panic!("illegal state: {:?}", s),
    }
}
