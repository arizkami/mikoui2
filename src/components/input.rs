use skia_safe::{Canvas, Color, Font, Paint, Rect};

use crate::components::Widget;
use crate::theme::{lerp_color, ZedTheme};

pub struct Input {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    placeholder: &'static str,
    text: String,
    focused: bool,
    hover: bool,
    hover_progress: f32,
    focus_progress: f32,
    cursor_visible: bool,
    cursor_timer: f32,
    cursor_blink_speed: f32,
}

impl Input {
    pub fn new(x: f32, y: f32, width: f32, height: f32, placeholder: &'static str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            placeholder,
            text: String::new(),
            focused: false,
            hover: false,
            hover_progress: 0.0,
            focus_progress: 0.0,
            cursor_visible: true,
            cursor_timer: 0.0,
            cursor_blink_speed: 1.0,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Widget for Input {
    fn draw(&self, canvas: &Canvas, font_manager: &mut crate::core::FontManager) {
        let border_radius = 6.0;
        let padding = 12.0;

        // Animated background color
        let base_bg = ZedTheme::INPUT_BG;
        let hover_bg = ZedTheme::INPUT_HOVER;
        let focus_bg = ZedTheme::INPUT_FOCUS;

        let mut current_bg = base_bg;
        if self.hover_progress > 0.0 {
            current_bg = lerp_color(current_bg, hover_bg, self.hover_progress);
        }
        if self.focus_progress > 0.0 {
            current_bg = lerp_color(current_bg, focus_bg, self.focus_progress);
        }

        // Animated scale on focus
        let scale = 1.0 + (self.focus_progress * 0.005);
        let center_x = self.x + self.width / 2.0;
        let center_y = self.y + self.height / 2.0;
        let scaled_width = self.width * scale;
        let scaled_height = self.height * scale;
        let scaled_x = center_x - scaled_width / 2.0;
        let scaled_y = center_y - scaled_height / 2.0;

        // Draw shadow on focus
        let shadow_opacity = self.focus_progress * 0.2;
        if shadow_opacity > 0.0 {
            let mut shadow_paint = Paint::default();
            shadow_paint.set_anti_alias(true);
            shadow_paint.set_color(Color::from_argb((shadow_opacity * 255.0) as u8, 0, 0, 0));
            canvas.draw_round_rect(
                Rect::from_xywh(scaled_x + 2.0, scaled_y + 2.0, scaled_width, scaled_height),
                border_radius,
                border_radius,
                &shadow_paint,
            );
        }

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

        // Animated border
        let border_base = ZedTheme::BORDER;
        let border_hover = Color::from_argb(
            255,
            (border_base.r() as f32 * 1.3) as u8,
            (border_base.g() as f32 * 1.3) as u8,
            (border_base.b() as f32 * 1.3) as u8,
        );
        let border_focus = ZedTheme::PRIMARY;

        let mut current_border = border_base;
        if self.hover_progress > 0.0 && self.focus_progress < 0.5 {
            current_border = lerp_color(current_border, border_hover, self.hover_progress);
        }
        if self.focus_progress > 0.0 {
            current_border = lerp_color(current_border, border_focus, self.focus_progress);
        }

        let border_width = 1.0 + (self.focus_progress * 1.0);
        let mut border_paint = Paint::default();
        border_paint.set_anti_alias(true);
        border_paint.set_style(skia_safe::PaintStyle::Stroke);
        border_paint.set_color(current_border);
        border_paint.set_stroke_width(border_width);

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

        // Draw focus ring
        if self.focus_progress > 0.0 {
            let ring_opacity = self.focus_progress * 0.3;
            let mut ring_paint = Paint::default();
            ring_paint.set_anti_alias(true);
            ring_paint.set_style(skia_safe::PaintStyle::Stroke);
            ring_paint.set_color(Color::from_argb(
                (ring_opacity * 255.0) as u8,
                ZedTheme::PRIMARY.r(),
                ZedTheme::PRIMARY.g(),
                ZedTheme::PRIMARY.b(),
            ));
            ring_paint.set_stroke_width(3.0);

            canvas.draw_round_rect(
                Rect::from_xywh(
                    scaled_x - 2.0,
                    scaled_y - 2.0,
                    scaled_width + 4.0,
                    scaled_height + 4.0,
                ),
                border_radius + 2.0,
                border_radius + 2.0,
                &ring_paint,
            );
        }

        // Draw text or placeholder
        let display_text = if self.text.is_empty() {
            self.placeholder
        } else {
            &self.text
        };
        let font_weight = if self.text.is_empty() { 350 } else { 400 };
        let font = font_manager.create_font(display_text, 13.0, font_weight);

        let text_color = if self.text.is_empty() {
            ZedTheme::TEXT_MUTED
        } else {
            ZedTheme::TEXT
        };

        let mut text_paint = Paint::default();
        text_paint.set_anti_alias(true);
        text_paint.set_color(text_color);

        let text_x = scaled_x + padding;
        let text_y = scaled_y + scaled_height / 2.0 + 4.0;

        canvas.draw_str(display_text, (text_x, text_y), &font, &text_paint);

        // Animated cursor
        if self.focused && self.cursor_visible {
            let cursor_x = if self.text.is_empty() {
                text_x
            } else {
                let (text_width, _) = font.measure_str(&self.text, Some(&text_paint));
                text_x + text_width
            };

            let cursor_alpha = if self.cursor_visible { 1.0 } else { 0.0 };
            let mut cursor_paint = Paint::default();
            cursor_paint.set_anti_alias(true);
            cursor_paint.set_color(Color::from_argb(
                (cursor_alpha * 255.0) as u8,
                ZedTheme::PRIMARY.r(),
                ZedTheme::PRIMARY.g(),
                ZedTheme::PRIMARY.b(),
            ));
            cursor_paint.set_stroke_width(2.0);

            canvas.draw_line(
                (cursor_x, scaled_y + 8.0),
                (cursor_x, scaled_y + scaled_height - 8.0),
                &cursor_paint,
            );
        }
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    fn update_hover(&mut self, x: f32, y: f32) {
        self.hover = self.contains(x, y);
    }

    fn update_animation(&mut self, elapsed: f32) {
        let animation_speed = 0.15;

        // Hover animation
        let target_hover = if self.hover { 1.0 } else { 0.0 };
        if (self.hover_progress - target_hover).abs() > 0.01 {
            self.hover_progress += (target_hover - self.hover_progress) * animation_speed;
        } else {
            self.hover_progress = target_hover;
        }

        // Focus animation
        let target_focus = if self.focused { 1.0 } else { 0.0 };
        if (self.focus_progress - target_focus).abs() > 0.01 {
            self.focus_progress += (target_focus - self.focus_progress) * (animation_speed * 1.5);
        } else {
            self.focus_progress = target_focus;
        }

        // Cursor blink (faster when focused)
        self.cursor_timer = elapsed;
        let blink_speed = if self.focused { 2.5 } else { 2.0 };
        self.cursor_visible = (elapsed * blink_speed).sin() > 0.0;
    }

    fn on_click(&mut self) {
        self.focused = true;
        println!("Input focused");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
