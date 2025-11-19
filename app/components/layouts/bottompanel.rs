use mikoui::{Widget, FontManager};
use mikoui::theme::current_theme;
use skia_safe::{Canvas, Color, Paint, Rect};
use mikoterminal::{Terminal, TerminalConfig, TerminalRenderer};

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
    terminal: Option<Terminal>,
    terminal_renderer: TerminalRenderer,
}

impl BottomPanel {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let terminal_renderer = TerminalRenderer::new(14.0);
        
        // Don't start terminal immediately - it will be started on first update
        // This avoids issues with PTY creation on the main thread
        
        Self {
            x,
            y,
            width,
            height: height.clamp(MIN_HEIGHT, MAX_HEIGHT),
            is_resizing: false,
            hover_resize: false,
            terminal: None,
            terminal_renderer,
        }
    }
    
    /// Initialize terminal (call this after panel is created)
    pub fn init_terminal(&mut self) {
        if self.terminal.is_some() {
            return; // Already initialized
        }
        
        // Create terminal with config
        let mut config = TerminalConfig::default();
        config.font_size = 14.0;
        
        // Calculate rows and cols based on panel size
        let (cell_width, cell_height) = self.terminal_renderer.cell_size();
        config.cols = ((self.width - 32.0) / cell_width).max(20.0) as u16;
        config.rows = ((self.height - 48.0) / cell_height).max(5.0) as u16;
        
        let mut terminal = Terminal::new(config);
        
        // Try to start the terminal
        match terminal.start() {
            Ok(_) => {
                println!("Terminal started successfully");
                self.terminal = Some(terminal);
            }
            Err(e) => {
                eprintln!("Failed to start terminal: {}", e);
                // Keep terminal as None - will show error message
            }
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
        bg_paint.set_color(Color::from_rgb(12, 12, 12)); // Dark terminal background
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
        
        // Header
        let text = "Terminal";
        let font = font_manager.create_font(text, 12.0, 600);
        let mut text_paint = Paint::default();
        text_paint.set_color(theme.foreground);
        text_paint.set_anti_alias(true);
        
        canvas.draw_str(
            text,
            (self.x + 16.0, self.y + 24.0),
            &font,
            &text_paint,
        );
        
        // Render terminal or show message
        if let Some(ref terminal) = self.terminal {
            self.terminal_renderer.render(
                terminal,
                canvas,
                self.x + 16.0,
                self.y + 40.0,
            );
        } else {
            // Show initialization message
            let msg = "Terminal initializing...";
            let font = font_manager.create_font(msg, 12.0, 400);
            let mut msg_paint = Paint::default();
            msg_paint.set_color(theme.muted_foreground);
            msg_paint.set_anti_alias(true);
            
            canvas.draw_str(
                msg,
                (self.x + 16.0, self.y + 60.0),
                &font,
                &msg_paint,
            );
        }
    }
    
    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
    
    fn update_hover(&mut self, x: f32, y: f32) {
        self.hover_resize = self.is_over_resize_handle(x, y);
    }
    
    fn update_animation(&mut self, _elapsed: f32) {
        // Initialize terminal on first update if not already done
        if self.terminal.is_none() {
            self.init_terminal();
        }
        
        // Update terminal
        if let Some(ref mut terminal) = self.terminal {
            let _ = terminal.update();
        }
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
