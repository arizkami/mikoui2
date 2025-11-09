use skia_safe::{Canvas, Paint, Rect};

use crate::components::Widget;
use crate::theme::{with_alpha, Theme};

pub struct Card {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    hover: bool,
    hover_progress: f32,
}

impl Card {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            hover: false,
            hover_progress: 0.0,
        }
    }
}

impl Widget for Card {
    fn draw(&self, canvas: &Canvas, _font_manager: &mut crate::core::FontManager) {
        let border_radius = Theme::RADIUS_LG;

        // Background
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(Theme::CARD);

        canvas.draw_round_rect(
            Rect::from_xywh(self.x, self.y, self.width, self.height),
            border_radius,
            border_radius,
            &paint,
        );

        // Border
        let mut border_paint = Paint::default();
        border_paint.set_anti_alias(true);
        border_paint.set_style(skia_safe::PaintStyle::Stroke);
        border_paint.set_color(Theme::BORDER);
        border_paint.set_stroke_width(1.0);

        canvas.draw_round_rect(
            Rect::from_xywh(
                self.x + 0.5,
                self.y + 0.5,
                self.width - 1.0,
                self.height - 1.0,
            ),
            border_radius,
            border_radius,
            &border_paint,
        );

        // Subtle shadow
        if self.hover_progress > 0.0 {
            let shadow_opacity = self.hover_progress * 0.1;
            let mut shadow_paint = Paint::default();
            shadow_paint.set_anti_alias(true);
            shadow_paint.set_color(with_alpha(Theme::BACKGROUND, (shadow_opacity * 255.0) as u8));
            
            canvas.draw_round_rect(
                Rect::from_xywh(self.x + 2.0, self.y + 2.0, self.width, self.height),
                border_radius,
                border_radius,
                &shadow_paint,
            );
        }
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    fn update_hover(&mut self, x: f32, y: f32) {
        self.hover = self.contains(x, y);
    }

    fn update_animation(&mut self, _elapsed: f32) {
        let animation_speed = 0.1;
        let target_hover = if self.hover { 1.0 } else { 0.0 };
        
        if (self.hover_progress - target_hover).abs() > 0.01 {
            self.hover_progress += (target_hover - self.hover_progress) * animation_speed;
        } else {
            self.hover_progress = target_hover;
        }
    }

    fn on_click(&mut self) {
        println!("Card clicked");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
