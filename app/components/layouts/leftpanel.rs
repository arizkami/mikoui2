use mikoui::{Widget, FontManager};
use mikoui::theme::current_theme;
use skia_safe::{Canvas, Color, Paint, Rect};
use crate::pages::Explorer;

const RESIZE_HANDLE_WIDTH: f32 = 4.0;
const MIN_WIDTH: f32 = 200.0;
const MAX_WIDTH: f32 = 600.0;
const HEADER_HEIGHT: f32 = 32.0;

pub struct LeftPanel {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_resizing: bool,
    hover_resize: bool,
    explorer: Explorer,
}

impl LeftPanel {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let clamped_width = width.clamp(MIN_WIDTH, MAX_WIDTH);
        let explorer = Explorer::new(
            x,
            y + HEADER_HEIGHT,
            clamped_width,
            height - HEADER_HEIGHT,
        );
        
        Self {
            x,
            y,
            width: clamped_width,
            height,
            is_resizing: false,
            hover_resize: false,
            explorer,
        }
    }
    
    pub fn new_with_path(x: f32, y: f32, width: f32, height: f32, root_path: std::path::PathBuf) -> Self {
        println!("LeftPanel::new_with_path called with: {}", root_path.display());
        let clamped_width = width.clamp(MIN_WIDTH, MAX_WIDTH);
        let explorer = crate::pages::Explorer::new_with_path(
            x,
            y + HEADER_HEIGHT,
            clamped_width,
            height - HEADER_HEIGHT,
            root_path,
        );
        
        Self {
            x,
            y,
            width: clamped_width,
            height,
            is_resizing: false,
            hover_resize: false,
            explorer,
        }
    }
    
    pub fn width(&self) -> f32 {
        self.width
    }
    
    pub fn set_height(&mut self, height: f32) {
        self.height = height;
        self.explorer.set_bounds(
            self.x,
            self.y + HEADER_HEIGHT,
            self.width,
            height - HEADER_HEIGHT,
        );
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
        self.explorer.set_bounds(
            self.x,
            self.y + HEADER_HEIGHT,
            new_width,
            self.height - HEADER_HEIGHT,
        );
    }
    
    pub fn is_resizing(&self) -> bool {
        self.is_resizing
    }
    
    pub fn explorer(&self) -> &Explorer {
        &self.explorer
    }
    
    pub fn explorer_mut(&mut self) -> &mut Explorer {
        &mut self.explorer
    }
    
    pub fn handle_mouse_press(&mut self, x: f32, y: f32) {
        // Check if clicking on scrollbar
        if self.explorer.is_over_scrollbar(x, y) {
            self.explorer.start_scrollbar_drag(y);
        }
    }
    
    pub fn handle_mouse_drag(&mut self, y: f32) {
        self.explorer.handle_scrollbar_drag(y);
    }
    
    pub fn handle_mouse_release(&mut self) {
        self.explorer.stop_scrollbar_drag();
    }
    
    pub fn is_scrollbar_dragging(&self) -> bool {
        self.explorer.is_scrollbar_dragging()
    }
    
    pub fn take_clicked_file(&mut self) -> Option<std::path::PathBuf> {
        self.explorer.take_clicked_file()
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
        
        // Header - show "EXPLORER" label
        let text = "EXPLORER";
        let font = font_manager.create_font(text, 11.0, 600);
        let mut text_paint = Paint::default();
        text_paint.set_color(theme.muted_foreground);
        text_paint.set_anti_alias(true);
        
        canvas.draw_str(
            text,
            (self.x + 16.0, self.y + 20.0),
            &font,
            &text_paint,
        );
        
        // Show current folder path if available
        if self.explorer.has_root() {
            let folder_name = self.explorer.get_root_name();
            let folder_font = font_manager.create_font(&folder_name, 12.0, 400);
            let mut folder_paint = Paint::default();
            folder_paint.set_color(theme.foreground);
            folder_paint.set_anti_alias(true);
            
            // Draw folder name on the right side of header
            let text_width = folder_font.measure_str(&folder_name, Some(&folder_paint)).0;
            let x_pos = self.x + self.width - text_width - 16.0;
            
            canvas.draw_str(
                &folder_name,
                (x_pos, self.y + 20.0),
                &folder_font,
                &folder_paint,
            );
        }
        
        // Draw explorer
        self.explorer.draw(canvas, font_manager);
    }
    
    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
    
    fn update_hover(&mut self, x: f32, y: f32) {
        self.hover_resize = self.is_over_resize_handle(x, y);
        
        // Update explorer hover if not resizing
        if !self.hover_resize {
            self.explorer.update_hover(x, y);
        }
    }
    
    fn update_animation(&mut self, _elapsed: f32) {
        self.explorer.update_animation(_elapsed);
    }
    
    fn on_click(&mut self) {
        // Forward click to explorer
        self.explorer.on_click();
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
