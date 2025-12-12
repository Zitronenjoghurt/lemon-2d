use crate::world::World;

pub struct SceneContext<'a, S> {
    pub state: &'a mut S,
    pub world: &'a mut World,
    pub dt: f32,
}
