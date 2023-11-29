use bevy::prelude::*;
use bevy::utils::HashMap;

use super::TILEMAP_ASSET_PATH;
use crate::graphics::typical::*;
use crate::typical::*;

// const BACKGROUND_COLOR: Color = Color::rgb(0.07, 0.12, 0.18);

pub const TILE_MAP_Z: i32 = 0;
pub const DARK_MAP_Z: i32 = 0;

pub const TILE_SIZE_W: f32 = 24.0;
pub const TILE_SIZE_H: f32 = 24.0;

// TileMap
//
#[derive(Component, Debug, Clone, Resource)]
pub struct TileMap {
    pub tile_size: TileSize,
    pub grid_size: GridSize,
    pub dimensions: PixelSize,
    pub center_offset: PixelPos,
    pub entities: HashMap<IVec3, Entity>,
}

impl TileMap {
    pub fn tile_offset(&self, x: i32, y: i32) -> PixelPos {
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
            entities: HashMap::new(),
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let vec2 = Vec2::new(TILE_SIZE_W, TILE_SIZE_H);
    let texture_handle: Handle<Image> = asset_server.load(TILEMAP_ASSET_PATH);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle.clone(), vec2, 56, 42, None, Some(vec2));
    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    commands.insert_resource(Tileset {
        atlas_handle: texture_atlas_handle,
    });
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

pub fn spawn_tile_map(
    mut commands: Commands,
    board: Res<Board>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
    mut stage_query: Query<(Entity, &Stage)>,
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

    let centre_offset = tile_map.center_offset;
    let (stage_entity, _stage) = stage_query.single_mut();
    let transform = Transform::from_xyz(centre_offset.x, centre_offset.y, 0.);

    commands
        .get_entity(stage_entity)
        .expect("no stage, no player!")
        .with_children(|parent| {
            parent.spawn((
                tile_map,
                SpatialBundle {
                    transform,
                    ..default()
                },
            ));
        });

    match state.get() {
        AppState::InitTileMap => next_state.set(AppState::InitMobs),
        s => panic!("illegal state: {:?}", s),
    }
}

pub fn update_tiles_for_player_cell_visibility(
    mut commands: Commands,
    tileset: Res<Tileset>,
    mut tile_map_query: Query<(Entity, &mut TileMap)>,
    mut sprite_query: Query<&mut TextureAtlasSprite>,
    cell_query: Query<(&Cell, &mut PlayerCellVisibility), Changed<PlayerCellVisibility>>,
) {
    let mut counter = 0;

    match tile_map_query.get_single_mut() {
        Ok((tm_e, mut tile_map)) => {
            commands.entity(tm_e).with_children(|tiles| {
                cell_query.for_each(|(cell, player_visibility)| {
                    // LOOP over cell visibility changes
                    if player_visibility.visible {
                        match tile_map.entities.get(&cell.position) {
                            Some(e) => {
                                if let Ok(mut sprite) = sprite_query.get_mut(*e) {
                                    // TODO: fade out based on distance from player
                                    //
                                    sprite.color.set_a(100.0);
                                }
                            }
                            None => {
                                counter += 1;
                                let PixelPos { x, y } =
                                    tile_map.tile_offset(cell.position.x, cell.position.y);
                                let texture_index = texture_index_for_cell(cell);
                                let sprite = TextureAtlasSprite::new(texture_index);
                                let transform = Transform::from_xyz(x, y, TILE_MAP_Z as f32);

                                let tile_entity = tiles
                                    .spawn(SpriteSheetBundle {
                                        texture_atlas: tileset.atlas_handle.to_owned(),
                                        sprite,
                                        transform,
                                        ..default()
                                    })
                                    .id();
                                tile_map.entities.insert(cell.position, tile_entity);
                            }
                        }
                    } else if player_visibility.seen {
                        match tile_map.entities.get(&cell.position) {
                            Some(e) => {
                                if let Ok(mut sprite) = sprite_query.get_mut(*e) {
                                    sprite.color.set_a(0.10);
                                }
                            }
                            None => {
                                println!("{:?}", tile_map.entities);
                            }
                        }
                    }
                });
            });
        }
        Err(e) => println!("{:?}", e),
    }
    if counter > 0 {
        println!("spawned {:?} tiles", counter);
    }
}
