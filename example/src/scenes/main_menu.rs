use crate::objects::text::TextObject;
use crate::MyGame;
use lemon_2d::prelude::{Scene, SceneContext, SceneUpdate, Transform};

#[derive(Default)]
pub struct MainMenuScene;

impl Scene<MyGame> for MainMenuScene {
    fn update(&mut self, ctx: &mut SceneContext<MyGame>) -> SceneUpdate<MyGame> {
        ctx.world.draw_all();
        SceneUpdate::default()
    }

    fn on_enter(&mut self, ctx: &mut SceneContext<MyGame>) {
        ctx.world.spawn_object(
            TextObject {
                text: "Hello World!".to_string(),
            },
            Transform::from_xy(20.0, 20.0),
        );
    }
}
