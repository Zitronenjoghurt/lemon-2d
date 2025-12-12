use crate::prelude::Vec2;

#[derive(Default)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: Rotation,
    pub z_index: u8,
}

impl Transform {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            ..Default::default()
        }
    }

    pub fn rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn x(&self) -> f32 {
        self.position.x
    }

    pub fn y(&self) -> f32 {
        self.position.y
    }
}

#[derive(Debug, Default)]
#[repr(transparent)]
// Rotation in radians
pub struct Rotation(f32);

impl Rotation {
    pub fn from_radians(radians: f32) -> Self {
        Self(radians)
    }

    pub fn from_degrees(degrees: f32) -> Self {
        Self(degrees.to_radians())
    }

    pub fn radians(&self) -> f32 {
        self.0
    }

    pub fn degrees(&self) -> f32 {
        self.0.to_degrees()
    }
}
