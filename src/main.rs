pub mod anatomy;
pub mod board;
pub mod character;
pub mod combat;
pub mod creature;
pub mod dice;
pub mod events;
pub mod graphics;
pub mod input;
pub mod player;
pub mod state;
pub mod time;
// pub mod ui;

pub mod typical {
    pub use crate::board::{
        cell::{
            Cell, CellFeature, CellFloorBundle, CellItems, CellWallBundle, Floor, Material, Wall,
        },
        direction::{Direction, CARDINAL_DIRECTIONS, COMPASS_DEGREES, DIRECTION_OFFSETS},
        primitives::{Area3d, Size3d},
        Board, PlayerCellVisibility, Position, BOARD_SIZE_X, BOARD_SIZE_Y, BOARD_SIZE_Z,
    };
    pub use crate::character::{Character, CharacterBundle, Equipment, Pace};
    pub use crate::creature::{species::Species, Attributes, Creature, CreatureSize, Locus};
    pub use crate::events::*;
    pub use crate::player::{Player, PlayerRes};
    pub use crate::state::{AppInitEvent, AppState};
    pub use crate::time::Clock;
    pub use bevy::math::{IVec2, IVec3, UVec2, UVec3};
    pub use bevy::prelude::{
        default, on_event, state_exists, state_exists_and_equals, App, BuildChildren, Bundle,
        Changed, Commands, Component, Deref, DerefMut, Entity, Event, EventReader, EventWriter,
        Has, IntoSystemConfigs, NextState, Plugin, Query, Res, ResMut, Resource, State, Vec2, Vec3,
        With, Without,
    };
    pub use bevy::prelude::{
        First, Last, OnEnter, OnExit, OnTransition, PostUpdate, PreUpdate, Startup, Update,
    };
    pub use bevy::utils::tracing::{debug, error, info, trace, warn, Level};

    pub use crate::{CameraMarker, MapMarker};
    pub use bevy::utils::{HashMap, HashSet};
}

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::{ClearColor, Color, DefaultPlugins, PluginGroup};
use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution, WindowTheme};
use bevy_fps_counter::FpsCounterPlugin;
use bevy_turborand::prelude::RngPlugin;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::utils::tracing::Level;
// use board::generator;
use graphics::player_avatar::{PlayerAvatar, PlayerAvatarRes};
use player::SpawnPlayerEvent;
use typical::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "~= D V E R G R =~".into(),
                        resolution: WindowResolution::new(2800.0, 1400.0),
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
                .set(LogPlugin {
                    level: Level::INFO,
                    filter: "wgpu=warn,bevy_ecs=info".to_string(),
                }),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Board>()
        .add_state::<AppState>()
        .add_event::<player::movement::DirectionalInput>()
        .add_event::<state::AppInitEvent>()
        .add_event::<player::SpawnPlayerEvent>()
        .add_event::<events::begin_action::UpdateLocus>()
        // plugins
        .add_plugins(FpsCounterPlugin)
        .add_plugins(RngPlugin::default())
        .add_plugins(time::TimePlugin)
        .add_systems(
            Startup,
            (
                board::generator::populate_board,
                player::spawn,
                apply_deferred,
                spawn_voxel_map,
                apply_deferred,
                graphics::player_avatar::spawn,
            )
                .chain(),
        )
        // .add_systems(OnEnter(AppState::SpawnPlayer), player::spawn)
        // .add_systems(
        //     OnEnter(AppState::BuildMap),
        //     board::generator::populate_board,
        // )
        // .add_systems(
        //     OnEnter(AppState::SpawnPlayerAvatar),
        //     graphics::player_avatar::spawn,
        // )
        // .add_systems(
        //     Update,
        //     graphics::components::spawn_stage.run_if(state_exists_and_equals(AppState::InitStage)),
        // )
        // .add_systems(OnEnter(AppState::InitUI), ui::spawn_layout)
        // .add_systems(
        //     PostUpdate,
        //     graphics::asset_loading::ensure_assets_loaded
        //         .run_if(state_exists_and_equals(AppState::LoadAssets)),
        // )
        // .add_systems(OnEnter(AppState::InitTileMap), spawn_voxel_map)
        // .add_systems(
        //     OnEnter(AppState::InitTileMap),
        //     graphics::tilemap::spawn_tile_map,
        // )
        // .add_systems(StartUp(AppState::InitPlayer), player::spawn_player)
        // .add_systems(
        //     OnEnter(AppState::InitMobs),
        //     graphics::player_avatar::spawn_player_avatar,
        // )
        //
        // MOVEMENT
        .add_systems(
            PreUpdate,
            input::keybindings.run_if(state_exists_and_equals(AppState::Ready)),
        )
        .add_systems(
            PreUpdate,
            (player::movement::validate_directional_input.after(input::keybindings))
                .run_if(state_exists_and_equals(AppState::Ready)),
        )
        .add_systems(
            PreUpdate,
            (creature::movement::process_movement
                .after(player::movement::validate_directional_input))
            .run_if(state_exists_and_equals(AppState::Ready)),
        )
        .add_systems(
            Update,
            graphics::move_anim::add_changed_creature_mob_move_anim
                .run_if(state_exists_and_equals(AppState::Ready)),
        )
        .add_systems(
            Update,
            (graphics::move_anim::player_movement
                .after(graphics::move_anim::add_changed_creature_mob_move_anim))
            .run_if(state_exists_and_equals(AppState::Ready)),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(PostUpdate, state::handle_app_init_event) // TODO REMOVE AFTER INIT COMPLETE
        .add_systems(PostUpdate, time::clock_frame_tick)
        // EVENTS
        // ok, ready?
        .run();
}

