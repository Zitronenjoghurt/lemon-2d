pub mod filter_mode;

pub struct GameConfig {
    pub title: String,
    pub width: u16,
    pub height: u16,
    pub filter_mode: filter_mode::FilterMode,
}
