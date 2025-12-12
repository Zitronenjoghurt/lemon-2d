use crate::prelude::Vec2;
use macroquad::camera::{set_camera, Camera2D};

pub struct Camera {
    zoom: f32,
    camera: Camera2D,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            camera: Default::default(),
        }
    }
}

impl Camera {
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn get_target(&self) -> Vec2 {
        self.camera.target
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
        self.camera.zoom = Vec2::new(
            zoom * 2.0 / macroquad::window::screen_width(),
            zoom * 2.0 / macroquad::window::screen_height(),
        );
        set_camera(&self.camera);
    }

    pub fn set_target(&mut self, target: Vec2) {
        self.camera.target = target;
        set_camera(&self.camera);
    }
}
