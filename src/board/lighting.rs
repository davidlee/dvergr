#![allow(dead_code)]

use crate::graphics::typical::*;
use crate::typical::*;

// light > light source
// inverse square
// repr. as
// [x,y] -> (direction3d / vector ; intensity u32)

#[derive(Component, Debug)]
pub struct LightSource {
    position: IVec3, // FIXME should be inherited
    color: Color,
    shape: (),
    intensity: f32,
}

struct LightingCache;
