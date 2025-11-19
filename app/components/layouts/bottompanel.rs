use mikoui::{Widget, FontManager};
use mikoui::theme::current_theme;
use skia_safe::{Canvas, Color, Paint, Rect};

const RESIZE_HANDLE_HEIGHT: f32 = 4.0;
const MIN_HEIGHT: f32 = 100.0;
const MAX_HEIGHT: f32 = 500.0;

pub struct BottomPanel {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_resizing: bool,
    hover_resize: bool,
}

impl BottomPanel {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height: height.clamp(MIN_HEIGHT, MAX_HEIGHT),
            is_resizing: false,
            hover_resize: false,
        }
    }
    
    pub fn height(&self) -> f32 {
        self.height
    }
    
    pub fn set_position(&mut self, y: f32) {
        self.y = y;
    }
    
    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }
    
    pub fn resize_handle_rect(&self) -> Rect {
        Rect::from_xywh(
            self.x,
            self.y - RESIZE_HANDLE_HEIGHT / 2.0,
            self.width,
            RESIZE_HANDLE_HEIGHT,
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
    
    pub fn resize_to(&mut self, y: f32, window_height: f32) {
        let new_height = (window_height - y).clamp(MIN_HEIGHT, MAX_HEIGHT);
        self.height = new_height;
        self.y = window_height - self.height;
    }
    
    pub fn is_resizing(&self) -> bool {
        self.is_resizing
    }
}

impl Widget for BottomPanel {
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
            (self.x, self.y),
            (self.x + self.width, self.y),
            &border_paint,
        );
        
        // Resize handle
        if self.hover_resize || self.is_resizing {
            let mut handle_paint = Paint::default();
            let alpha = if self.is_resizing { 100 } else { 50 };
            handle_paint.set_color(Color::from_argb(alpha, 100, 150, 255));
            handle_paint.set_anti_alias(true);
            
            let handle_rect = self.resize_handle_rect();
            canvas.draw_rect(handle_rect, &handle_paint);
        }
        
        // Placeholder content
        let text = "Bottom Panel (Terminal / Output)";
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
