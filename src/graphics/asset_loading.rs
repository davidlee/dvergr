use super::typical::*;
use crate::typical::*;

// use super::SPRITESHEET_ASSET_PATH;
// use super::TILEMAP_ASSET_PATH;

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
                super::tilemap::load_tileset,
                super::mobs::load_spritesheet.after(super::tilemap::load_tileset),
            ),
        )
        .add_systems(
            PostUpdate,
            ensure_assets_loaded.run_if(state_exists_and_equals(AppState::LoadAssets)),
        );
    }
}

#[derive(Resource, Debug)]
pub struct AssetsLoading {
    pub assets: Vec<Handle<Image>>,
    pub count: usize,
}

impl AssetsLoading {
    pub fn init_done(&self) -> bool {
        self.count == 2 // hax
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
