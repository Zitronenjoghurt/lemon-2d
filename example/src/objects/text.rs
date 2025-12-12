use lemon_2d::prelude::{Behavior, BehaviorContext, TextAlign, TextVisual, Visual};

pub struct TextObject {
    pub text: String,
}

impl Behavior for TextObject {
    fn default_visual(&self) -> Visual {
        Visual::Text(
            TextVisual::new(&self.text)
                .align(TextAlign::Center)
                .white()
                .size(20.0),
        )
    }

    fn update(&mut self, ctx: &mut BehaviorContext) {
        ctx.transform.rotate_degrees(ctx.dt * 50.0);
    }
}
