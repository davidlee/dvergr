use super::primitives::*;
use bevy::{
    math::{IVec2, IVec3},
    utils::HashSet,
};

// Rect
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Rect {
    pub origin: IVec2,
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
// pub fn circle(centre: IVec3, radius: i32) -> Vec<IVec3> {
//     let mut circle_squares = vec![];

//     let [cx, cy, cz] = centre.to_array();

//     let top = cy - radius;
//     let bot = cy + radius;

//     for y in top..bot {
//         let dy: i32 = y - cy;
//         let dx: f32 = f32::sqrt((radius * radius - dy * dy) as f32);
//         let left: i32 = f32::ceil(cx as f32 - dx) as i32;
//         let right: i32 = f32::floor(cx as f32 + dx) as i32;

//         for x in left..right {
//             circle_squares.push(IVec3::new(x, y, cz))
//         }
//     }
//     circle_squares
// }

pub fn circle_hash_set(centre: IVec3, radius: i32) -> HashSet<[i32; 3]> {
    let mut circle = HashSet::new();

    let [cx, cy, z] = centre.to_array();

    let top = cy - radius;
    let bot = cy + radius;

    for y in top..bot {
        let dy: i32 = y - cy;
        let dx: f32 = f32::sqrt((radius * radius - dy * dy) as f32);
        let left: i32 = f32::ceil(cx as f32 - dx) as i32;
        let right: i32 = f32::floor(cx as f32 + dx) as i32;

        for x in left..right {
            circle.insert([x, y, z]);
        }
    }
    circle
}
