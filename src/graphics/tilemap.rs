use super::TILEMAP_ASSET_PATH;
use crate::graphics::typical::*;
use crate::typical::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.07, 0.12, 0.18);

pub const TILE_MAP_Z: u32 = 0;
pub const DARK_MAP_Z: u32 = 0;

pub const TILE_SIZE_W: f32 = 24.0;
pub const TILE_SIZE_H: f32 = 24.0;
// TileMap
//
#[derive(Component, Debug, Copy, Clone, Resource)]
pub struct TileMap {
    pub tile_size: TileSize,
    pub grid_size: GridSize,
    pub dimensions: PixelSize,
    pub center_offset: PixelPos,
}

impl TileMap {
    pub fn tile_offset(&self, x: u32, y: u32) -> PixelPos {
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

// DarkMap (fog of war)
//

#[derive(Component, Debug, Copy, Clone, Resource)]
pub struct DarkMap {
    pub tile_size: TileSize,
    pub grid_size: GridSize,
    pub dimensions: PixelSize,
    pub center_offset: PixelPos,
}

impl From<TileMap> for DarkMap {
    fn from(tile_map: TileMap) -> Self {
        DarkMap {
            tile_size: tile_map.tile_size,
            grid_size: tile_map.grid_size,
            dimensions: tile_map.dimensions,
            center_offset: tile_map.center_offset,
        }
    }
}

// Resource

#[derive(Resource, Debug)]
pub struct Tileset {
    atlas_handle: Handle<TextureAtlas>,
}

// Functions

pub fn load_tileset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut next_state: ResMut<NextState<AppState>>,
    // state: Res<State<AppState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut loading: ResMut<AssetsLoading>,
) {
    println!("LOAD TILESET!");

    let vec2 = Vec2::new(TILE_SIZE_W, TILE_SIZE_H);
    let texture_handle: Handle<Image> = asset_server.load(TILEMAP_ASSET_PATH);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle.clone(), vec2, 56, 42, None, Some(vec2));
    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    commands.insert_resource(Tileset {
        atlas_handle: texture_atlas_handle,
    });

    // TODO
    // this is a bit janky and not very DRY
    // improve the asset loading strategy
    loading.assets.push(texture_handle);
    loading.count += 1;
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

// systems

pub fn render_darkmap_changes(
    // mut board_mut: ResMut<Board>,
    cell_query: Query<(Entity, &Cell), Changed<Cell>>,
) {
    for (_, cell) in cell_query.iter() {
        println!("CHANGED CELL {:?}", cell);

        // bugger, Cells don't belong to any Entity so we can't get them this way.
        // better fix that.
    }
}

pub fn spawn_tile_map(
    mut commands: Commands,
    tileset: Res<Tileset>,
    board: Res<Board>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
    mut stage_query: Query<(Entity, &Stage)>,
    cells_query: Query<(Entity, &mut Cell)>,
) {
    let tile_map = TileMap::new(
        TileSize {
            width: TILE_SIZE_W,
            height: TILE_SIZE_H,
        },
        GridSize {
            width: board.size.width,
            height: board.size.height,
        },
    );
    let dark_map = DarkMap::from(tile_map);

    let (stage_entity, _) = stage_query.single_mut();

    commands
        .get_entity(stage_entity)
        .unwrap()
        .with_children(|stage_entity| {
            // for tile_map / dark_map
            let transform =
                Transform::from_xyz(tile_map.center_offset.x, tile_map.center_offset.y, 0.);
            stage_entity
                .spawn((
                    tile_map,
                    SpatialBundle {
                        transform,
                        ..default()
                    },
                ))
                .with_children(|tiles| {
                    for iy in 0..tile_map.grid_size.height {
                        for ix in 0..tile_map.grid_size.width {
                            let pos = UVec3 {
                                x: ix,
                                y: iy,
                                z: TILE_MAP_Z,
                            };

                            if let Some(cell_entity) = board.cell_entities.get(&pos) {
                                let PixelPos { x, y } = tile_map.tile_offset(ix, iy);
                                let transform = Transform::from_xyz(x, y, TILE_MAP_Z as f32);
                                // is this terrible?
                                if let Ok(cell) = cells_query.get_component::<Cell>(*cell_entity) {
                                    let texture_index = texture_index_for_cell(cell);
                                    let sprite = TextureAtlasSprite::new(texture_index);

                                    tiles.spawn(SpriteSheetBundle {
                                        texture_atlas: tileset.atlas_handle.clone(),
                                        sprite,
                                        transform,
                                        ..default()
                                    });
                                }
                            } else {
                                println!("missing cell: {:?}", pos);
                                panic!("!!");
                            }
                        }
                    }
                });
            stage_entity
                .spawn((
                    dark_map,
                    SpatialBundle {
                        transform, // same as tile_map's
                        ..default()
                    },
                ))
                .with_children(|dark| {
                    for iy in 0..dark_map.grid_size.height {
                        for ix in 0..dark_map.grid_size.width {
                            let PixelPos { x, y } = tile_map.tile_offset(ix, iy);
                            let transform = Transform::from_xyz(x, y, DARK_MAP_Z as f32);
                            dark.spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: BACKGROUND_COLOR,
                                    custom_size: Some(Vec2::new(TILE_SIZE_W, TILE_SIZE_H)),
                                    ..default()
                                },
                                transform,
                                ..default()
                            });
                            // create a square
                        }
                    }
                });
        });

    match state.get() {
        AppState::InitTileMap => next_state.set(AppState::InitMobs),
        s => panic!("illegal state: {:?}", s),
    }
}
