pub mod anatomy;
pub mod board;
pub mod character;
pub mod combat;
pub mod creature;
pub mod dice;
pub mod events;
pub mod graphics;
// pub mod input;
pub mod player;
pub mod state;
pub mod time;
pub mod ui;

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
    pub use crate::player::Player;
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

    pub use bevy::utils::{HashMap, HashSet};
}

use bevy::prelude::{ClearColor, Color, DefaultPlugins, ImagePlugin, PluginGroup};
use bevy::render::render_resource::SurfaceTexture;
use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution, WindowTheme};
use bevy_fps_counter::FpsCounterPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_turborand::prelude::RngPlugin;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::utils::tracing::Level;
use player::SpawnPlayerEvent;
use typical::*;

fn main() {
    App::new()
        .add_plugins(
            (DefaultPlugins
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
                    // level: Level::TRACE,
                    // level: Level::INFO,
                    level: Level::WARN,
                    filter: "wgpu=warn,bevy_ecs=info".to_string(),
                    ..default()
                })),
        )
        // .set(ImagePlugin::default_nearest()),)) // no blurry sprites
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Board>()
        .add_state::<AppState>()
        .add_event::<player::movement::DirectionalInput>()
        .add_event::<state::AppInitEvent>()
        .add_event::<player::SpawnPlayerEvent>()
        .add_event::<events::begin_action::UpdateLocus>()
        // plugins
        .add_plugins(FpsCounterPlugin)
        .add_plugins(PanCamPlugin)
        .add_plugins(RngPlugin::default())
        .add_plugins(time::TimePlugin)
        // .add_plugins(graphics::asset_loading::AssetLoadingPlugin)
        //
        // INITIALIZATION
        // .add_systems(Startup, spawn_camera)
        // .add_systems(
        //     OnEnter(AppState::InitAssets),
        //     (
        //         // graphics::tilemap::load_tileset,
        //         // graphics::mobs::load_spritesheet.after(graphics::tilemap::load_tileset),
        //     ),
        // )
        .add_systems(
            Startup,
            (
                board::generator::populate_board,
                player::spawn_player.after(board::generator::populate_board),
            ),
        )
        .add_systems(
            Startup,
            ((spawn_camera, spawn_voxel_map).after(player::spawn_player),),
        )
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
        // .add_systems(
        //     PreUpdate,
        //     // (input::keybindings, input::mousey_mousey)
        //         .run_if(state_exists_and_equals(AppState::Game)),
        // )
        // .add_systems(
        //     PreUpdate,
        //     (player::movement::validate_directional_input.after(input::keybindings))
        //         .run_if(state_exists_and_equals(AppState::Game)),
        // )
        // .add_systems(
        //     PreUpdate,
        //     (creature::movement::process_movement
        //         .after(player::movement::validate_directional_input))
        //     .run_if(state_exists_and_equals(AppState::Game)),
        // )
        // .add_systems(
        //     Update,
        //     graphics::move_anim::add_changed_creature_mob_move_anim
        //         .run_if(state_exists_and_equals(AppState::Game)),
        // )
        // .add_systems(
        //     Update,
        //     (graphics::move_anim::mob_movement
        //         .after(graphics::move_anim::add_changed_creature_mob_move_anim))
        //     .run_if(state_exists_and_equals(AppState::Game)),
        // )
        //
        // VISIBILITY
        .add_systems(
            Update,
            player::visibility::mark_player_visible_cells
                // .after(graphics::move_anim::mob_movement))
                .run_if(state_exists_and_equals(AppState::Game)),
        )
        // .add_systems(
        //     Update,
        //     (
        //         graphics::tilemap::update_tiles_for_player_cell_visibility,
        //         graphics::tilemap::anim_fade_sprite_alpha,
        //     )
        //         .after(player::visibility::mark_player_visible_cells)
        //         .run_if(state_exists_and_equals(AppState::Game)),
        // )
        // MISC
        //
        //
        // .add_systems(
        //     Update,
        //     graphics::render_gizmos.run_if(state_exists_and_equals(AppState::Game)),
        // )
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(PostUpdate, state::handle_app_init_event) // TODO REMOVE AFTER INIT COMPLETE
        .add_systems(PostUpdate, time::clock_frame_tick)
        // EVENTS
        // ok, ready?
        .run();
}

