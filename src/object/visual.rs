use crate::object::transform::Transform;
use macroquad::text::draw_text;

pub enum Visual {
    None,
    Text(String),
}

impl Visual {
    pub fn draw(&self, transform: &Transform) {
        match self {
            Self::None => {}
            Self::Text(text) => {
                draw_text(
                    text.as_str(),
                    transform.x,
                    transform.y,
                    20.0,
                    macroquad::color::WHITE,
                );
            }
        }
    }
}
