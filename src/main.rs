//! Shows how to render simple primitive shapes with a single color.

// use bevy::window::WindowPlugin;
use bevy::{
    core::FrameCount,
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::Anchor,
    window::{PresentMode, WindowTheme},
};

// #[derive(Clone, Debug)]
// struct Position {
//     x: u16,
//     y: u16,
//     z: u16,
// }

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

    fn x_offset(&self, col: u16) -> f32 {
        let k: f32 = col as f32;
        self.width * k + self.spacing * k
    }

    fn y_offset(&self, row: u16) -> f32 {
        let k: f32 = row as f32;
        self.height * k + self.spacing * k
    }

    fn size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

#[derive(Debug)]
struct TileSet {
    x_count: u16,
    y_count: u16,
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
            x_count as f32 * TILE_CONFIG.width + (x_count + 2) as f32 * TILE_CONFIG.spacing;

        let height =
            y_count as f32 * TILE_CONFIG.width + (y_count + 2) as f32 * TILE_CONFIG.spacing;

        TileSet {
            x_count,
            y_count,
            x: (TILE_CONFIG.width - width) / 2.0,
            y: (TILE_CONFIG.height - height) / 2.0,
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
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "One day I will be a roguelike".into(),
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
            // LogDiagnosticsPlugin::default(),
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

    let cols: Vec<u16> = (0..tileset.x_count).collect();
    let rows: Vec<u16> = (0..tileset.y_count).collect();

    for col in &cols {
        for row in &rows {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.45, 0.25, 0.75),
                    custom_size: Some(TILE_CONFIG.size()),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    tileset.x + TILE_CONFIG.x_offset(*col),
                    tileset.y + TILE_CONFIG.y_offset(*row),
                    0.,
                )),
                ..default()
            });
        }
    }
}

// TODO add resizeability
// TODO add tick to blit tiles
// TODO convert to use bevy_ecs_tilemap
// TODO figure out how to iterate over sprites

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
