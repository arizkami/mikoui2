use skia_safe::{Canvas, Color, Font, Paint};

use crate::components::Widget;

pub struct Label {
    x: f32,
    y: f32,
    text: &'static str,
    font_size: f32,
    weight: i32,
    color: Color,
}

impl Label {
    pub fn new(
        x: f32,
        y: f32,
        text: &'static str,
        font_size: f32,
        weight: i32,
        color: Color,
    ) -> Self {
        Self {
            x,
            y,
            text,
            font_size,
            weight,
            color,
        }
    }
}

impl Widget for Label {
    fn draw(&self, canvas: &Canvas, font_factory: &dyn Fn(f32, i32) -> Font) {
        let font = font_factory(self.font_size, self.weight);

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(self.color);

        canvas.draw_str(self.text, (self.x, self.y + self.font_size), &font, &paint);
    }

    fn contains(&self, _x: f32, _y: f32) -> bool {
        false // Labels are not interactive
    }

    fn update_hover(&mut self, _x: f32, _y: f32) {}

    fn update_animation(&mut self, _elapsed: f32) {}

    fn on_click(&mut self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
