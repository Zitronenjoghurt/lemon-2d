use crate::scene::Scene;
use config::GameConfig;
use context::GameContext;

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
            world: Default::default(),
            debug: Default::default(),
        };

        macroquad::Window::from_config(
            macroquad::conf::Conf {
                miniquad_conf: macroquad::miniquad::conf::Conf {
                    window_title: config.title,
                    window_width: config.width as i32,
                    window_height: config.height as i32,
                    platform: macroquad::miniquad::conf::Platform {
                        apple_gfx_api: config.apple_graphics_backend.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                default_filter_mode: config.filter_mode.into(),
                ..Default::default()
            },
            async move {
                ctx.on_start();
                loop {
                    let delta_time = macroquad::time::get_frame_time();
                    let start = macroquad::time::get_time();
                    ctx.update(delta_time);
                    let update_time = macroquad::time::get_time() - start;
                    if config.debug_mode {
                        ctx.debug.frame_time(delta_time);
                        ctx.debug.update_time(update_time);
                    }
                    macroquad::prelude::next_frame().await;
                }
            },
        );
    }
}
