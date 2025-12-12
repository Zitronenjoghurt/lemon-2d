use crate::object::behavior::{Behavior, BehaviorContext};
use crate::object::{Object, ObjectId};
use crate::prelude::Transform;
use crate::world::background::Background;
use crate::world::camera::Camera;
use ahash::HashMap;

pub mod background;
pub mod camera;

#[derive(Default)]
pub struct World {
    pub camera: Camera,
    pub background: Background,
    objects: HashMap<ObjectId, Object>,
    next_id: ObjectId,
}

impl World {
    pub fn spawn_object(&mut self, behavior: impl Behavior, transform: Transform) -> ObjectId {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);

        let object = Object::new(id, behavior, transform);
        self.objects.insert(id, object);

        id
    }

    pub fn draw_objects(&self) {
        for object in self.objects.values() {
            object.draw();
        }
    }

    pub fn draw_all(&self) {
        self.background.draw();
        self.draw_objects();
    }

    pub fn update_all(&mut self, dt: f32) {
        for object in self.objects.values_mut() {
            let mut context = BehaviorContext {
                transform: &mut object.transform,
                visual: &mut object.visual,
                dt,
            };
            object.behavior.update(&mut context);
        }
    }
}
