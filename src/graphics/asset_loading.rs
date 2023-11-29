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
        });
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
pub fn ensure_assets_loaded(mut _commands: Commands, mut ev_writer: EventWriter<AppInitEvent>) {
    println!("faked out asset loading complete");
    ev_writer.send(AppInitEvent::SetAppState(AppState::InitUI));
}
