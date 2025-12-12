use crate::object::behavior::Behavior;
use crate::object::{Object, ObjectId};
use crate::prelude::Transform;
use crate::world::camera::Camera;
use ahash::HashMap;

pub mod camera;

#[derive(Default)]
pub struct World {
    pub camera: Camera,
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

    pub fn draw_all(&self) {
        for object in self.objects.values() {
            object.draw();
        }
    }
}
