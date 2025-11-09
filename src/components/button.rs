use skia_safe::{Canvas, Color, Font, Paint, Rect};

use crate::components::Widget;
use crate::theme::{lerp_color, ZedTheme};

#[derive(Clone, Copy)]
pub enum ButtonStyle {
    Primary,
    Secondary,
}

pub struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: &'static str,
    style: ButtonStyle,
    hover: bool,
    active: bool,
    hover_progress: f32,
    active_progress: f32,
}

impl Button {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        text: &'static str,
        style: ButtonStyle,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            text,
            style,
            hover: false,
            active: false,
            hover_progress: 0.0,
            active_progress: 0.0,
        }
    }
}

impl Widget for Button {
    fn draw(&self, canvas: &Canvas, font_factory: &dyn Fn(f32, i32) -> Font) {
        let border_radius = 6.0;

        // Define colors based on style
        let (base_color, hover_color, active_color, text_color) = match self.style {
            ButtonStyle::Primary => (
                ZedTheme::PRIMARY,
                ZedTheme::PRIMARY_HOVER,
                ZedTheme::PRIMARY_ACTIVE,
                Color::WHITE,
            ),
            ButtonStyle::Secondary => (
                ZedTheme::SURFACE,
                ZedTheme::INPUT_HOVER,
                ZedTheme::ELEVATED,
                ZedTheme::TEXT,
            ),
        };

        // Interpolate colors
        let mut current_bg = base_color;
        if self.hover_progress > 0.0 {
            current_bg = lerp_color(current_bg, hover_color, self.hover_progress);
        }
        if self.active_progress > 0.0 {
            current_bg = lerp_color(current_bg, active_color, self.active_progress);
        }

        // Draw shadow
        let shadow_opacity = (self.hover_progress * 0.15) + (self.active_progress * 0.1);
        if shadow_opacity > 0.0 {
            let shadow_offset_y = 2.0 + (self.hover_progress * 2.0) - (self.active_progress * 1.0);
            let mut shadow_paint = Paint::default();
            shadow_paint.set_anti_alias(true);
            shadow_paint.set_color(Color::from_argb(
                (shadow_opacity * 255.0) as u8,
                0,
                0,
                0,
            ));

            canvas.draw_round_rect(
                Rect::from_xywh(self.x + 1.0, self.y + shadow_offset_y, self.width, self.height),
                border_radius,
                border_radius,
                &shadow_paint,
            );
        }

        // Animated scale
        let scale = 1.0 - (self.active_progress * 0.02);
        let center_x = self.x + self.width / 2.0;
        let center_y = self.y + self.height / 2.0;
        let scaled_width = self.width * scale;
        let scaled_height = self.height * scale;
        let scaled_x = center_x - scaled_width / 2.0;
        let scaled_y = center_y - scaled_height / 2.0;

        // Draw background
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(current_bg);

        canvas.draw_round_rect(
            Rect::from_xywh(scaled_x, scaled_y, scaled_width, scaled_height),
            border_radius,
            border_radius,
            &paint,
        );

        // Draw border for secondary
        if matches!(self.style, ButtonStyle::Secondary) {
            let border_base = ZedTheme::BORDER;
            let border_hover = ZedTheme::BORDER_FOCUS;
            let current_border = lerp_color(border_base, border_hover, self.hover_progress);

            let mut border_paint = Paint::default();
            border_paint.set_anti_alias(true);
            border_paint.set_style(skia_safe::PaintStyle::Stroke);
            border_paint.set_color(current_border);
            border_paint.set_stroke_width(1.0);

            canvas.draw_round_rect(
                Rect::from_xywh(
                    scaled_x + 0.5,
                    scaled_y + 0.5,
                    scaled_width - 1.0,
                    scaled_height - 1.0,
                ),
                border_radius,
                border_radius,
                &border_paint,
            );
        }

        // Draw text
        let font_weight = if matches!(self.style, ButtonStyle::Primary) {
            500
        } else {
            450
        };
        let font = font_factory(13.0, font_weight);

        let mut text_paint = Paint::default();
        text_paint.set_anti_alias(true);
        text_paint.set_color(text_color);

        let (text_width, _) = font.measure_str(self.text, Some(&text_paint));
        let text_x = scaled_x + (scaled_width - text_width) / 2.0;
        let text_y = scaled_y + scaled_height / 2.0 + 4.0;

        canvas.draw_str(self.text, (text_x, text_y), &font, &text_paint);
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    fn update_hover(&mut self, x: f32, y: f32) {
        self.hover = self.contains(x, y);
    }

    fn update_animation(&mut self, _elapsed: f32) {
        let animation_speed = 0.15;

        // Hover animation
        let target_hover = if self.hover { 1.0 } else { 0.0 };
        if (self.hover_progress - target_hover).abs() > 0.01 {
            self.hover_progress += (target_hover - self.hover_progress) * animation_speed;
        } else {
            self.hover_progress = target_hover;
        }

        // Active animation
        let target_active = if self.active { 1.0 } else { 0.0 };
        if (self.active_progress - target_active).abs() > 0.01 {
            self.active_progress += (target_active - self.active_progress) * (animation_speed * 2.0);
        } else {
            self.active_progress = target_active;
        }
    }

    fn on_click(&mut self) {
        println!("Button clicked: {}", self.text);
        self.active = true;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
