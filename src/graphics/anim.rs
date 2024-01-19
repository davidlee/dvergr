use bevy::prelude::*;

// LerpVec3

#[derive(Component, Debug)]
pub struct LerpVec3 {
    pub initial: Vec3,
    pub target: Vec3,
    pub delta: Vec3,
    pub frames_total: u32,
    pub frames_remaining: u32,
}

impl LerpVec3 {
    pub fn is_done(&self) -> bool {
        self.frames_remaining == 0
    }

    pub fn next(&mut self) -> u32 {
        self.frames_remaining -= 1;
        self.frames_remaining
    }

    pub fn from_translation(initial: Vec3, target: Vec3, frames: u32) -> Self {
        let delta = Vec3 {
            x: (target.x - initial.x) / frames as f32,
            y: (target.y - initial.y) / frames as f32,
            z: (target.z - initial.z) / frames as f32,
        };

        LerpVec3 {
            initial,
            target,
            delta,
            frames_remaining: frames,
            frames_total: frames,
        }
    }
}
