// CRATES

use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};

use bevy_ecs_tilemap::prelude::*;
use bevy_pancam::PanCamPlugin;

// MODULES

pub mod action;
pub mod anatomy;
pub mod attributes;
pub mod config;
pub mod damage;
pub mod dice;
pub mod map;
pub mod ui;
pub mod sys {
    pub mod player_movement;
}
pub mod time;

use attributes::*;
use config::*;
#[allow(unused_imports)]
use map::*;
use sys::player_movement::*;
use time::TimePlugin;

// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
// #[allow(dead_code)]
// enum AppState {
//     #[default]
//     SetupLogicalMap,
//     // LoadTextures,
//     // DrawUI,
//     // CreateCharacter,
//     // Prepare,
//     // BuildWorld,
//     // PopulateMap,
//     // Embark,
//     Playing,
//     // GameOver,
// }

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
            .set(ImagePlugin::default_nearest()),)) // no blurry sprites
        .add_plugins(PanCamPlugin::default())
        .add_plugins(TimePlugin::default())
        // .add_state::<AppState>()
        .add_plugins(TilemapPlugin)
        // .add_systems(OnEnter(AppState::LoadTextures), load_textures)
        // .add_systems(OnEnter(AppState::DrawUI), spawn_layout)
        // .add_systems(
        // Update,
        // check_textures.run_if(in_state(AppState::LoadTextures)),
        // )
        // .add_systems(OnEnter(AppState::CreateCharacter), create_character)
        // .add_systems(Startup, startup)
        .add_systems(Startup, ui::spawn_layout)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, keybindings)
        // .add_systems(Update, player_movement)
        // .add_systems(Update, commands_actions)
        .add_event::<PlayerMovementEvent>()
        .run();
}

// STARTUP

// pub fn build_logical_map(commands: Commands, res)

// COMPONENTS

#[derive(Component, Debug, Clone, Copy)]
struct Creature;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    attributes: Attributes,
}

#[allow(dead_code)]
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
fn commands_actions(mut commands: Commands, mut query: Query<(&mut Player, &mut TilePos)>) {}
