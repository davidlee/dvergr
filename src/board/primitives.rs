use bevy::math::UVec3;

pub type Area3d = Vec<UVec3>;

// Size3d
#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Size3d {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
// pub type Pos2d = UVec2;

// Size2d
#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Size2d {
    pub width: u32,
    pub height: u32,
}
