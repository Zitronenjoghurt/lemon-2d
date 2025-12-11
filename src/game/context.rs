use crate::scene::Scene;

pub struct GameContext<S> {
    pub scene: Box<dyn Scene<S>>,
    pub state: S,
}

impl<S> GameContext<S> {
    pub fn update(&mut self, dt: f32) {
        let update = self.scene.update(&mut self.state, dt);
        if let Some(new_scene) = update.new_scene {
            self.scene = new_scene;
        }
    }
}
