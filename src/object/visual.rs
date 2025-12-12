use crate::object::transform::Transform;
use crate::object::visual::text::TextVisual;
use macroquad::text::draw_text;

pub mod text;

#[derive(Debug, Default)]
pub enum Visual {
    #[default]
    None,
    Text(TextVisual),
}

impl Visual {
    pub fn draw(&self, transform: &Transform) {
        match self {
            Self::None => {}
            Self::Text(text) => {
                self.draw_text(text, transform);
            }
        }
    }

    fn draw_text(&self, text: &TextVisual, transform: &Transform) {
        let dims = macroquad::text::measure_text(&text.text, None, text.size as u16, text.scale);

        let (x, y) = match text.align {
            text::TextAlign::Center => (
                transform.x() - dims.width / 2.0,
                transform.y() + dims.height / 2.0,
            ),
            text::TextAlign::Left => (transform.x(), transform.y() + dims.height / 2.0),
            text::TextAlign::Right => (
                transform.x() - dims.width,
                transform.y() + dims.height / 2.0,
            ),
        };

        draw_text(&text.text, x, y, text.size, text.color);
    }
}
