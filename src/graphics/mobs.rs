use super::typical::*;
use super::SPRITESHEET_ASSET_PATH;

use crate::typical::*;

const TILE_SIZE_W: f32 = 32.0;
const TILE_SIZE_H: f32 = 32.0;

#[derive(Resource, Debug)]
pub struct DwarfSpritesheet {
    #[allow(dead_code)]
    pub atlas_handle: Handle<TextureAtlas>,
}

pub fn load_spritesheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut loading: ResMut<AssetsLoading>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut ev_writer: EventWriter<AppInitEvent>,
) {
    trace!("loading SpriteSheet (characters)");

    let texture_handle: Handle<Image> = asset_server.load(SPRITESHEET_ASSET_PATH);
    let vec2 = Vec2::new(TILE_SIZE_W, TILE_SIZE_H);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle.clone(), vec2, 56, 42, None, Some(vec2));

    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    commands.insert_resource(DwarfSpritesheet {
        atlas_handle: texture_atlas_handle,
    });

    ev_writer.send(AppInitEvent::SetAppState(AppState::InitBoard));
}

#[derive(Component, Debug)]
pub struct CreatureEntityRef(pub Entity);
