use crate::prelude::Visual;
use std::any::Any;

pub trait Behavior: Any {
    fn default_visual(&self) -> Visual;
    fn update(&mut self, _dt: f32) {}
}
