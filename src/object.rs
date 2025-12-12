use behavior::Behavior;
use transform::Transform;
use visual::Visual;

pub mod behavior;
pub mod transform;
pub mod visual;

pub type ObjectId = usize;

pub struct Object {
    pub id: ObjectId,
    pub transform: Transform,
    pub visual: Visual,
    pub behavior: Box<dyn Behavior>,
}

impl Object {
    pub fn new(id: ObjectId, behavior: impl Behavior, transform: Transform) -> Self {
        Self {
            id,
            transform,
            visual: behavior.default_visual(),
            behavior: Box::new(behavior),
        }
    }

    pub fn draw(&self) {
        self.visual.draw(&self.transform);
    }
}
