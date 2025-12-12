use lemon_2d::prelude::*;
use tracing_subscriber::EnvFilter;

mod objects;
mod scenes;

#[derive(Default)]
pub struct MyGame;

impl Game for MyGame {
    fn init() -> Self {
        Self
    }

    fn config(&self) -> GameConfig {
        GameConfig {
            title: "My Game".to_string(),
            width: 1920,
            height: 1080,
            filter_mode: FilterMode::Nearest,
            debug_mode: true,
            ..Default::default()
        }
    }

    fn default_scene() -> Box<dyn Scene<Self>> {
        Box::new(scenes::main_menu::MainMenuScene)
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace")),
        )
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    MyGame::run();
}
