use skia_safe::{Canvas, Paint, Rect};

use crate::components::Widget;
use crate::theme::{current_theme, lerp_color, Theme};

/// Lightweight skeleton/loading placeholder with pulse animation
pub struct Skeleton {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    border_radius: f32,
    pulse_speed: f32,
    pulse_value: f32,
}

impl Skeleton {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            border_radius: Theme::RADIUS_MD,
            pulse_speed: 1.5,
            pulse_value: 0.0,
        }
    }

    /// Adjust the corner radius for rectangular skeletons
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Configure the pulse animation speed multiplier
    pub fn pulse_speed(mut self, speed: f32) -> Self {
        self.pulse_speed = speed.max(0.2);
        self
    }

    /// Helper to turn the skeleton into a circle
    pub fn circle(mut self, diameter: f32) -> Self {
        self.width = diameter;
        self.height = diameter;
        self.border_radius = diameter / 2.0;
        self
    }

    /// Convenience constructor for circular skeletons
    pub fn new_circle(x: f32, y: f32, diameter: f32) -> Self {
        Self::new(x, y, diameter, diameter).circle(diameter)
    }
}

impl Widget for Skeleton {
    fn draw(&self, canvas: &Canvas, _font_manager: &mut crate::core::FontManager) {
        let colors = current_theme();
        let base = colors.muted;
        let highlight = lerp_color(colors.muted, colors.background, 0.25);
        let fill = lerp_color(base, highlight, self.pulse_value);

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(fill);

        canvas.draw_round_rect(
            Rect::from_xywh(self.x, self.y, self.width, self.height),
            self.border_radius,
            self.border_radius,
            &paint,
        );
    }

    fn contains(&self, _x: f32, _y: f32) -> bool {
        false
    }

    fn update_hover(&mut self, _x: f32, _y: f32) {}

    fn update_animation(&mut self, elapsed: f32) {
        let phase = elapsed * self.pulse_speed;
        self.pulse_value = (phase.sin() + 1.0) * 0.5;
    }

    fn on_click(&mut self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
