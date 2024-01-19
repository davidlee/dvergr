use bevy::prelude::*;

pub trait SimpleFrameTimer {
    fn next(&mut self) -> u32;
    fn is_done(&self) -> bool;
}

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

    // pub fn current(&self) -> Vec3 {
    //     self.initial + self.delta * self.current_frame as f32
    // }
}
impl SimpleFrameTimer for LerpVec3 {
    fn is_done(&self) -> bool {
        self.frames_remaining == 0
    }

    fn next(&mut self) -> u32 {
        self.frames_remaining -= 1;
        self.frames_remaining
    }
}
// Lerpf32

#[derive(Component, Debug)]
pub struct Lerpf32 {
    initial: f32,
    pub target: f32,
    pub delta: f32,
    pub total_frames: u32,
    pub frames_remaining: u32,
}

impl Lerpf32 {
    pub fn new(from: f32, to: f32, steps: u32) -> Self {
        Self {
            initial: from,
            target: to,
            delta: (to - from) / steps as f32,
            total_frames: steps,
            frames_remaining: 0,
        }
    }

    pub fn current(&self) -> f32 {
        self.initial + self.delta * self.frames_remaining as f32
    }
}

impl SimpleFrameTimer for Lerpf32 {
    fn is_done(&self) -> bool {
        self.frames_remaining == 0
    }

    fn next(&mut self) -> u32 {
        self.frames_remaining -= 1;
        self.frames_remaining
    }
}
