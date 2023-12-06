use bevy::prelude::*;
use bevy::utils::HashMap;

use super::TILEMAP_ASSET_PATH;
use crate::graphics::typical::*;
use crate::typical::*;

#[allow(dead_code)]
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
    pub center_offset: Vec2,
    pub entities: HashMap<IVec3, Entity>,
}

impl TileMap {
    pub fn tile_offset(&self, x: i32, y: i32) -> Vec2 {
        let x = self.tile_size.width * x as f32;
        let y = self.tile_size.height * y as f32;
        Vec2 { x, y }
    }

    pub fn new(tile_size: TileSize, grid_size: GridSize) -> Self {
        let width = tile_size.width * grid_size.width as f32;
        let height = tile_size.height * grid_size.height as f32;

        TileMap {
            tile_size,
            grid_size,
            dimensions: PixelSize { width, height },
            center_offset: Vec2 {
                x: 0. - width / 2.,
                y: 0. - height / 2.,
            },
            entities: HashMap::new(),
        }
    }

    pub fn translate(&self, board_pos: &IVec3) -> Vec2 {
        let x = (self.tile_size.width * board_pos.x as f32) + self.center_offset.x;
        let y = (self.tile_size.height * board_pos.y as f32) + self.center_offset.y;
        Vec2::new(x, y)
    }

    pub fn from_pixels(&self, cursor_pos: &Vec2) -> IVec3 {
        let x = (cursor_pos.x - self.center_offset.x) / self.tile_size.width;
        let y = (cursor_pos.y - self.center_offset.y) / self.tile_size.height;
        IVec3::new(x as i32, y as i32, 0)
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

// systems

pub fn spawn_tile_map(
    mut commands: Commands,
    board: Res<Board>,
    mut stage_query: Query<(Entity, &Stage)>,
    mut ev_writer: EventWriter<AppInitEvent>,
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

    ev_writer.send(AppInitEvent::SetAppState(AppState::InitMobs))
}

pub fn update_tiles_for_player_cell_visibility(
    mut commands: Commands,
    tileset: Res<Tileset>,
    mut tile_map_query: Query<(Entity, &mut TileMap)>,
    mut sprite_query: Query<(&mut TextureAtlasSprite, Option<&mut Lerpf32>)>,
    cell_query: Query<
        (&Cell, &mut PlayerCellVisibility, Option<&Wall>),
        Changed<PlayerCellVisibility>,
    >,
) {
    let (tile_map_entity, mut tile_map) = match tile_map_query.get_single_mut() {
        Err(_) => return,
        Ok((tile_map_entity, tile_map)) => (tile_map_entity, tile_map),
    };

    // collect a list of entities to animate fading out; we can't use (reborrow) commands
    // inside .with_children
    let mut fade_entites: Vec<Entity> = vec![];

    commands.entity(tile_map_entity).with_children(|tiles| {
        cell_query.for_each(|(cell, player_visibility, maybe_wall)| {
            if player_visibility.visible {
                match tile_map.entities.get(&cell.position) {
                    // newly visible
                    None => {
                        let Vec2 { x, y } = tile_map.tile_offset(cell.position.x, cell.position.y);
                        let texture_index = if maybe_wall.is_some() {
                            I_WALL
                        } else {
                            I_FLOOR
                        };
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
                        // add a reference to the tile map's entity store
                        tile_map.entities.insert(cell.position, tile_entity);
                    }
                    // seen previously
                    Some(e) => {
                        if let Ok((mut sprite, maybe_anim)) = sprite_query.get_mut(*e) {
                            sprite.color.set_a(100.0);
                            if let Some(mut anim) = maybe_anim {
                                anim.reset();
                            }
                        }
                    }
                }
            } else if player_visibility.seen {
                // newly obscured
                match tile_map.entities.get(&cell.position) {
                    Some(e) => {
                        if let Ok((_sprite, _maybe_anim)) = sprite_query.get_mut(*e) {
                            fade_entites.push(*e);
                        }
                    }
                    None => {
                        error!("unknown tile entities seen {:?}", tile_map.entities);
                    }
                }
            }
        });
    });

    // add component to fade alpha of component
    fade_entites.iter().for_each(|e| {
        commands.entity(*e).insert(Lerpf32::new(1.0, 0.1, 120));
    });
}

pub fn anim_fade_sprite_alpha(
    // mut commands: Commands,
    mut query: Query<(Entity, &mut Lerpf32, &mut TextureAtlasSprite)>,
) {
    for (_, mut anim, mut sprite) in query.iter_mut() {
        if anim.noop() {
            continue;
        } else if anim.done() {
            sprite.color = sprite.color.with_a(anim.target);
            anim.reset();
        } else {
            sprite.color = sprite.color.with_a(anim.current());
            anim.next();
        }
    }
}
