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
// use bevy::core_pipeline::core_3d::{Camera3dDepthLoadOp, Camera3dDepthTextureUsage};
use bevy::pbr::OpaqueRendererMethod;
use bevy::prelude::{ClearColor, Color, DefaultPlugins, PluginGroup};
use bevy::render::view::ColorGrading;
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
        .insert_resource(Msaa::default())
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
        .add_systems(
            Update,
                graphics::player_avatar::flicker_torches
            .run_if(state_exists_and_equals(AppState::Ready)),
        )
        .add_systems(Update, graphics::move_anim::move_head)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(PostUpdate, state::handle_app_init_event) // TODO REMOVE AFTER INIT COMPLETE
        .add_systems(PostUpdate, time::clock_frame_tick)
        // EVENTS
        // ok, ready?
        .run();
}

#[derive(Component, Debug)]
pub struct MapMarker;

#[derive(Component, Debug)]
pub struct TorchMarker;

#[derive(Component, Debug)]
pub struct TorchSecondaryLightMarker;

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
// const VOXEL_CUBE_MARGIN: f32 = 0.08;
                        const FOV:f32 = 120.;
const CAMERA3D_Z_POS:f32 = 20.;
fn spawn_voxel_map(
    board: Res<Board>,
    mut commands: Commands,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: ResMut<AssetServer>,
    mut ev: EventReader<SpawnPlayerEvent>,
    // mut ambient_light: Res<AmbientLight>,
    // player_query: Query<(Entity, &Player)>,
    player_ref: Res<PlayerRes>,
) {
    // ..
    let texture_handle: Handle<Image> = asset_server.load("dirt.png");
    // ambient_light.color = Color::BLACK;


    let floor_material = materials.add(StandardMaterial {
        reflectance: 0.01,
        perceptual_roughness: 1.0,
        diffuse_transmission: 0.0,
        base_color_texture: Some(texture_handle.clone()),

        // normal_map_texture: None,
        metallic: 0.0,
        // parallax_mapping_method: ParallaxMappingMethod::Relief { max_steps: 3 },
        ior: 1.0,

        thickness: 1.0,
        specular_transmission: 0.2,
        attenuation_distance: 0.01,
        attenuation_color: Color::BLACK,
        emissive: Color::BLACK,
        alpha_mode: AlphaMode::Opaque,
        opaque_render_method: OpaqueRendererMethod::Deferred,
        fog_enabled: false,
        double_sided: true,
        ..default()
    });

    // let margin = f32::EPSILON * 2.0;

    let shape = meshes.add(
        shape::Cube {
            size: VOXEL_CUBE_SIZE + f32::EPSILON,
        }
        .into(),
    );

    let bx = 0.0 - board.size.width as f32;
    let by = 0.0 - board.size.height as f32;

    let player_entity = player_ref.entity;

    commands.insert_resource(AmbientLight {
        color: Color::BLACK,
        brightness: 0.0,
    });

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
                        x as f32,
                        y as f32,
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
                    material: floor_material.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                    ..default()
                },));

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
                        // player.spawn((PointLightBundle {
                        //     point_light: PointLight {
                        //         intensity: 950.,
                        //         range: 120.,
                        //         shadows_enabled: true,
                        //         color: Color::rgba_linear(0.8, 0.3, 0.05, 1.0),
                        //         ..default()
                        //     },
                        //     transform: Transform::from_xyz(0., 0., 0.25),
                        //     ..default()
                        
                        player.spawn((SpotLightBundle {
                            spot_light: SpotLight {
                                intensity: 950.,
                                range: 120.,
                                shadows_enabled: true,
                                color: Color::rgba_linear(0.8, 0.3, 0.05, 1.0),
                                outer_angle: 2.5,
                                inner_angle: 0.2,
                                ..default()
                            },
                            // transform: Transform::from_xyz(0., 0., 0.25).looking_at(Vec3::new(0.0, 1.0, 0.), Vec3::splat(0.)),
                            transform: Transform::from_xyz(0., 0., 0.25).looking_at(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.)),
                            ..default()
                        }, TorchMarker)).with_children( |torch| { 
                            torch.spawn((TorchSecondaryLightMarker, SpatialBundle::default())
                            ); 
                        });

                        // camera ...
                    
                        
                        player.spawn((
                            CameraMarker,
                            Camera3dBundle {
                                projection: Projection::Perspective(PerspectiveProjection { fov: FOV, ..default() }),
                                transform: Transform::from_xyz(0., 0., CAMERA3D_Z_POS)
                                    .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
                                camera_3d: Camera3d {
                                    clear_color: ClearColorConfig::Custom(Color::BLACK),
                                    screen_space_specular_transmission_steps: 3,
                                    screen_space_specular_transmission_quality: bevy::core_pipeline::core_3d::ScreenSpaceTransmissionQuality::Ultra,
                                    ..default()
                                },
                                tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
                                color_grading: ColorGrading {
                                    exposure: 0.5,
                                    gamma: 1.4,
                                    pre_saturation: 0.8,
                                    post_saturation: 0.6,
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
