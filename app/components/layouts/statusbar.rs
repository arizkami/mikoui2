use mikoui::{current_theme, Widget};
use skia_safe::{Canvas, Font, Paint, Rect};

pub struct StatusBar {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    language: String,
    cursor_line: usize,
    cursor_column: usize,
}

impl StatusBar {
    const HEIGHT: f32 = 24.0;
    
    pub fn new(x: f32, y: f32, width: f32) -> Self {
        Self {
            x,
            y,
            width,
            height: Self::HEIGHT,
            language: "Text".to_string(),
            cursor_line: 1,
            cursor_column: 1,
        }
    }
    
    pub fn height(&self) -> f32 {
        self.height
    }
    
    pub fn set_bounds(&mut self, x: f32, y: f32, width: f32) {
        self.x = x;
        self.y = y;
        self.width = width;
    }
    
    pub fn update_editor_info(&mut self, language: String, cursor_line: usize, cursor_column: usize) {
        self.language = language;
        self.cursor_line = cursor_line;
        self.cursor_column = cursor_column;
    }
}

impl Widget for StatusBar {
    fn draw(&self, canvas: &Canvas, font_manager: &mut mikoui::FontManager) {
        let theme = current_theme();
        
        // Background
        let mut bg_paint = Paint::default();
        bg_paint.set_color(theme.primary);
        bg_paint.set_anti_alias(true);
        canvas.draw_rect(
            Rect::from_xywh(self.x, self.y, self.width, self.height),
            &bg_paint,
        );
        
        // Create font for text
        let font = font_manager.create_font("", 13.0, 400);
        
        // Text paint
        let mut text_paint = Paint::default();
        text_paint.set_color(theme.primary_foreground);
        text_paint.set_anti_alias(true);
        
        // Language indicator (left side)
        canvas.draw_str(
            &self.language,
            (self.x + 10.0, self.y + 16.0),
            &font,
            &text_paint,
        );
        
        // Cursor position (right side)
        let cursor_info = format!("Ln {}, Col {}", self.cursor_line, self.cursor_column);
        let cursor_info_width = font.measure_str(&cursor_info, None).0;
        canvas.draw_str(
            &cursor_info,
            (self.x + self.width - cursor_info_width - 10.0, self.y + 16.0),
            &font,
            &text_paint,
        );
    }
    
    fn update_hover(&mut self, _x: f32, _y: f32) {
        // Status bar doesn't have hover states
    }
    
    fn on_click(&mut self) {
        // Status bar doesn't handle clicks
    }
    
    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
    
    fn update_animation(&mut self, _elapsed: f32) {
        // No animations
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
