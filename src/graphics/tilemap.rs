use std::ops::Add;

use bevy::utils::HashMap;
// use bevy::utils::hashbrown::ashMap;

use super::TILEMAP_ASSET_PATH;
use crate::graphics::typical::*;
// use crate::player::visibility::mark_player_visible_cells;
use crate::typical::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.07, 0.12, 0.18);

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

// DarkMap (fog of war)
//

#[derive(Component, Debug, Clone, Resource)]
pub struct DarkMap {
    pub tile_size: TileSize,
    pub grid_size: GridSize,
    pub dimensions: PixelSize,
    pub center_offset: PixelPos,
    store: HashMap<IVec3, Entity>,
}

// impl From<TileMap> for DarkMap {
//     fn from(tile_map: TileMap) -> Self {
//         DarkMap {
//             tile_size: tile_map.tile_size,
//             grid_size: tile_map.grid_size,
//             dimensions: tile_map.dimensions,
//             center_offset: tile_map.center_offset,
//             store: HashMap::new(),
//         }
//     }
// }

impl DarkMap {
    pub fn new_from(tile_map: &TileMap) -> Self {
        DarkMap {
            tile_size: tile_map.tile_size,
            grid_size: tile_map.grid_size,
            dimensions: tile_map.dimensions,
            center_offset: tile_map.center_offset,
            store: HashMap::new(),
        }
    }

    pub fn insert(&mut self, pos: IVec3, entity: Entity) {
        println!("DM {:?} #insert {:?} ", entity, pos);
        self.store.insert(pos, entity);
    }

    pub fn get(&mut self, pos: &IVec3) -> Option<&Entity> {
        self.store.get(pos)
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
    mut loading: ResMut<AssetsLoading>,
) {
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

// Once I get this working, throw it away and make it less like ass.
pub fn render_darkmap_changes(
    board: ResMut<Board>,
    dark_map_query: Query<&DarkMap>,
    cell_query: Query<(Entity, &Cell), Changed<Cell>>,
    mut sprite_query: Query<&Sprite>,
    mut commands: Commands,
) {
    // // get the cells that have changed
    // for (cell_entity, cell) in cell_query.iter() {
    //     // their positions
    //     if let Some(pos) = board.cell_store.get_pos(&cell_entity) {
    //         // then get a ref to the DarkMap store
    //         match dark_map_query.get_single() {
    //             Ok(dark_map) => {
    //                 println!("Cell {:?} , DarkMap {:?}", cell, dark_map);
    //                 // then find the corresponding entity for that position
    //                 match dark_map.store.get(pos) {
    //                     // now we got the sprite's entity, find the sprite ...
    //                     Some(dm_entity) => {
    //                         println!("{:?}", dm_entity);

    //                         // match sprite_query.get_mut(*dm_entity) {
    //                         //     Ok(mut sprite) => {
    //                         //         // let mut sprite = sprite.clone();
    //                         //         println!("DarkMap got {:?}", sprite);
    //                         //         // sprite.color = Color::RED;

    //                         //         // commands.entity(*sprite_entity).insert(sprite);
    //                         //         commands.entity(*dm_entity).remove::<Sprite>();
    //                         //     }
    //                         //     Err(e) => println!("{:?}", e),
    //                         // }
    //                     }
    //                     None => (), //println!("nah .. {:?}"),
    //                 }
    //             }
    //             Err(e) => {
    //                 println!("Failed to get DarkMap: poop error");
    //             }
    //         }
    //     }
    //     // println!("CHANGED CELL {:?}", cell);
    // }
}

pub fn spawn_tile_map(
    mut commands: Commands,
    // tileset: Res<Tileset>,
    board: Res<Board>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
    mut stage_query: Query<(Entity, &Stage)>,
    // cells_query: Query<(Entity, &mut Cell)>,
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
    let dark_map = DarkMap::new_from(&tile_map);

    let (stage_entity, _) = stage_query.single_mut();

    commands
        .get_entity(stage_entity)
        .unwrap()
        .with_children(|stage_entity| {
            // for tile_map / dark_map
            let transform =
                Transform::from_xyz(tile_map.center_offset.x, tile_map.center_offset.y, 0.);

            stage_entity
                // spawn the TileMap
                .spawn((
                    tile_map,
                    SpatialBundle {
                        transform,
                        ..default()
                    },
                ));
            // spawn the darkmap
            stage_entity.spawn((
                dark_map,
                SpatialBundle {
                    transform, // same as tile_map's
                    ..default()
                },
            ));
            // .with_children(|dark| {
            //     for iy in 0..BOARD_SIZE_Y {
            //         for ix in 0..BOARD_SIZE_X {
            //             let PixelPos { x, y } = tile_map.tile_offset(ix, iy);
            //             let transform = Transform::from_xyz(x, y, DARK_MAP_Z as f32);
            //             let entity = dark
            //                 .spawn(SpriteBundle {
            //                     sprite: Sprite {
            //                         color: BACKGROUND_COLOR.with_a(0.3),
            //                         custom_size: Some(Vec2::new(TILE_SIZE_W, TILE_SIZE_H)),
            //                         ..default()
            //                     },
            //                     transform,
            //                     ..default()
            //                 })
            //                 .id();
            //             dark_map.insert(IVec3::new(ix, iy, 0), entity);
            //         }
            //     }
            // });
        });

    match state.get() {
        AppState::InitTileMap => next_state.set(AppState::InitMobs),
        s => panic!("illegal state: {:?}", s),
    }
}

pub fn update_tiles_for_player_cell_visibility(
    mut commands: Commands,
    mut cmd: Commands,
    tileset: Res<Tileset>,
    mut tile_map_query: Query<(Entity, &mut TileMap)>,
    mut sprite_query: Query<&mut TextureAtlasSprite>,
    cell_query: Query<(&Cell, &mut PlayerCellVisibility), Changed<PlayerCellVisibility>>,
    // dark_map_query: Query<(&mut DarkMap)>,
) {
    let mut counter = 0;
    match tile_map_query.get_single_mut() {
        Ok((tm_e, mut tile_map)) => {
            commands.entity(tm_e).with_children(|tiles| {
                cell_query.for_each(|(cell, player_visibility)| {
                    if player_visibility.visible {
                        match tile_map.entities.get(&cell.position) {
                            Some(e) => {
                                if let Ok(mut sprite) = sprite_query.get_mut(*e) {
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
                                cmd.entity(*e).log_components();
                                if let Ok(mut sprite) = sprite_query.get_mut(*e) {
                                    sprite.color.set_a(0.10);
                                }
                            }
                            None => {
                                println!("{:?}", tile_map.entities);
                                // println!("Surprise.");
                            }
                        }
                    } else {
                    }
                    // darkmap
                });
            });
        }
        Err(e) => println!("{:?}", e),
    }
    if counter > 0 {
        println!("spawned {:?} tiles", counter);
    }
}
