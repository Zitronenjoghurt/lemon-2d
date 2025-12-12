use macroquad::miniquad::conf::AppleGfxApi;

#[derive(Debug, Default, Clone, Copy)]
pub enum AppleGraphicsBackend {
    #[default]
    Metal,
    OpenGL,
}

impl From<AppleGraphicsBackend> for AppleGfxApi {
    fn from(value: AppleGraphicsBackend) -> Self {
        match value {
            AppleGraphicsBackend::Metal => AppleGfxApi::Metal,
            AppleGraphicsBackend::OpenGL => AppleGfxApi::OpenGl,
        }
    }
}
