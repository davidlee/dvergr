// CRATES

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};

use bevy_ecs_tilemap::prelude::*;

mod action;
mod anatomy;
mod attributes;
mod config;
mod damage;
mod dice;
mod map;
mod sys {
    pub mod player_movement;
}

// MODULES

use attributes::*;
use config::*;
use sys::player_movement::*;

use map::*;

// MAIN

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "One day I will be a roguelike".into(),
                    resolution: default_res(),
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
        .add_systems(Update, keybindings)
        // .add_systems(Update, commands_stance)
        .add_systems(Update, player_movement)
        .add_systems(Update, commands_actions)
        .add_event::<PlayerMovementEvent>()
        .run();
}

// STARTUP

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("16x16_gr.png");
    let window_res = default_res();
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let map_size = get_tilemap_size(&window_res, &tile_size);

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

    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

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

    let texture_handle: Handle<Image> = asset_server.load("16x16_thief.png");
    let mob_tile_storage: TileStorage = TileStorage::empty(map_size);

    // create a second, sparse tileamp at a higher z for mobs:
    //
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
        .insert(PlayerBundle::new());
}

// COMPONENTS

#[derive(Component, Debug)]
struct Creature;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    attributes: Attributes,
}

impl PlayerBundle {
    fn new() -> Self {
        PlayerBundle {
            player: Player,
            attributes: Attributes::new(),
        }
    }
}

// SYSTEMS

#[allow(dead_code, unused_mut, unused_variables)]
fn commands_general(
    mut commands: Commands,
    mut query: Query<(&mut Player, &mut TilePos, &TilemapId)>,
) {
}

#[allow(dead_code, unused_mut, unused_variables)]
fn commands_actions(mut commands: Commands, mut query: Query<(&mut Player, &mut TilePos)>) {}

fn tick(_commands: Commands, mut query: Query<&mut Transform>) {
    query.for_each_mut(|x| println!("-> {:?}", x))
    // println!("tick");
}
