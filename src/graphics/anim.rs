use bevy::prelude::*;

pub trait SimpleTimer {
    fn next(&mut self) -> u32;
    fn done(&self) -> bool;
    fn noop(&self) -> bool;
    fn reset(&mut self) -> ();
}

#[derive(Component, Debug)]
pub struct Lerpf32 {
    pub initial: f32,
    pub target: f32,
    pub delta: f32,
    pub total_frames: u32,
    pub current_frame: u32,
}

#[derive(Component, Debug)]
pub struct LerpVec3 {
    pub initial: Vec3,
    pub target: Vec3,
    pub delta: Vec2,
    pub current_frame: u32,
    pub total_frames: u32,
}

impl Lerpf32 {
    pub fn new(from: f32, to: f32, steps: u32) -> Self {
        Self {
            initial: from,
            target: to,
            delta: (to - from) / steps as f32,
            total_frames: steps,
            current_frame: 0,
        }
    }

    pub fn current(&self) -> f32 {
        self.initial + self.delta * self.current_frame as f32
    }
}

impl SimpleTimer for Lerpf32 {
    fn done(&self) -> bool {
        self.current_frame >= self.total_frames
    }

    fn noop(&self) -> bool {
        self.total_frames == 0
    }

    fn reset(&mut self) -> () {
        self.initial = 0.;
        self.target = 0.;
        self.total_frames = 0;
        self.current_frame = 0;
    }

    fn next(&mut self) -> u32 {
        self.current_frame += 1;
        self.current_frame
    }
}

impl SimpleTimer for LerpVec3 {
    fn done(&self) -> bool {
        self.current_frame >= self.total_frames
    }

    fn noop(&self) -> bool {
        self.total_frames == 0
    }

    fn reset(&mut self) -> () {
        // self.initial = Vec3::ZERO;
        // self.target = Vec3::ZERO;
        self.total_frames = 0;
        self.current_frame = 0;
        self.delta = Vec2::ZERO;
    }

    fn next(&mut self) -> u32 {
        self.current_frame += 1;
        self.current_frame
    }
}

impl LerpVec3 {
    pub fn from_translation(initial: Vec3, target: Vec3, frames: u32) -> Self {
        let delta = Vec2 {
            x: (target.x - initial.x) / frames as f32,
            y: (target.y - initial.y) / frames as f32,
        };

        LerpVec3 {
            initial,
            target,
            delta,
            current_frame: frames,
            total_frames: frames,
        }
    }

    // pub fn current(&self) -> Vec3 {
    //     self.initial + self.delta * self.current_frame as f32
    // }
}
