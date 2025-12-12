pub mod apple_graphics_backend;
pub mod filter_mode;

#[derive(Default)]
pub struct GameConfig {
    pub title: String,
    pub width: u16,
    pub height: u16,
    pub filter_mode: filter_mode::FilterMode,
    pub debug_mode: bool,
    /// Default is Metal, use OpenGL if you target older devices.
    pub apple_graphics_backend: apple_graphics_backend::AppleGraphicsBackend,
}
