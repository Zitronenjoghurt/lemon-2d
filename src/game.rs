pub use crate::scene::Scene;
pub use config::GameConfig;
pub use context::GameContext;

pub(crate) mod config;
pub(crate) mod context;

pub trait Game: Sized + 'static {
    fn init() -> Self;
    fn config(&self) -> GameConfig;
    fn default_scene() -> Box<dyn Scene<Self>>;

    fn run() {
        let state = Self::init();
        let config = state.config();
        let mut ctx = GameContext {
            scene: Self::default_scene(),
            state,
        };

        macroquad::Window::from_config(
            macroquad::conf::Conf {
                miniquad_conf: macroquad::miniquad::conf::Conf {
                    window_title: config.title,
                    window_width: config.width as i32,
                    window_height: config.height as i32,
                    ..Default::default()
                },
                ..Default::default()
            },
            async move {
                loop {
                    let delta_time = macroquad::time::get_frame_time();
                    ctx.update(delta_time);
                    macroquad::prelude::next_frame().await;
                }
            },
        );
    }
}
