pub trait Scene<G> {
    fn update(&mut self, state: &mut G, dt: f32) -> SceneUpdate<G>;
}

#[derive(Default)]
pub struct SceneUpdate<G> {
    pub new_scene: Option<Box<dyn Scene<G>>>,
}
