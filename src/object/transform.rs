#[derive(Default)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub z_index: u8,
}

impl Transform {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self { x, y, z_index: 0 }
    }
}
