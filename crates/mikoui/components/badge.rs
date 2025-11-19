use skia_safe::{Canvas, Color, Paint, Rect};

use crate::components::Widget;
use crate::theme::{current_theme, Theme, Variant};

pub struct Badge {
    x: f32,
    y: f32,
    text: &'static str,
    variant: Variant,
    hover: bool,
    hover_progress: f32,
}

impl Badge {
    pub fn new(x: f32, y: f32, text: &'static str) -> Self {
        Self {
            x,
            y,
            text,
            variant: Variant::Default,
            hover: false,
            hover_progress: 0.0,
        }
    }
    
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
    
    fn get_width(&self, font_manager: &mut crate::core::FontManager) -> f32 {
        let font = font_manager.create_font(self.text, Theme::TEXT_XS, 500);
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        let (text_width, _) = font.measure_str(self.text, Some(&paint));
        text_width + (Theme::SPACE_2 * 2.0)
    }
}

impl Widget for Badge {
    fn draw(&self, canvas: &Canvas, font_manager: &mut crate::core::FontManager) {
        let border_radius = Theme::RADIUS_SM;
        let height = 22.0;
        let colors = current_theme();
        
        let width = self.get_width(font_manager);

        // Colors based on variant
        let (bg_color, text_color, has_border) = match self.variant {
            Variant::Default => (colors.primary, colors.primary_foreground, false),
            Variant::Secondary => (colors.secondary, colors.secondary_foreground, false),
            Variant::Destructive => (colors.destructive, colors.destructive_foreground, false),
            Variant::Outline => (Color::TRANSPARENT, colors.foreground, true),
            _ => (colors.secondary, colors.secondary_foreground, false),
        };

        // Background
        if bg_color != Color::TRANSPARENT {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_color(bg_color);

            canvas.draw_round_rect(
                Rect::from_xywh(self.x, self.y, width, height),
                border_radius,
                border_radius,
                &paint,
            );
        }

        // Border for outline variant
        if has_border {
            let mut border_paint = Paint::default();
            border_paint.set_anti_alias(true);
            border_paint.set_style(skia_safe::PaintStyle::Stroke);
            border_paint.set_color(colors.border);
            border_paint.set_stroke_width(1.0);

            canvas.draw_round_rect(
                Rect::from_xywh(
                    self.x + 0.5,
                    self.y + 0.5,
                    width - 1.0,
                    height - 1.0,
                ),
                border_radius,
                border_radius,
                &border_paint,
            );
        }

        // Text
        let font = font_manager.create_font(self.text, Theme::TEXT_XS, 500);
        let mut text_paint = Paint::default();
        text_paint.set_anti_alias(true);
        text_paint.set_color(text_color);

        let (text_width, _) = font.measure_str(self.text, Some(&text_paint));
        let text_x = self.x + (width - text_width) / 2.0;
        let text_y = self.y + height / 2.0 + 4.0;

        canvas.draw_str(self.text, (text_x, text_y), &font, &text_paint);
    }

    fn contains(&self, _x: f32, _y: f32) -> bool {
        false // Badges are typically not interactive
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
