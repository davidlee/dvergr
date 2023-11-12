use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};

use bevy_ecs_tilemap::prelude::*;
// mod helpers;

struct Resolution {
    x: f32,
    y: f32,
}

const DEFAULT_RES: Resolution = Resolution {
    x: 1024.0,
    y: 768.0,
};

#[derive(Component, Debug)]
struct Creature;

#[derive(Component, Debug)]
struct Player;

fn player_movement(mut query: Query<(&mut Player, &mut TilePos)>, keys: Res<Input<KeyCode>>) {
    for (_player, mut pos) in query.iter_mut() {
        // FIXME proper bounds checking
        // TODO action points
        if keys.just_pressed(KeyCode::Up) && pos.y < 32 {
            pos.y += 1;
        }

        if keys.just_pressed(KeyCode::Down) && pos.y > 0 {
            pos.y -= 1;
        }

        if keys.just_pressed(KeyCode::Left) && pos.x > 0 {
            pos.x -= 1;
        }

        if keys.just_pressed(KeyCode::Right) && pos.x < 32 {
            pos.x += 1;
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "One day I will be a roguelike".into(),
                    resolution: (DEFAULT_RES.x, DEFAULT_RES.y).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            })
            .set(
                ImagePlugin::default_nearest(),
                // LogDiagnosticsPlugin::default(),
                // FrameTimeDiagnosticsPlugin,
            ),))
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, (tick, bevy::window::close_on_esc))
        .add_systems(Update, player_movement)
        .run();
}

fn get_tilemap_size(resolution: &Resolution, tile_size: &TilemapTileSize) -> TilemapSize {
    let x: u32 = (resolution.x / tile_size.x) as u32;
    let y: u32 = (resolution.y / tile_size.y) as u32;
    TilemapSize { x, y }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("16x16_0.png");

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };

    let map_size = get_tilemap_size(&DEFAULT_RES, &tile_size);

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // let's try adding another tilemap for a second layer
    //
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    // let tilemap_transform =  get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0y),

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    let texture_handle: Handle<Image> = asset_server.load("16x16_char.png");
    let mob_tile_storage: TileStorage = TileStorage::empty(map_size);

    let tilemap_entity = commands.spawn_empty().id();
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: mob_tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0),
        ..Default::default()
    });

    let tile_pos: TilePos = TilePos { x: 1, y: 1 };

    // insert a single tile
    commands
        .spawn(TileBundle {
            position: tile_pos,
            tilemap_id: TilemapId(tilemap_entity),
            ..Default::default()
        })
        .insert((Player, Creature));
}

fn tick(_commands: Commands, mut query: Query<&mut Transform>) {
    query.for_each_mut(|x| println!("-> {:?}", x))
    // println!("tick");
}
