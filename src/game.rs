pub trait Game: Sized + 'static {
    fn config() -> GameConfig;
    fn init() -> Self;
    fn update(&mut self, delta_time: f32);

    fn run() {
        let config = Self::config();
        let mut game = Self::init();

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
                    game.update(delta_time);
                    macroquad::prelude::next_frame().await;
                }
            },
        );
    }
}

pub struct GameConfig {
    pub title: String,
    pub width: u16,
    pub height: u16,
}
