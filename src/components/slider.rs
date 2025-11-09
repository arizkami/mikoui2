use skia_safe::{Canvas, Paint, Rect};

use crate::components::Widget;
use crate::theme::{current_theme, with_alpha, Theme};

pub struct Slider {
    x: f32,
    y: f32,
    width: f32,
    label: &'static str,
    value: f32, // 0.0 to 1.0
    hover: bool,
    dragging: bool,
    hover_progress: f32,
}

impl Slider {
    pub fn new(x: f32, y: f32, width: f32, label: &'static str, initial_value: f32) -> Self {
        Self {
            x,
            y,
            width,
            label,
            value: initial_value.clamp(0.0, 1.0),
            hover: false,
            dragging: false,
            hover_progress: 0.0,
        }
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(0.0, 1.0);
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    fn track_rect(&self) -> Rect {
        let track_height = 4.0;
        let track_y = self.y + 20.0;
        Rect::from_xywh(self.x, track_y, self.width, track_height)
    }

    fn thumb_center(&self) -> (f32, f32) {
        let track = self.track_rect();
        let thumb_x = self.x + self.value * self.width;
        let thumb_y = track.center_y();
        (thumb_x, thumb_y)
    }
}

impl Widget for Slider {
    fn draw(&self, canvas: &Canvas, font_manager: &mut crate::core::FontManager) {
        // Draw label
        let font = font_manager.create_font(self.label, Theme::TEXT_SM, 500);
        let mut text_paint = Paint::default();
        let colors = current_theme();
        text_paint.set_anti_alias(true);
        text_paint.set_color(colors.foreground);
        canvas.draw_str(self.label, (self.x, self.y + 12.0), &font, &text_paint);

        // Draw track background
        let track = self.track_rect();
        let mut track_paint = Paint::default();
        track_paint.set_anti_alias(true);
        track_paint.set_color(colors.secondary);
        canvas.draw_round_rect(track, 2.0, 2.0, &track_paint);

        // Draw filled track
        let filled_width = self.value * self.width;
        if filled_width > 0.0 {
            let mut filled_paint = Paint::default();
            filled_paint.set_anti_alias(true);
            filled_paint.set_color(colors.primary);
            canvas.draw_round_rect(
                Rect::from_xywh(track.left(), track.top(), filled_width, track.height()),
                2.0,
                2.0,
                &filled_paint,
            );
        }

        // Draw thumb
        let (thumb_x, thumb_y) = self.thumb_center();
        let thumb_radius = if self.hover || self.dragging { 10.0 } else { 8.0 };

        // Thumb shadow
        let shadow_opacity = if self.hover || self.dragging { 0.2 } else { 0.1 };
        let mut shadow_paint = Paint::default();
        shadow_paint.set_anti_alias(true);
        shadow_paint.set_color(with_alpha(colors.background, (shadow_opacity * 255.0) as u8));
        canvas.draw_circle((thumb_x, thumb_y + 2.0), thumb_radius, &shadow_paint);

        // Thumb background
        let mut thumb_paint = Paint::default();
        thumb_paint.set_anti_alias(true);
        thumb_paint.set_color(colors.background);
        canvas.draw_circle((thumb_x, thumb_y), thumb_radius, &thumb_paint);

        // Thumb border
        let mut border_paint = Paint::default();
        border_paint.set_anti_alias(true);
        border_paint.set_style(skia_safe::PaintStyle::Stroke);
        border_paint.set_color(colors.border);
        border_paint.set_stroke_width(2.0);
        canvas.draw_circle((thumb_x, thumb_y), thumb_radius - 1.0, &border_paint);
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        let (thumb_x, thumb_y) = self.thumb_center();
        let dx = x - thumb_x;
        let dy = y - thumb_y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance <= 12.0 // Larger hit area
    }

    fn update_hover(&mut self, x: f32, y: f32) {
        self.hover = self.contains(x, y);
    }

    fn update_animation(&mut self, _elapsed: f32) {
        let animation_speed = 0.2;
        let target_hover = if self.hover || self.dragging { 1.0 } else { 0.0 };
        if (self.hover_progress - target_hover).abs() > 0.01 {
            self.hover_progress += (target_hover - self.hover_progress) * animation_speed;
        } else {
            self.hover_progress = target_hover;
        }
    }

    fn on_click(&mut self) {
        self.dragging = true;
        println!("Slider value: {:.2}", self.value);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
