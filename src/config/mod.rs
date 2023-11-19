// use bevy::prelude::*;
// use bevy::window::PrimaryWindow;
// use bevy::window::WindowResolution;
// // use bevy::winit::get_best_videomode;
// use bevy::winit::WinitWindows;

// const MIN: (f32, f32) = (1024.0, 768.0);

// pub fn default_res() -> WindowResolution {
//     WindowResolution::new(1024.0, 768.0)
// }

// // TODO

// #[allow(dead_code)]
// const FRACTIONAL_SIZE: (f32, f32) = (0.8, 0.8);
// #[allow(dead_code)]
// fn get_monitor_res(
//     winit_windows: NonSend<WinitWindows>,
//     window_query: Query<Entity, With<PrimaryWindow>>,
// ) -> Option<WindowResolution> {
//     if let Some(monitor) = window_query
//         .get_single()
//         .ok()
//         .and_then(|entity| winit_windows.get_window(entity))
//         .and_then(|winit_window| winit_window.current_monitor())
//     {
//         let size = <(f32, f32)>::from(monitor.size());
//         info!("{:?}", size);
//         if size > MIN {
//             return Some(WindowResolution::from(size));
//         }
//     }
//     None
// }
