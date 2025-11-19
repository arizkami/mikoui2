use skia_safe::{Canvas, Paint, Rect};

use crate::components::Widget;
use crate::theme::{current_theme, with_alpha, Theme};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressSize {
    Xs,     // 2px
    Sm,     // 4px
    Md,     // 8px
    Lg,     // 12px
    Xl,     // 16px
}

impl ProgressSize {
    pub fn height(&self) -> f32 {
        match self {
            ProgressSize::Xs => 2.0,
            ProgressSize::Sm => 4.0,
            ProgressSize::Md => 8.0,
            ProgressSize::Lg => 12.0,
            ProgressSize::Xl => 16.0,
        }
    }
    
    pub fn show_label(&self) -> bool {
        // Only show labels for larger sizes
        matches!(self, ProgressSize::Lg | ProgressSize::Xl)
    }
}

pub struct ProgressBar {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    progress: f32,          // 0.0 to 1.0
    animated_progress: f32, // Smoothly animated progress
    label: Option<&'static str>,
    pulse_offset: f32,
    size: ProgressSize,
}

impl ProgressBar {
    pub fn new(x: f32, y: f32, width: f32) -> Self {
        let size = ProgressSize::Md;
        Self {
            x,
            y,
            width,
            height: size.height(),
            progress: 0.0,
            animated_progress: 0.0,
            label: None,
            pulse_offset: 0.0,
            size,
        }
    }
    
    pub fn size(mut self, size: ProgressSize) -> Self {
        self.size = size;
        self.height = size.height();
        self
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
    fn draw(&self, canvas: &Canvas, font_manager: &mut crate::core::FontManager) {
        let border_radius = self.height / 2.0;
        let colors = current_theme();

        // Draw background
        let mut bg_paint = Paint::default();
        bg_paint.set_anti_alias(true);
        bg_paint.set_color(colors.secondary);
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
            progress_paint.set_color(colors.primary);
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
                shine_paint.set_color(with_alpha(colors.primary_foreground, 40));
                
                let shine_rect = Rect::from_xywh(
                    pulse_x.max(self.x),
                    self.y,
                    pulse_width.min(self.x + filled_width - pulse_x),
                    self.height,
                );
                canvas.draw_round_rect(shine_rect, border_radius, border_radius, &shine_paint);
            }
        }

        // Draw label if present and size allows
        if let Some(label) = self.label {
            if self.size.show_label() {
                let font = font_manager.create_font(label, Theme::TEXT_XS, 500);
                
                let (text_width, _) = font.measure_str(label, None);
                let text_x = self.x + (self.width - text_width) / 2.0;
                let text_y = self.y + self.height / 2.0 + 4.0;

                // Text
                let mut text_paint = Paint::default();
                text_paint.set_anti_alias(true);
                text_paint.set_color(colors.foreground);
                canvas.draw_str(label, (text_x, text_y), &font, &text_paint);
            }
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
