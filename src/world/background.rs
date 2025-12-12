use macroquad::color::Color;
use macroquad::window::clear_background;

#[derive(Default)]
pub struct Background {
    pub color: Option<Color>,
}

impl Background {
    pub fn draw(&self) {
        if let Some(color) = self.color {
            clear_background(color)
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }
}
