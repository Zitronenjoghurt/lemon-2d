#[derive(Debug, Default, Clone, Copy)]
pub enum FilterMode {
    #[default]
    Linear,
    Nearest,
}

impl From<FilterMode> for macroquad::miniquad::FilterMode {
    fn from(filter_mode: FilterMode) -> Self {
        match filter_mode {
            FilterMode::Linear => macroquad::miniquad::FilterMode::Linear,
            FilterMode::Nearest => macroquad::miniquad::FilterMode::Nearest,
        }
    }
}
