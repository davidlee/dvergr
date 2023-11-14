use bevy::{
    // app::AppError,
    prelude::*,
    window::{PresentMode, WindowTheme},
};

use bevy_ecs_tilemap::prelude::*;
use bevy_turborand::prelude::*;
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

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
struct PrimaryAttributes {
    dexterity: u8,
    agility: u8,
    resilience: u8,
    speed: u8,
    power: u8,
    will: u8,
    intuition: u8,
    magnetism: u8,
    perception: u8,
    acuity: u8,
}

const D10_VALUES: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
impl PrimaryAttributes {
    fn random() -> PrimaryAttributes {
        let rand = Rng::new();
        let d10 = || -> u8 { *rand.sample(&D10_VALUES).unwrap() };
        PrimaryAttributes {
            dexterity: d10(),
            agility: d10(),
            resilience: d10(),
            speed: d10(),
            power: d10(),
            will: d10(),
            intuition: d10(),
            magnetism: d10(),
            perception: d10(),
            acuity: d10(),
        }
    }
}

#[derive(Component, Debug)]
#[allow(dead_code)]
struct SecondaryAttributes {
    stamina: u8,
    reflexes: u8,
    composure: u8,
    stride: f32,   // square per tick at Relaxed pace
    recovery: f32, // stamina per tick at rest
}

impl SecondaryAttributes {
    fn new(attrs: &PrimaryAttributes) -> Self {
        SecondaryAttributes {
            stamina: (attrs.resilience + attrs.power) / 2,
            reflexes: (attrs.speed + attrs.acuity) / 2,
            composure: (attrs.will + attrs.magnetism) / 2,
            recovery: 1.0,
            stride: 1.0,
        }
        //
    }
}

#[allow(dead_code)]
#[derive(Component, Debug)]
struct Attributes {
    primary: PrimaryAttributes,
    secondary: SecondaryAttributes,
    stance: Stance,
    pace: Pace,
    facing: Direction,
    moving: Option<Direction>,
    inventory: (),
    conditions: (),
    needs: (),
    thoughts: (),
    wounds: (),
}

impl Attributes {
    fn new() -> Attributes {
        let primary = PrimaryAttributes::random();
        let secondary = SecondaryAttributes::new(&(primary.clone()));
        Attributes {
            primary,
            secondary,
            stance: Stance::Standing,
            pace: Pace::Relaxed,
            facing: Direction::Up,
            moving: None,
            inventory: (),
            conditions: (),
            needs: (),
            thoughts: (),
            wounds: (),
        }
    }
}

#[allow(dead_code)]
#[derive(Component, Debug)]
enum Pace {
    Inactive,    // 0.0
    Painstaking, // 0.25
    Deliberate,  // 0.5
    Relaxed,     // 1.0 * stride
    Brisk,       // 1.5
    Rapid,       // 3.0
    Reckless,    // 6.0
}

#[allow(dead_code)]
#[derive(Component, Debug)]
enum Stance {
    Grappling, // other
    Clinch,    // other
    OnGuard,
    Standing,
    Flatfooted,
    Unbalanced,
    Falling,
    Prone,
    Kneeling,
    Jumping,
    Climbing,
}

#[allow(dead_code)]
#[derive(Component, Debug, Copy, Clone)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn as_xy(self) -> (i32, i32) {
        match self {
            Self::Up => (0, 1),
            Self::UpRight => (1, 1),
            Self::Right => (1, 0),
            Self::DownRight => (1, -1),
            Self::Down => (0, -1),
            Self::DownLeft => (-1, -1),
            Self::Left => (-1, 0),
            Self::UpLeft => (-1, 1),
        }
    }
}

#[derive(Event, Debug)]
struct PlayerMovementEvent {
    direction: Direction,
}

// #[derive(Component, Debug)]
// enum CurrentAction<T> {
//     None,
//     Some(T),
// }

// enum MacroGameState {
//     Loading,
// }

// enum GameUIMode {
//     Game,
//     Inventory,
// }

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

fn keybindings(mut ev_player_move: EventWriter<PlayerMovementEvent>, keys: Res<Input<KeyCode>>) {
    let shifted: bool = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    if keys.just_pressed(KeyCode::Up) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                Direction::UpLeft
            } else {
                Direction::Up
            },
        })
    }

    if keys.just_pressed(KeyCode::Down) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                Direction::DownRight
            } else {
                Direction::Down
            },
        })
    }

    if keys.just_pressed(KeyCode::Left) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                Direction::DownLeft
            } else {
                Direction::Left
            },
        })
    }

    if keys.just_pressed(KeyCode::Right) {
        ev_player_move.send(PlayerMovementEvent {
            direction: if shifted {
                Direction::UpRight
            } else {
                Direction::Right
            },
        })
    }
}

#[allow(dead_code, unused_mut, unused_variables)]
fn commands_general(
    mut commands: Commands,
    mut query: Query<(&mut Player, &mut TilePos, &TilemapId)>,
) {
}

// TODO collision detection
fn move_to<'a, 'b>(
    &pos: &'a TilePos,
    dir: &'b Direction,
    map_size: &TilemapSize,
) -> Result<TilePos, &'b str> {
    let mut dest = TilePos { x: pos.x, y: pos.y };

    let result = (|| -> Result<TilePos, &str> {
        let (x, y) = dir.as_xy();
        dest.x = (dest.x as i32 + x) as u32;
        dest.y = (dest.y as i32 + y) as u32;
        Ok(dest)
    })()?;

    if result.within_map_bounds(map_size) {
        Ok(dest)
    } else {
        // TODO send invalid command notification
        // println!("Out of bounds! {:?}", dest)
        Err(&"out of bounds")
    }
}

fn player_movement(
    mut ev_player_move: EventReader<PlayerMovementEvent>,
    mut pos_query: Query<(&mut Player, &mut TilePos)>,
    map_size_query: Query<&TilemapSize>,
) {
    let (_player, mut pos) = pos_query.single_mut();
    // FIXME find the TilemapSize through a sensible reference
    let map_size: &TilemapSize = map_size_query.iter().find(|_x| -> bool { true }).unwrap();

    for e in ev_player_move.iter() {
        if let Ok(to) = move_to(&pos, &e.direction, map_size) {
            TilePos { x: pos.x, y: pos.y } = to;
        } else {
            // invalid command
        }
    }
}

#[allow(dead_code, unused_mut, unused_variables)]
fn commands_actions(mut commands: Commands, mut query: Query<(&mut Player, &mut TilePos)>) {}

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
        .add_systems(Update, keybindings)
        // .add_systems(Update, commands_stance)
        .add_systems(Update, player_movement)
        .add_systems(Update, commands_actions)
        .add_event::<PlayerMovementEvent>()
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

fn tick(_commands: Commands, mut query: Query<&mut Transform>) {
    query.for_each_mut(|x| println!("-> {:?}", x))
    // println!("tick");
}
