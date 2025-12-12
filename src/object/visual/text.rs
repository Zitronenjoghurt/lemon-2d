use macroquad::color::{Color, BLACK, WHITE};

#[derive(Debug)]
pub struct TextVisual {
    pub text: String,
    pub scale: f32,
    pub size: f32,
    pub color: Color,
    pub align: TextAlign,
}

impl Default for TextVisual {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            scale: 1.0,
            size: 14.0,
            color: Default::default(),
            align: Default::default(),
        }
    }
}

impl TextVisual {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn black(mut self) -> Self {
        self.color = BLACK;
        self
    }

    pub fn white(mut self) -> Self {
        self.color = WHITE;
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}
