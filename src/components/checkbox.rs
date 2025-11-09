use skia_safe::{Canvas, Paint, Rect};

use crate::components::Widget;
use crate::theme::{current_theme, lerp_color, with_alpha, Theme};

pub struct Checkbox {
    x: f32,
    y: f32,
    size: f32,
    label: &'static str,
    checked: bool,
    hover: bool,
    hover_progress: f32,
    check_progress: f32,
    active: bool,
    active_progress: f32,
    disabled: bool,
}

impl Checkbox {
    pub fn new(x: f32, y: f32, label: &'static str) -> Self {
        Self {
            x,
            y,
            size: 20.0,
            label,
            checked: false,
            hover: false,
            hover_progress: 0.0,
            check_progress: 0.0,
            active: false,
            active_progress: 0.0,
            disabled: false,
        }
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Widget for Checkbox {
    fn draw(&self, canvas: &Canvas, font_manager: &mut crate::core::FontManager) {
        let border_radius = Theme::RADIUS_SM;
        let colors = current_theme();

        // Animated scale
        let scale = 1.0 - (self.active_progress * 0.05);
        let center_x = self.x + self.size / 2.0;
        let center_y = self.y + self.size / 2.0;
        let scaled_size = self.size * scale;
        let scaled_x = center_x - scaled_size / 2.0;
        let scaled_y = center_y - scaled_size / 2.0;

        // Background color (shadcn style - checked state uses primary)
        let bg_color = if self.disabled {
            with_alpha(colors.muted, 128)
        } else if self.check_progress > 0.0 {
            lerp_color(colors.background, colors.primary, self.check_progress)
        } else {
            colors.background
        };

        // Background
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(bg_color);

        canvas.draw_round_rect(
            Rect::from_xywh(scaled_x, scaled_y, scaled_size, scaled_size),
            border_radius,
            border_radius,
            &paint,
        );

        // Border
        let border_color = if self.disabled {
            with_alpha(colors.border, 128)
        } else if self.check_progress > 0.0 {
            lerp_color(colors.border, colors.primary, self.check_progress)
        } else {
            colors.border
        };

        let mut border_paint = Paint::default();
        border_paint.set_anti_alias(true);
        border_paint.set_style(skia_safe::PaintStyle::Stroke);
        border_paint.set_color(border_color);
        border_paint.set_stroke_width(1.5);

        canvas.draw_round_rect(
            Rect::from_xywh(
                scaled_x + 0.75,
                scaled_y + 0.75,
                scaled_size - 1.5,
                scaled_size - 1.5,
            ),
            border_radius,
            border_radius,
            &border_paint,
        );

        // Checkmark (shadcn style - simple check)
        if self.check_progress > 0.0 {
            let check_color = if self.disabled {
                with_alpha(colors.primary_foreground, 128)
            } else {
                colors.primary_foreground
            };
            
            let mut check_paint = Paint::default();
            check_paint.set_anti_alias(true);
            check_paint.set_style(skia_safe::PaintStyle::Stroke);
            check_paint.set_color(with_alpha(check_color, (self.check_progress * 255.0) as u8));
            check_paint.set_stroke_width(2.0);
            check_paint.set_stroke_cap(skia_safe::PaintCap::Round);
            check_paint.set_stroke_join(skia_safe::PaintJoin::Round);

            // Animated checkmark
            let progress = self.check_progress;
            let padding = 4.0;
            let x_base = scaled_x + padding;
            let y_base = scaled_y + scaled_size / 2.0;

            // First line (down-right)
            let line1_end_x = x_base + (4.0 * progress.min(0.5) * 2.0);
            let line1_end_y = y_base + (4.0 * progress.min(0.5) * 2.0);
            canvas.draw_line((x_base, y_base), (line1_end_x, line1_end_y), &check_paint);

            // Second line (up-right)
            if progress > 0.5 {
                let line2_progress = (progress - 0.5) * 2.0;
                let line2_start_x = x_base + 4.0;
                let line2_start_y = y_base + 4.0;
                let line2_end_x = line2_start_x + (8.0 * line2_progress);
                let line2_end_y = line2_start_y - (8.0 * line2_progress);
                canvas.draw_line(
                    (line2_start_x, line2_start_y),
                    (line2_end_x, line2_end_y),
                    &check_paint,
                );
            }
        }

        // Label
        let font = font_manager.create_font(self.label, Theme::TEXT_SM, 400);
        let text_color = if self.disabled {
            with_alpha(colors.foreground, 128)
        } else {
            colors.foreground
        };
        
        let mut text_paint = Paint::default();
        text_paint.set_anti_alias(true);
        text_paint.set_color(text_color);

        canvas.draw_str(
            self.label,
            (self.x + self.size + Theme::SPACE_2, self.y + self.size / 2.0 + 5.0),
            &font,
            &text_paint,
        );
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + 200.0 && y >= self.y && y <= self.y + self.size
    }

    fn update_hover(&mut self, x: f32, y: f32) {
        self.hover = self.contains(x, y);
    }

    fn update_animation(&mut self, _elapsed: f32) {
        let animation_speed = 0.2;

        // Hover animation
        let target_hover = if self.hover { 1.0 } else { 0.0 };
        if (self.hover_progress - target_hover).abs() > 0.01 {
            self.hover_progress += (target_hover - self.hover_progress) * animation_speed;
        } else {
            self.hover_progress = target_hover;
        }

        // Check animation
        let target_check = if self.checked { 1.0 } else { 0.0 };
        if (self.check_progress - target_check).abs() > 0.01 {
            self.check_progress += (target_check - self.check_progress) * (animation_speed * 1.5);
        } else {
            self.check_progress = target_check;
        }

        // Active animation
        let target_active = if self.active { 1.0 } else { 0.0 };
        if (self.active_progress - target_active).abs() > 0.01 {
            self.active_progress += (target_active - self.active_progress) * (animation_speed * 3.0);
        } else {
            self.active_progress = target_active;
        }

        // Reset active state after animation
        if self.active && self.active_progress > 0.9 {
            self.active = false;
        }
    }

    fn on_click(&mut self) {
        if !self.disabled {
            self.checked = !self.checked;
            self.active = true;
            println!("Checkbox toggled: {}", self.checked);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
