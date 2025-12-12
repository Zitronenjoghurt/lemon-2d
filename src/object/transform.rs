use crate::prelude::Vec2;

#[derive(Default)]
pub struct Transform {
    pub position: Vec2,
    pub z_index: u8,
}

impl Transform {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            z_index: 0,
        }
    }

    pub fn x(&self) -> f32 {
        self.position.x
    }

    pub fn y(&self) -> f32 {
        self.position.y
    }
}
