use crate::MyGame;
use lemon_2d::prelude::{Scene, SceneUpdate};

#[derive(Default)]
pub struct MainMenuScene;

impl Scene<MyGame> for MainMenuScene {
    fn update(&mut self, _state: &mut MyGame, _dt: f32) -> SceneUpdate<MyGame> {
        SceneUpdate::default()
    }
}
