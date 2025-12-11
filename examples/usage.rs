use lemon_2d::game::{Game, GameConfig};

pub struct TestGame;

impl Game for TestGame {
    fn config() -> GameConfig {
        GameConfig {
            title: "Test Game".to_string(),
            width: 800,
            height: 600,
        }
    }

    fn init() -> Self {
        Self
    }

    fn update(&mut self, _dt: f32) {}
}

fn main() {
    TestGame::run();
}
