use crate::game::Game;
use crate::scene::{context::SceneContext, Scene};
use crate::world::World;

pub struct GameContext<G: Game> {
    pub scene: Box<dyn Scene<G>>,
    pub state: G,
    pub world: World,
}

impl<G> GameContext<G>
where
    G: Game,
{
    pub fn update(&mut self, dt: f32) {
        self.update_scene(dt);
    }

    pub fn on_start(&mut self) {
        let mut scene_context = SceneContext {
            state: &mut self.state,
            world: &mut self.world,
            dt: 0.0,
        };
        self.scene.on_enter(&mut scene_context);
    }

    fn update_scene(&mut self, dt: f32) {
        let mut context = SceneContext {
            state: &mut self.state,
            world: &mut self.world,
            dt,
        };
        let update = self.scene.update(&mut context);
        if let Some(new_scene) = update.new_scene {
            self.scene.on_exit(&mut context);
            self.scene = new_scene;
            self.scene.on_enter(&mut context);
        }
    }
}
