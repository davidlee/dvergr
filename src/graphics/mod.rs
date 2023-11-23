use crate::board::{BoardRes, Cell, Pos3d};
use crate::AppState;
use bevy::prelude::*;

pub mod tilemap;
pub use tilemap::{TileMap, TileMapPlugin};

#[derive(Component, Debug, Copy, Clone)]
pub struct PixelSize {
    pub width: f32,
    pub height: f32,
}
type TileSize = PixelSize;

#[derive(Component, Debug, Copy, Clone)]
pub struct GridSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Component, Debug, Copy, Clone)]
pub struct PixelPos {
    pub x: f32,
    pub y: f32,
}
