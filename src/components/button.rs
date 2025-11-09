use skia_safe::{Canvas, Color, Paint, Rect};

use crate::components::Widget;
use crate::theme::{current_theme, lerp_color, with_alpha, Size, Theme, Variant};

pub struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: &'static str,
    variant: Variant,
    size: Size,
    hover: bool,
    active: bool,
    hover_progress: f32,
    active_progress: f32,
    disabled: bool,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, text: &'static str) -> Self {
        Self {
            x,
            y,
            width,
            height: Size::Md.height(),
            text,
            variant: Variant::Default,
            size: Size::Md,
            hover: false,
            active: false,
            hover_progress: 0.0,
            active_progress: 0.0,
            disabled: false,
        }
    }
    
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self.height = size.height();
        self
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Widget for Button {
    fn draw(&self, canvas: &Canvas, font_manager: &mut crate::core::FontManager) {
        let border_radius = Theme::RADIUS_MD;
        let font_size = self.size.font_size();
        let colors = current_theme();

        // Get colors based on variant
        let (base_bg, hover_bg, text_color, has_border) = match self.variant {
            Variant::Default => (
                colors.primary,
                with_alpha(colors.primary, 230),
                colors.primary_foreground,
                false,
            ),
            Variant::Destructive => (
                colors.destructive,
                with_alpha(colors.destructive, 230),
                colors.destructive_foreground,
                false,
            ),
            Variant::Outline => (
                Color::TRANSPARENT,
                colors.accent,
                colors.accent_foreground,
                true,
            ),
            Variant::Secondary => (
                colors.secondary,
                with_alpha(colors.secondary, 200),
                colors.secondary_foreground,
                false,
            ),
            Variant::Ghost => (
                Color::TRANSPARENT,
                colors.accent,
                colors.accent_foreground,
                false,
            ),
            Variant::Link => (
                Color::TRANSPARENT,
                Color::TRANSPARENT,
                colors.primary,
                false,
            ),
        };

        // Apply disabled state
        let (current_bg, current_text) = if self.disabled {
            (with_alpha(base_bg, 128), with_alpha(text_color, 128))
        } else {
            let bg = if self.hover_progress > 0.0 {
                lerp_color(base_bg, hover_bg, self.hover_progress)
            } else {
                base_bg
            };
            (bg, text_color)
        };

        // Animated scale on press
        let scale = 1.0 - (self.active_progress * 0.02);
        let center_x = self.x + self.width / 2.0;
        let center_y = self.y + self.height / 2.0;
        let scaled_width = self.width * scale;
        let scaled_height = self.height * scale;
        let scaled_x = center_x - scaled_width / 2.0;
        let scaled_y = center_y - scaled_height / 2.0;

        // Draw background
        if current_bg != Color::TRANSPARENT {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_color(current_bg);

            canvas.draw_round_rect(
                Rect::from_xywh(scaled_x, scaled_y, scaled_width, scaled_height),
                border_radius,
                border_radius,
                &paint,
            );
        }

        // Draw border for outline variant
        if has_border {
            let border_color = if self.disabled {
                with_alpha(colors.border, 128)
            } else {
                colors.border
            };
            
            let mut border_paint = Paint::default();
            border_paint.set_anti_alias(true);
            border_paint.set_style(skia_safe::PaintStyle::Stroke);
            border_paint.set_color(border_color);
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
        let font_weight = match self.variant {
            Variant::Default | Variant::Destructive => 500,
            _ => 450,
        };
        let font = font_manager.create_font(self.text, font_size, font_weight);

        let mut text_paint = Paint::default();
        text_paint.set_anti_alias(true);
        text_paint.set_color(current_text);

        // Underline for link variant on hover
        if matches!(self.variant, Variant::Link) && self.hover_progress > 0.5 {
            let (text_width, _) = font.measure_str(self.text, Some(&text_paint));
            let text_x = scaled_x + (scaled_width - text_width) / 2.0;
            let underline_y = scaled_y + scaled_height / 2.0 + 8.0;
            
            let mut underline_paint = Paint::default();
            underline_paint.set_anti_alias(true);
            underline_paint.set_color(current_text);
            underline_paint.set_stroke_width(1.0);
            
            canvas.draw_line(
                (text_x, underline_y),
                (text_x + text_width, underline_y),
                &underline_paint,
            );
        }

        let (text_width, _) = font.measure_str(self.text, Some(&text_paint));
        let text_x = scaled_x + (scaled_width - text_width) / 2.0;
        let text_y = scaled_y + scaled_height / 2.0 + (font_size * 0.3);

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

        // Active animation - automatically reset after reaching peak
        let target_active = if self.active { 1.0 } else { 0.0 };
        if (self.active_progress - target_active).abs() > 0.01 {
            self.active_progress += (target_active - self.active_progress) * (animation_speed * 2.0);
        } else {
            self.active_progress = target_active;
            // Reset active state after animation completes
            if self.active && self.active_progress >= 0.9 {
                self.active = false;
            }
        }
    }

    fn on_click(&mut self) {
        if !self.disabled {
            println!("Button clicked: {}", self.text);
            self.active = true;
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