#[derive(Component, Debug)]
pub struct MapMarker;

// TODO organise map contents inside containers?

// #[derive(Component, Debug)]
// pub struct MapCubesMarker;

// #[derive(Component, Debug)]
// pub struct MapMobsMarker;

#[derive(Component, Debug)]
pub struct CameraMarker;

use crate::graphics::CreatureEntityRef;

// slightly larger than 1.0 so the overlap prevents bleed through
const VOXEL_CUBE_SIZE: f32 = 1.0;
const VOXEL_CUBE_MARGIN: f32 = 0.08;

fn spawn_voxel_map(
    board: Res<Board>,
    mut commands: Commands,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: ResMut<AssetServer>,
    mut ev: EventReader<SpawnPlayerEvent>,
    // player_query: Query<(Entity, &Player)>,
    player_ref: Res<PlayerRes>,
) {
    // ..
    let texture_handle: Handle<Image> = asset_server.load("dirt.png");

    let mask_material = materials.add(StandardMaterial {
        reflectance: 0.00,
        emissive: Color::NONE,
        alpha_mode: AlphaMode::Opaque,
        base_color: Color::BLACK,
        ..default()
    });

    let floor_material = materials.add(StandardMaterial {
        reflectance: 0.00,
        base_color_texture: Some(texture_handle.clone()),
        emissive: Color::NONE,
        alpha_mode: AlphaMode::Opaque,
        base_color: Color::WHITE,
        ..default()
    });

    let wall_material = materials.add(StandardMaterial {
        reflectance: 0.00,
        base_color_texture: Some(texture_handle),
        emissive: Color::NONE,
        alpha_mode: AlphaMode::Opaque,
        // base_color: Color::NONE,
        ..default()
    });

    let margin = f32::EPSILON * 2.0;

    let shape = meshes.add(
        shape::Cube {
            size: VOXEL_CUBE_SIZE,
        }
        .into(),
    );

    let mask_shape = meshes.add(
        shape::Cube {
            size: VOXEL_CUBE_SIZE + 2.0 * VOXEL_CUBE_MARGIN,
        }
        .into(),
    );

    let bx = 0.0 - board.size.width as f32;
    let by = 0.0 - board.size.height as f32;

    let player_entity = player_ref.entity;

    commands
        .spawn((
            MapMarker,
            TransformBundle {
                local: Transform::from_xyz(bx, by, 0.),
                ..default()
            },
            Visibility::Inherited,
            InheritedVisibility::default(),
        ))
        .with_children(|ch| {
            for (ivec, _e) in board.cell_store.iter() {
                let [x, y, z] = ivec.to_array();

                // floor
                ch.spawn((PbrBundle {
                    mesh: shape.clone(),
                    material: floor_material.clone(),
                    transform: Transform::from_xyz(
                        x as f32 - margin,
                        y as f32 - margin,
                        z as f32 - 1.0,
                    ),
                    ..default()
                },));
            }

            for (ivec, _e) in board.wall_store.iter() {
                let [x, y, z] = ivec.to_array();

                // wall
                ch.spawn((PbrBundle {
                    mesh: shape.clone(),
                    material: wall_material.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                    ..default()
                },));

                // haxx: mask
                // ch.spawn((PbrBundle {
                //     mesh: mask_shape.clone(),
                //     material: wall_material.clone(),
                //     transform: Transform::from_xyz(x as f32, y as f32, z as f32 + 1.0),
                //     ..default()
                // },));
            }

            for SpawnPlayerEvent(position) in ev.read() {
                let player_avatar_entity = ch
                    .spawn((
                        PlayerAvatar,
                        CreatureEntityRef(player_entity),
                        SpatialBundle {
                            transform: Transform::from_xyz(
                                position.x as f32,
                                position.y as f32,
                                0.,
                            ),
                            ..default()
                        },
                    ))
                    .with_children(|player| {
                        // lights ...
                        player.spawn(PointLightBundle {
                            point_light: PointLight {
                                intensity: 1500.0,
                                range: 15.,
                                shadows_enabled: true,
                                color: Color::GOLD,
                                ..default()
                            },
                            transform: Transform::from_xyz(0., 0., 0.25),
                            ..default()
                        });
                        // camera ...
                        player.spawn((
                            CameraMarker,
                            Camera3dBundle {
                                transform: Transform::from_xyz(0., 0., 40.0)
                                    .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
                                camera_3d: Camera3d {
                                    clear_color: ClearColorConfig::None,
                                    ..default()
                                },
                                ..default()
                            },
                        ));
                    })
                    .id();
                // create resource so we can access the entitity elsewhere
                cmd.insert_resource(PlayerAvatarRes {
                    entity: player_avatar_entity,
                });
            }
        });
}
