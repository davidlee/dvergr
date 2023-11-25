use crate::board::{BoardRes, Cell, Pos3d};
use crate::AppState;
use bevy::prelude::*;

pub mod tilemap;
pub use tilemap::{TileMap, TileMapPlugin};

pub mod mobs;
pub use mobs::DwarfSpritesheet;

#[derive(Component, Debug, Copy, Clone)]
pub struct PixelSize {
    pub width: f32,
    pub height: f32,
}
type TileSize = PixelSize;

#[derive(Component, Debug, Copy, Clone)]
pub struct GridSize {
    pub width: i32,
    pub height: i32,
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

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_stage);
    }
}

fn spawn_stage(mut commands: Commands) {
    commands.spawn((StageBundle::default(), SpatialBundle::default()));
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
        .add_systems(Startup, mobs::load_spritesheet)
        .add_systems(Startup, tilemap::load_tileset)
        .add_systems(
            PostUpdate,
            ensure_assets_loaded.run_if(state_exists_and_equals(AppState::LoadAssets)),
        );
    }
}

pub fn ensure_assets_loaded(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut loading: ResMut<AssetsLoading>,
) {
    for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {
                println!("Asset Loaded .. {:?} -- {}", loading, id);
                // if loading.count > 0 {
                loading.count -= 1;
                if loading.count == 0 {
                    println!("Assets loaded, next state ... ");
                    state.set(AppState::DrawUI);
                    commands.remove_resource::<AssetsLoading>();
                }
                // } else {
                //     println!("stack underflow");
                // }
            }
            _ => (),
        }
    }
}
