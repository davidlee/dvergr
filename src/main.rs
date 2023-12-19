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
use graphics::player_avatar::PlayerAvatar;
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
        .add_plugins(RngPlugin::default())
        .add_plugins(time::TimePlugin)
        .add_systems(
            Startup,
            (
                // graphics::mobs::load_spritesheet,
                board::generator::populate_board,
                player::spawn.after(board::generator::populate_board),
                spawn_voxel_map.after(player::spawn),
                // graphics::player_avatar::spawn.after(spawn_voxel_map),
            ),
        )
        .add_systems(
            OnEnter(AppState::SpawnPlayer),
            graphics::player_avatar::spawn,
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
                .run_if(state_exists_and_equals(AppState::Ready)),
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

#[derive(Component, Debug)]
pub struct MapMarker;

#[derive(Component, Debug)]
pub struct CameraMarker;

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
    // let map = commands.spawn_empty().id();
    let texture_handle: Handle<Image> = asset_server.load("dirt.png");

    let my_material = materials.add(StandardMaterial {
        reflectance: 0.1,
        // attenuation_distance: 0.1,
        // attenuation_color: Color::BLACK,
        // thickness: 1.0,
        base_color_texture: Some(texture_handle),
        emissive: Color::NONE,
        alpha_mode: AlphaMode::Opaque,
        base_color: Color::WHITE,

        ..default()
    });

    // slightly larger than 1.0 so the overlap prevents bleed through
    let shape = meshes.add(shape::Cube { size: 1.04 }.into());

    let bx = 0.0 - board.size.width as f32;
    let by = 0.0 - board.size.height as f32;

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

                // haxx: floor
                ch.spawn((PbrBundle {
                    mesh: shape.clone(),
                    material: my_material.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32 - 1.0),
                    ..default()
                },));
            }

            for (ivec, _e) in board.wall_store.iter() {
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
        let x = bx + position.x as f32;
        let y = by + position.y as f32;

        commands
            .spawn((
                PointLightBundle {
                    point_light: PointLight {
                        intensity: 3500.0,
                        range: 45.,
                        shadows_enabled: true,
                        color: Color::GOLD,
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.5),
                    ..default()
                },
                PlayerAvatar,
            ))
            .with_children(|player| {
                player.spawn((
                    CameraMarker,
                    Camera3dBundle {
                        transform: Transform::from_xyz(0., 0., 40.0)
                            .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
                        camera_3d: Camera3d {
                            clear_color: ClearColorConfig::None,
                            // order: 0,
                            ..default()
                        },
                        ..default()
                    },
                ));
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
