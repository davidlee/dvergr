use bevy::math::IVec3;

pub type Area3d = Vec<IVec3>;

// Size3d
#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Size3d {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
}
// pub type Pos2d = UVec2;

// Size2d
#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Size2d {
    pub width: i32,
    pub height: i32,
}
