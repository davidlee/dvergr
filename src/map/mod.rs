use crate::AppState;
use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
pub struct TileMap {
    tile_size: TileSize,
    grid_size: GridSize,
    dimensions: PixelSize,
    center_offset: PixelPos,
}

impl TileMap {
    fn tile_offset(&self, x: usize, y: usize) -> PixelPos {
        let x = self.tile_size.width * x as f32;
        let y = self.tile_size.height * y as f32;
        PixelPos { x, y }
    }

    pub fn new(tile_size: TileSize, grid_size: GridSize) -> Self {
        let width = tile_size.width * grid_size.width as f32;
        let height = tile_size.height * grid_size.height as f32;

        TileMap {
            tile_size,
            grid_size,
            dimensions: PixelSize { width, height },
            center_offset: PixelPos {
                x: 0. - width / 2.,
                y: 0. - height / 2.,
            },
        }
    }
}

#[derive(Component, Debug, Copy, Clone)]
pub struct PixelSize {
    width: f32,
    height: f32,
}
type TileSize = PixelSize;

#[derive(Component, Debug, Copy, Clone)]
pub struct GridSize {
    width: usize,
    height: usize,
}

#[derive(Component, Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct PixelPos {
    x: f32,
    y: f32,
}
///

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_map)
            .add_systems(OnEnter(AppState::LoadAssets), load_tileset);
    }
}

#[derive(Resource, Debug)]
pub struct RPGTileset {
    atlas_handle: Handle<TextureAtlas>,
}

///

pub fn load_tileset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("img/or16w_t.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(24.0, 24.0),
        56,
        2,
        None,
        Some(Vec2 { x: 24.0, y: 24.0 }),
    );
    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    commands.insert_resource(RPGTileset {
        atlas_handle: texture_atlas_handle,
    });

    next_state.set(AppState::DrawUI);

    // let map_entity = commands
}

pub fn spawn_map(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    tileset: Res<RPGTileset>,
) {
    println!("DRAW MAP");

    let tile_map = TileMap::new(
        TileSize {
            width: 24.0,
            height: 24.0,
        },
        GridSize {
            width: 24,
            height: 16,
        },
    );

    commands
        .spawn((
            tile_map.clone(),
            SpatialBundle {
                transform: Transform::from_xyz(
                    tile_map.center_offset.x,
                    tile_map.center_offset.y,
                    0.,
                ),
                ..default()
            },
        ))
        .with_children(|tm| {
            for iy in 0..16 {
                for ix in 0..24 {
                    let PixelPos { x, y } = tile_map.tile_offset(ix, iy);

                    tm.spawn((SpriteSheetBundle {
                        texture_atlas: tileset.atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(0),
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.0),
                            scale: Vec3::splat(1.0),
                            ..default()
                        },
                        ..default()
                    },));
                }
            }
        });
}
