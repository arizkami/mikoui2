use skia_safe::{Canvas, Color, Font, Paint, Rect};

use crate::components::Widget;
use crate::theme::ZedTheme;

pub struct ProgressBar {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    progress: f32,          // 0.0 to 1.0
    animated_progress: f32, // Smoothly animated progress
    label: Option<&'static str>,
    pulse_offset: f32,
}

impl ProgressBar {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            progress: 0.0,
            animated_progress: 0.0,
            label: None,
            pulse_offset: 0.0,
        }
    }

    pub fn with_label(mut self, label: &'static str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }
}

impl Widget for ProgressBar {
    fn draw(&self, canvas: &Canvas, font_factory: &dyn Fn(f32, i32) -> Font) {
        let border_radius = self.height / 2.0;

        // Draw background with subtle gradient
        let mut bg_paint = Paint::default();
        bg_paint.set_anti_alias(true);
        bg_paint.set_color(ZedTheme::SURFACE);
        canvas.draw_round_rect(
            Rect::from_xywh(self.x, self.y, self.width, self.height),
            border_radius,
            border_radius,
            &bg_paint,
        );

        // Draw animated progress
        let filled_width = self.animated_progress * self.width;
        if filled_width > 0.0 {
            // Main progress bar
            let mut progress_paint = Paint::default();
            progress_paint.set_anti_alias(true);
            progress_paint.set_color(ZedTheme::PRIMARY);
            canvas.draw_round_rect(
                Rect::from_xywh(self.x, self.y, filled_width, self.height),
                border_radius,
                border_radius,
                &progress_paint,
            );

            // Animated shine/pulse effect
            let pulse_width = 40.0;
            let pulse_x = self.x + (filled_width * self.pulse_offset) - pulse_width / 2.0;
            
            if pulse_x > self.x && pulse_x < self.x + filled_width {
                let mut shine_paint = Paint::default();
                shine_paint.set_anti_alias(true);
                shine_paint.set_color(Color::from_argb(40, 255, 255, 255));
                
                let shine_rect = Rect::from_xywh(
                    pulse_x.max(self.x),
                    self.y,
                    pulse_width.min(self.x + filled_width - pulse_x),
                    self.height,
                );
                canvas.draw_round_rect(shine_rect, border_radius, border_radius, &shine_paint);
            }

            // Highlight on top edge
            let mut highlight_paint = Paint::default();
            highlight_paint.set_anti_alias(true);
            highlight_paint.set_color(Color::from_argb(30, 255, 255, 255));
            canvas.draw_round_rect(
                Rect::from_xywh(self.x, self.y, filled_width, self.height / 3.0),
                border_radius,
                border_radius,
                &highlight_paint,
            );
        }

        // Draw label if present
        if let Some(label) = self.label {
            let font = font_factory(11.0, 500);
            
            // Draw text with shadow for better visibility
            let (text_width, _) = font.measure_str(label, None);
            let text_x = self.x + (self.width - text_width) / 2.0;
            let text_y = self.y + self.height / 2.0 + 4.0;

            // Shadow
            let mut shadow_paint = Paint::default();
            shadow_paint.set_anti_alias(true);
            shadow_paint.set_color(Color::from_argb(100, 0, 0, 0));
            canvas.draw_str(label, (text_x + 0.5, text_y + 0.5), &font, &shadow_paint);

            // Text
            let mut text_paint = Paint::default();
            text_paint.set_anti_alias(true);
            text_paint.set_color(ZedTheme::TEXT);
            canvas.draw_str(label, (text_x, text_y), &font, &text_paint);
        }
    }

    fn contains(&self, _x: f32, _y: f32) -> bool {
        false // Progress bars are not interactive
    }

    fn update_hover(&mut self, _x: f32, _y: f32) {}

    fn update_animation(&mut self, elapsed: f32) {
        // Smooth progress animation
        let animation_speed = 0.1;
        if (self.animated_progress - self.progress).abs() > 0.001 {
            self.animated_progress += (self.progress - self.animated_progress) * animation_speed;
        } else {
            self.animated_progress = self.progress;
        }

        // Pulse/shine animation
        self.pulse_offset = (elapsed * 0.5).fract();
    }

    fn on_click(&mut self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
