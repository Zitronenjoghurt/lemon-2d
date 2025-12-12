use crate::object::transform::Transform;
use crate::object::visual::text::TextVisual;
use macroquad::text::{draw_text_ex, TextParams};

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

        let (offset_x, offset_y) = match text.align {
            text::TextAlign::Center => (-dims.width / 2.0, dims.height / 2.0),
            text::TextAlign::Left => (0.0, dims.height / 2.0),
            text::TextAlign::Right => (-dims.width, dims.height / 2.0),
        };

        let cos = transform.rotation.radians().cos();
        let sin = transform.rotation.radians().sin();
        let rotated_offset_x = offset_x * cos - offset_y * sin;
        let rotated_offset_y = offset_x * sin + offset_y * cos;

        let x = transform.x() + rotated_offset_x;
        let y = transform.y() + rotated_offset_y;

        draw_text_ex(
            &text.text,
            x,
            y,
            TextParams {
                font: None,
                font_size: text.size as u16,
                font_scale: text.scale,
                font_scale_aspect: 1.0,
                rotation: transform.rotation.radians(),
                color: text.color,
            },
        );
    }
}
