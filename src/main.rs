// CRATES

use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution, WindowTheme};
// use bevy_pancam::PanCamPlugin;

// MODULES

pub mod action;
pub mod anatomy;
pub mod attributes;
pub mod board;
pub mod damage;
pub mod dice;
pub mod ui;
pub mod sys {
    pub mod player_movement;
}
pub mod player;
pub mod time;

// imports

use board::*;
use sys::player_movement::*;
use time::TimePlugin;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
#[allow(dead_code)]
enum AppState {
    #[default]
    LoadAssets,
    Game,
    // PrepareBoard,
    // SetupLogicalMap,
    // LoadTextures,
    // DrawUI,
    // CreateCharacter,
    // Prepare,
    // BuildWorld,
    // PopulateMap,
    // Embark,
    // GameOver,
}

// atlas setup
// per https://github.com/bevyengine/bevy/blob/main/examples/2d/texture_atlas.rs

#[derive(Resource, Default)]
struct RpgSpriteFolder(Handle<LoadedFolder>);

// fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // load multiple, individual sprites from a folder
//     commands.insert_resource(RpgSpriteFolder(asset_server.load_folder("assets/rd/16x16")));
// }

// fn check_textures(
//     mut next_state: ResMut<NextState<AppState>>,
//     rpg_sprite_folder: ResMut<RpgSpriteFolder>,
//     mut events: EventReader<AssetEvent<LoadedFolder>>,
// ) {
//     // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
//     for event in events.read() {
//         if event.is_loaded_with_dependencies(&rpg_sprite_folder.0) {
//             next_state.set(AppState::DrawUI);
//         }
//     }
// }

// MAIN

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "One day I will be a roguelike".into(),
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
            .set(ImagePlugin::default_nearest()),)) // no blurry sprites
        // .add_plugins(PanCamPlugin::default())
        .add_plugins(TimePlugin::default())
        // .add_plugins(MapPlugin {})
        .add_plugins(BoardPlugin)
        .add_state::<AppState>()
        // .add_state::<states::MainState>()
        // .add_plugins(TilemapPlugin)
        // .add_systems(OnEnter(AppState::LoadTextures), load_textures)
        // .add_systems(OnEnter(AppState::DrawUI), spawn_layout)
        // .add_systems(
        // Update,
        // check_textures.run_if(in_state(AppState::LoadTextures)),
        // )
        // .add_systems(OnEnter(AppState::CreateCharacter), create_character)
        // .add_systems(Startup, startup)
        // .add_systems(Startup, ui::spawn_layout)
        .add_systems(Startup, load_sprites)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, keybindings)
        // .add_systems(Update, player_movement)
        // .add_systems(Update, commands_actions)
        .add_event::<PlayerMovementEvent>()
        .run();
}

// STARTUP

fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("img/or16w_t.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(24.0, 24.0),
        56,
        2,
        None,
        Some(Vec2 { x: 24.0, y: 24.0 }),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());

    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(52),
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    },));
}

// COMPONENTS

// SYSTEMS

// #[allow(dead_code, unused_mut, unused_variables)]
// fn commands_actions(mut commands: Commands, mut query: Query<(&mut Player, &mut TilePos)>) {}
