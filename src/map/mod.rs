use crate::board::{BoardRes, Cell, Pos3d};
use crate::AppState;
use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
pub struct TileMap {
    pub tile_size: TileSize,
    pub grid_size: GridSize,
    pub dimensions: PixelSize,
    pub center_offset: PixelPos,
}

impl TileMap {
    fn tile_offset(&self, x: i32, y: i32) -> PixelPos {
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
#[allow(dead_code)]
pub struct PixelPos {
    pub x: f32,
    pub y: f32,
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
        42,
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

const I_FLOOR: usize = 843;
const I_WALL: usize = 0;

fn texture_index_for_cell(cell: &Cell) -> usize {
    if cell.passable() {
        I_FLOOR
    } else {
        I_WALL
    }
}

pub fn spawn_map(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    tileset: Res<RPGTileset>,
    br: Res<BoardRes>,
) {
    let tile_map = TileMap::new(
        TileSize {
            width: 24.0,
            height: 24.0,
        },
        GridSize {
            width: br.size().width,
            height: br.size().height,
        },
    );

    println!("SPAWN MAP {:?}", tile_map);
    println!("from BOARD {:?}", br);
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
            for iy in 0..tile_map.grid_size.height {
                for ix in 0..tile_map.grid_size.width {
                    let pos = Pos3d { x: ix, y: iy, z: 0 }; // FIXME z axis
                    if let Some(cell) = br.board.get(&pos) {
                        let ti = texture_index_for_cell(cell);
                        let sprite = TextureAtlasSprite::new(ti);
                        let PixelPos { x, y } = tile_map.tile_offset(ix, iy);
                        tm.spawn(SpriteSheetBundle {
                            texture_atlas: tileset.atlas_handle.clone(),
                            sprite,
                            transform: Transform::from_xyz(x, y, 0.0),
                            ..default()
                        });
                    } else {
                        println!("missing cell: {:?}", pos);
                    }
                }
            }
        });
}
