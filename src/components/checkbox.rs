use skia_safe::{Canvas, Color, Font, Paint, Rect};

use crate::components::Widget;
use crate::theme::{lerp_color, ZedTheme};

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
}

impl Checkbox {
    pub fn new(x: f32, y: f32, label: &'static str) -> Self {
        Self {
            x,
            y,
            size: 18.0,
            label,
            checked: false,
            hover: false,
            hover_progress: 0.0,
            check_progress: 0.0,
            active: false,
            active_progress: 0.0,
        }
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }
}

impl Widget for Checkbox {
    fn draw(&self, canvas: &Canvas, font_factory: &dyn Fn(f32, i32) -> Font) {
        let border_radius = 4.0;

        // Animated scale
        let scale = 1.0 - (self.active_progress * 0.05);
        let center_x = self.x + self.size / 2.0;
        let center_y = self.y + self.size / 2.0;
        let scaled_size = self.size * scale;
        let scaled_x = center_x - scaled_size / 2.0;
        let scaled_y = center_y - scaled_size / 2.0;

        // Animated background color
        let bg_base = ZedTheme::INPUT_BG;
        let bg_hover = ZedTheme::INPUT_HOVER;
        let bg_checked = ZedTheme::PRIMARY;

        let mut current_bg = bg_base;
        if self.hover_progress > 0.0 && !self.checked {
            current_bg = lerp_color(current_bg, bg_hover, self.hover_progress);
        }
        if self.check_progress > 0.0 {
            current_bg = lerp_color(current_bg, bg_checked, self.check_progress);
        }

        // Draw shadow on hover
        let shadow_opacity = self.hover_progress * 0.15;
        if shadow_opacity > 0.0 {
            let mut shadow_paint = Paint::default();
            shadow_paint.set_anti_alias(true);
            shadow_paint.set_color(Color::from_argb((shadow_opacity * 255.0) as u8, 0, 0, 0));
            canvas.draw_round_rect(
                Rect::from_xywh(scaled_x + 1.0, scaled_y + 1.0, scaled_size, scaled_size),
                border_radius,
                border_radius,
                &shadow_paint,
            );
        }

        // Background
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(current_bg);

        canvas.draw_round_rect(
            Rect::from_xywh(scaled_x, scaled_y, scaled_size, scaled_size),
            border_radius,
            border_radius,
            &paint,
        );

        // Animated border
        let border_base = ZedTheme::BORDER;
        let border_hover = ZedTheme::BORDER_FOCUS;
        let border_checked = ZedTheme::PRIMARY;

        let mut current_border = border_base;
        if self.hover_progress > 0.0 && self.check_progress < 0.5 {
            current_border = lerp_color(current_border, border_hover, self.hover_progress);
        }
        if self.check_progress > 0.0 {
            current_border = lerp_color(current_border, border_checked, self.check_progress);
        }

        let mut border_paint = Paint::default();
        border_paint.set_anti_alias(true);
        border_paint.set_style(skia_safe::PaintStyle::Stroke);
        border_paint.set_color(current_border);
        border_paint.set_stroke_width(1.0);

        canvas.draw_round_rect(
            Rect::from_xywh(
                scaled_x + 0.5,
                scaled_y + 0.5,
                scaled_size - 1.0,
                scaled_size - 1.0,
            ),
            border_radius,
            border_radius,
            &border_paint,
        );

        // Animated checkmark
        if self.check_progress > 0.0 {
            let mut check_paint = Paint::default();
            check_paint.set_anti_alias(true);
            check_paint.set_style(skia_safe::PaintStyle::Stroke);
            check_paint.set_color(Color::from_argb(
                (self.check_progress * 255.0) as u8,
                255,
                255,
                255,
            ));
            check_paint.set_stroke_width(1.5 * self.check_progress);
            check_paint.set_stroke_cap(skia_safe::PaintCap::Round);

            // Animated checkmark drawing
            let progress = self.check_progress;
            let x_base = scaled_x + (4.0 * scale);
            let y_base = scaled_y + (9.0 * scale);

            // First line (animated)
            let line1_end_x = x_base + (3.0 * scale * progress.min(0.5) * 2.0);
            let line1_end_y = y_base + (3.0 * scale * progress.min(0.5) * 2.0);
            canvas.draw_line((x_base, y_base), (line1_end_x, line1_end_y), &check_paint);

            // Second line (animated after first)
            if progress > 0.5 {
                let line2_progress = (progress - 0.5) * 2.0;
                let line2_start_x = x_base + (3.0 * scale);
                let line2_start_y = y_base + (3.0 * scale);
                let line2_end_x = line2_start_x + (7.0 * scale * line2_progress);
                let line2_end_y = line2_start_y - (6.0 * scale * line2_progress);
                canvas.draw_line(
                    (line2_start_x, line2_start_y),
                    (line2_end_x, line2_end_y),
                    &check_paint,
                );
            }
        }

        // Animated label
        let font = font_factory(13.0, 400);
        let label_alpha = 1.0 - (self.active_progress * 0.2);
        let mut text_paint = Paint::default();
        text_paint.set_anti_alias(true);
        text_paint.set_color(Color::from_argb(
            (label_alpha * 255.0) as u8,
            ZedTheme::TEXT.r(),
            ZedTheme::TEXT.g(),
            ZedTheme::TEXT.b(),
        ));

        canvas.draw_str(
            self.label,
            (self.x + 28.0, self.y + 13.0),
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
        self.checked = !self.checked;
        self.active = true;
        println!("Checkbox toggled: {}", self.checked);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
