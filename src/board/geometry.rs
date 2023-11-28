use super::primitives::*;
use bevy::math::{IVec3, UVec2, UVec3};

// Rect
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Rect {
    pub origin: UVec2,
    pub size: Size2d,
}

// RectPrism
#[allow(dead_code)]
pub struct RectPrism {
    pub origin: IVec3,
    pub size: Size3d,
}

// https://www.redblobgames.com/grids/circle-drawing/
//
pub fn circle(centre: UVec3, radius: i32) -> Vec<UVec3> {
    let mut circle_squares = vec![];

    let [cx, cy, cz] = uvec3_to_ivec3(centre).to_array();

    let top = cy - radius;
    let bot = cy + radius;

    for y in top..bot {
        let dy: i32 = y - cy;
        let dx: f32 = f32::sqrt((radius * radius - dy * dy) as f32);
        let left: i32 = f32::ceil(cx as f32 - dx) as i32;
        let right: i32 = f32::floor(cx as f32 + dx) as i32;

        for x in left..right {
            circle_squares.push(UVec3::new(x as u32, y as u32, cz as u32))
        }
    }
    circle_squares
}

// TODO can this go somewhere better? can't monkey-patch IVec3::From<_>

pub fn uvec3_to_ivec3(uv: UVec3) -> IVec3 {
    IVec3::new(uv.x as i32, uv.y as i32, uv.z as i32)
}
