use mikoui::{Widget, FontManager};
use mikoui::theme::current_theme;
use skia_safe::{Canvas, Color, Paint, Rect};

const RESIZE_HANDLE_WIDTH: f32 = 4.0;
const MIN_WIDTH: f32 = 200.0;
const MAX_WIDTH: f32 = 600.0;

pub struct LeftPanel {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_resizing: bool,
    hover_resize: bool,
}

impl LeftPanel {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width: width.clamp(MIN_WIDTH, MAX_WIDTH),
            height,
            is_resizing: false,
            hover_resize: false,
        }
    }
    
    pub fn width(&self) -> f32 {
        self.width
    }
    
    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }
    
    pub fn resize_handle_rect(&self) -> Rect {
        Rect::from_xywh(
            self.x + self.width - RESIZE_HANDLE_WIDTH / 2.0,
            self.y,
            RESIZE_HANDLE_WIDTH,
            self.height,
        )
    }
    
    pub fn is_over_resize_handle(&self, x: f32, y: f32) -> bool {
        let handle = self.resize_handle_rect();
        x >= handle.left && x <= handle.right && y >= handle.top && y <= handle.bottom
    }
    
    pub fn start_resize(&mut self) {
        self.is_resizing = true;
    }
    
    pub fn stop_resize(&mut self) {
        self.is_resizing = false;
    }
    
    pub fn resize_to(&mut self, x: f32) {
        let new_width = (x - self.x).clamp(MIN_WIDTH, MAX_WIDTH);
        self.width = new_width;
    }
    
    pub fn is_resizing(&self) -> bool {
        self.is_resizing
    }
}

impl Widget for LeftPanel {
    fn draw(&self, canvas: &Canvas, font_manager: &mut FontManager) {
        let theme = current_theme();
        
        // Background
        let mut bg_paint = Paint::default();
        bg_paint.set_color(theme.card);
        bg_paint.set_anti_alias(true);
        
        let panel_rect = Rect::from_xywh(self.x, self.y, self.width, self.height);
        canvas.draw_rect(panel_rect, &bg_paint);
        
        // Border
        let mut border_paint = Paint::default();
        border_paint.set_color(theme.border);
        border_paint.set_style(skia_safe::PaintStyle::Stroke);
        border_paint.set_stroke_width(1.0);
        border_paint.set_anti_alias(true);
        
        canvas.draw_line(
            (self.x + self.width, self.y),
            (self.x + self.width, self.y + self.height),
            &border_paint,
        );
        
        // Resize handle (visual indicator when hovering)
        if self.hover_resize || self.is_resizing {
            let mut handle_paint = Paint::default();
            let alpha = if self.is_resizing { 100 } else { 50 };
            handle_paint.set_color(Color::from_argb(alpha, 100, 150, 255));
            handle_paint.set_anti_alias(true);
            
            let handle_rect = self.resize_handle_rect();
            canvas.draw_rect(handle_rect, &handle_paint);
        }
        
        // Placeholder content
        let text = "Left Panel";
        let font = font_manager.create_font(text, 14.0, 400);
        let mut text_paint = Paint::default();
        text_paint.set_color(theme.muted_foreground);
        text_paint.set_anti_alias(true);
        
        canvas.draw_str(
            text,
            (self.x + 16.0, self.y + 32.0),
            &font,
            &text_paint,
        );
    }
    
    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
    
    fn update_hover(&mut self, x: f32, y: f32) {
        self.hover_resize = self.is_over_resize_handle(x, y);
    }
    
    fn update_animation(&mut self, _elapsed: f32) {
        // No animation for now
    }
    
    fn on_click(&mut self) {
        // Handle clicks if needed
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
