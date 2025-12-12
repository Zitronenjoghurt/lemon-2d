use crate::prelude::{Transform, Visual};
use std::any::Any;

pub struct BehaviorContext<'a> {
    pub transform: &'a mut Transform,
    pub visual: &'a mut Visual,
    pub dt: f32,
}

pub trait Behavior: Any {
    fn default_visual(&self) -> Visual;
    fn update(&mut self, _ctx: &mut BehaviorContext<'_>) {}
}
