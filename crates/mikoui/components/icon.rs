use skia_safe::{Canvas, Color, Image, Paint, Rect};
use std::sync::Arc;
use std::cell::RefCell;

use crate::components::Widget;
use crate::core::FontManager;

#[derive(Clone, Copy, PartialEq)]
pub enum IconSize {
    Small = 16,
    Medium = 24,
    Large = 32,
    XLarge = 48,
}

impl IconSize {
    pub fn as_f32(self) -> f32 {
        self as i32 as f32
    }
}

pub struct Icon {
    x: f32,
    y: f32,
    size: IconSize,
    color: Color,
    svg_content: &'static str,
    cached_image: RefCell<Option<Arc<Image>>>,
    hover: bool,
    hover_progress: f32,
    active: bool,
    active_progress: f32,
}

impl Icon {
    pub fn new(x: f32, y: f32, svg_content: &'static str, size: IconSize, color: Color) -> Self {
        Self {
            x,
            y,
            size,
            color,
            svg_content,
            cached_image: RefCell::new(None),
            hover: false,
            hover_progress: 0.0,
            active: false,
            active_progress: 0.0,
        }
    }
    
    fn load_svg(&self) -> Option<Image> {
        // Parse SVG from embedded content
        let opt = usvg::Options::default();
        let tree = usvg::Tree::from_str(self.svg_content, &opt).ok()?;
        
        // Get the target size for rendering
        let target_size = self.size.as_f32() as u32;
        
        // Create a pixmap to render into
        let mut pixmap = tiny_skia::Pixmap::new(target_size, target_size)?;
        
        // Calculate transform to fit the icon in the target size
        let svg_size = tree.size();
        let scale_x = target_size as f32 / svg_size.width();
        let scale_y = target_size as f32 / svg_size.height();
        let scale = scale_x.min(scale_y);
        
        let transform = tiny_skia::Transform::from_scale(scale, scale);
        
        // Render the SVG
        resvg::render(&tree, transform, &mut pixmap.as_mut());
        
        // Convert to Skia image
        let image_info = skia_safe::ImageInfo::new(
            (target_size as i32, target_size as i32),
            skia_safe::ColorType::RGBA8888,
            skia_safe::AlphaType::Premul,
            None,
        );
        
        Image::from_raster_data(
            &image_info,
            skia_safe::Data::new_copy(pixmap.data()),
            target_size as usize * 4,
        )
    }
}

impl Widget for Icon {
    fn draw(&self, canvas: &Canvas, _font_manager: &mut FontManager) {
        // Load SVG if not cached
        if self.cached_image.borrow().is_none() {
            if let Some(img) = self.load_svg() {
                *self.cached_image.borrow_mut() = Some(Arc::new(img));
            }
        }
        
        if let Some(ref image) = *self.cached_image.borrow() {
            // Animated scale
            let scale = 1.0 - (self.active_progress * 0.1) + (self.hover_progress * 0.1);
            let size = self.size.as_f32();
            let center_x = self.x + size / 2.0;
            let center_y = self.y + size / 2.0;

            canvas.save();
            
            // Apply transformations
            canvas.translate((center_x, center_y));
            canvas.scale((scale, scale));
            canvas.translate((-size / 2.0, -size / 2.0));

            // Animated alpha
            let alpha = (1.0 - self.active_progress * 0.3 + self.hover_progress * 0.2).clamp(0.0, 1.0);
            
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_alpha_f(alpha);
            
            // Apply color filter to change icon color
            let color_filter = skia_safe::color_filters::blend(
                self.color,
                skia_safe::BlendMode::SrcIn,
            );
            paint.set_color_filter(color_filter);

            // Draw the image scaled to the icon size
            let dest_rect = Rect::from_xywh(0.0, 0.0, size, size);
            canvas.draw_image_rect(image.as_ref(), None, dest_rect, &paint);

            canvas.restore();
        }
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        let size = self.size.as_f32();
        x >= self.x && x <= self.x + size && y >= self.y && y <= self.y + size
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

        // Active animation
        let target_active = if self.active { 1.0 } else { 0.0 };
        if (self.active_progress - target_active).abs() > 0.01 {
            self.active_progress += (target_active - self.active_progress) * (animation_speed * 2.0);
        } else {
            self.active_progress = target_active;
        }

        // Reset active state
        if self.active && self.active_progress > 0.9 {
            self.active = false;
        }
    }

    fn on_click(&mut self) {
        self.active = true;
        println!("Icon clicked");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
