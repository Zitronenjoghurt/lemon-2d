use crate::prelude::Game;
use crate::scene::context::SceneContext;

pub mod context;

pub trait Scene<G: Game> {
    fn update(&mut self, ctx: &mut SceneContext<G>) -> SceneUpdate<G>;
    fn on_enter(&mut self, _ctx: &mut SceneContext<G>) {}
    fn on_exit(&mut self, _ctx: &mut SceneContext<G>) {}
}

#[derive(Default)]
pub struct SceneUpdate<G: Game> {
    pub new_scene: Option<Box<dyn Scene<G>>>,
}
