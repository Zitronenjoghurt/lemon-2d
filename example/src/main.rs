use lemon_2d::prelude::*;

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
            width: 800,
            height: 600,
        }
    }

    fn default_scene() -> Box<dyn Scene<Self>> {
        Box::new(scenes::main_menu::MainMenuScene)
    }
}

fn main() {
    MyGame::run();
}
