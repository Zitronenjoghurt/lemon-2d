use lemon_2d::prelude::{Behavior, Visual};

pub struct TextObject {
    pub text: String,
}

impl Behavior for TextObject {
    fn default_visual(&self) -> Visual {
        Visual::Text(self.text.clone())
    }
}