fn spawn_camera(mut commands: Commands, board: Res<Board>) {
    let x = 0. - board.size.width as f32 / 2.0;
    let y = 0. - board.size.height as f32 / 2.0;
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(x, y, 40.0).looking_at(Vec3::new(x, y, 0.), Vec3::Y),
        ..default()
    });
}

#[derive(Component, Debug)]
struct Map;

#[derive(Component, Debug)]
struct CameraMarker;

const IMAGE_PATH: &str = "sq.png";

fn spawn_voxel_map(
    mut commands: Commands,
    board: Res<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: ResMut<AssetServer>,
    // query: Query<&Locus>,
    mut ev: EventReader<SpawnPlayerEvent>,
) {
    // ..
    let map = commands.spawn_empty().id();
    let texture_handle: Handle<Image> = asset_server.load("dirt.png");
    let my_material = materials.add(StandardMaterial {
        occlusion_texture: Some(texture_handle.clone()),
        base_color_texture: Some(texture_handle),
        attenuation_color: Color::WHITE,
        attenuation_distance: 10.0,
        // base_color_texture: Some(texture_handle),
        emissive: Color::NONE,
        perceptual_roughness: 0.9,
        // metallic: 0.0,
        reflectance: 0.1,
        opaque_render_method: bevy::pbr::OpaqueRendererMethod::Auto,
        // fog_enabled: true,
        // diffuse_transmission: 0.1,
        // attenuation_color: Color::BLACK,
        // specular_transmission: 0.1,
        // unlit: true,
        alpha_mode: AlphaMode::Opaque,
        base_color: Color::WHITE,
        ..default()
    });

    let shape = meshes.add(shape::Cube::default().into());

    let bx = 0.0 - board.size.width as f32;
    let by = 0.0 - board.size.height as f32;

    warn!("voxelating");
    commands
        .spawn((
            Map,
            TransformBundle {
                local: Transform::from_xyz(bx, by, 0.),
                ..default()
            },
            Visibility::Inherited,
            InheritedVisibility::default(),
        ))
        .with_children(|ch| {
            for (ivec, e) in board.cell_store.iter() {
                let [x, y, z] = ivec.to_array();

                // haxx: floor
                ch.spawn((PbrBundle {
                    mesh: shape.clone(),
                    material: my_material.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32 - 1.0),
                    ..default()
                },));
            }

            for (ivec, e) in board.wall_store.iter() {
                let [x, y, z] = ivec.to_array();

                // haxx: floor
                ch.spawn((PbrBundle {
                    mesh: shape.clone(),
                    material: my_material.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                    ..default()
                },));
            }
        });

    for SpawnPlayerEvent(position) in ev.read() {
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 5000.0,
                range: 75.,
                shadows_enabled: true,
                color: Color::GOLD,
                ..default()
            },
            transform: Transform::from_xyz(bx + position.x as f32, by + position.y as f32, 0.3),
            ..default()
        });
    }

    // if let Ok(x) = query.get_single() {
    //     let position = match x.position {
    //         Position::Point(pos) => pos,
    //         _ => IVec3::ZERO,
    //     };

    //     warn!("###### {:?}", position);
    //     // makes some lights

    //     commands.spawn(PointLightBundle {
    //         point_light: PointLight {
    //             intensity: 5000.0,
    //             range: 100.,
    //             shadows_enabled: true,
    //             color: Color::GOLD,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(bx + position.x as f32, by + position.y as f32, 15.),
    //         ..default()
    //     });
    // } else {

    // }
}
