//! Shows how to render simple primitive shapes with a single color.

// use bevy::window::WindowPlugin;
use bevy::{
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{CursorGrabMode, PresentMode, WindowLevel, WindowTheme},
};

struct Position {
    x: u16,
    y: u16,
    z: u16,
}

struct Rect {
    width: f32,
    height: f32,
}

struct TileConfig {
    width: f32,
    height: f32,
    spacing: f32,
}

impl TileConfig {
    const fn square(n: f32, sp: f32) -> Self {
        Self {
            width: n,
            height: n,
            spacing: sp,
        }
    }

    fn w(&self) -> f32 {
        self.width + self.spacing
    }

    fn h(&self) -> f32 {
        self.height + self.spacing
    }
}

#[derive(Debug)]
struct TileSet {
    x_count: u16,
    y_count: u16,
    // width: f32,
    // height: f32,
    x: f32,
    y: f32,
}

// CONSTANTS
const TILE_CONFIG: TileConfig = TileConfig::square(16.0, 5.0);

impl TileSet {
    fn initial() -> TileSet {
        let x_count = (WINDOW_CONFIG.width / TILE_CONFIG.w() - 0.0) as u16;
        let y_count = (WINDOW_CONFIG.height / TILE_CONFIG.h() - 0.0) as u16;
        let width: f32 =
            x_count as f32 * TILE_CONFIG.width + (x_count - 1) as f32 * TILE_CONFIG.spacing;

        let height =
            y_count as f32 * TILE_CONFIG.width + (y_count - 1) as f32 * TILE_CONFIG.spacing;

        TileSet {
            x_count,
            y_count,
            // width,
            // height,
            x: 0.0 - (width / 2.0) + (TILE_CONFIG.width / 2.0),
            y: 0.0 - (height / 2.0) + (TILE_CONFIG.height / 2.0),
        }
    }
}

const WINDOW_CONFIG: Rect = Rect {
    width: 800.0,
    height: 600.0,
};

// #[derive(Debug)]
// struct TileCount {
//     horizontal: u16,
//     vertical: u16,
// }

// struct Player;

fn main() {
    // let stageWidth: f32 = TILE_CONFIG.w() * 10.0;
    // let stageHeight: f32 = TILE_CONFIG.h() * 10.0;

    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    resolution: (WINDOW_CONFIG.width, WINDOW_CONFIG.height).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (tick, make_visible, bevy::window::close_on_esc))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let tileset = TileSet::initial();
    println!("TILESET {:?}", tileset);

    let vxs: Vec<f32> = (0..tileset.x_count).map(|x| x as f32).collect();
    let vys: Vec<f32> = (0..tileset.y_count).map(|x| x as f32).collect();

    for xn in &vxs {
        for yn in &vys {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.45, 0.25, 0.75),
                    custom_size: Some(Vec2::new(TILE_CONFIG.width, TILE_CONFIG.height)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    tileset.x + (TILE_CONFIG.w() * xn),
                    tileset.y + (TILE_CONFIG.h() * yn),
                    0.,
                )),
                ..default()
            });
        }
    }
}

fn tick(_commands: Commands) {
    // println!("tick");
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}
