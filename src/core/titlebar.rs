use skia_safe::{Canvas, Color, Paint, Rect};
use crate::core::FontManager;
use crate::components::Widget;
use crate::theme::current_theme;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::{
        GetSystemMenu, TrackPopupMenu, TPM_RETURNCMD, TPM_RIGHTBUTTON,
        SendMessageW, WM_SYSCOMMAND,
    },
};

/// Window control button types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowControl {
    Minimize,
    Maximize,
    Restore,
    Close,
}

/// Windows-specific titlebar features
#[cfg(target_os = "windows")]
pub mod windows_titlebar {
    use super::*;
    
    /// Show the system menu (right-click menu on titlebar)
    pub fn show_system_menu(hwnd: isize, x: i32, y: i32) -> bool {
        unsafe {
            let hwnd = HWND(hwnd as *mut std::ffi::c_void);
            let hmenu = GetSystemMenu(hwnd, false);
            
            if hmenu.is_invalid() {
                return false;
            }
            
            let cmd = TrackPopupMenu(
                hmenu,
                TPM_RETURNCMD | TPM_RIGHTBUTTON,
                x,
                y,
                0,
                hwnd,
                None,
            );
            
            if cmd.0 != 0 {
                SendMessageW(hwnd, WM_SYSCOMMAND, WPARAM(cmd.0 as usize), LPARAM(0));
                true
            } else {
                false
            }
        }
    }
    
    /// Enable Windows 11 snap layouts on maximize button
    /// This allows the snap layout overlay to appear when hovering over maximize
    pub fn enable_snap_layouts(_hwnd: isize) -> bool {
        // Windows 11 automatically shows snap layouts for standard windows
        // For custom titlebars, we need to handle WM_NCHITTEST to return HTMAXBUTTON
        // This is typically done in the window procedure, not here
        // For now, we'll just return true as a placeholder
        true
    }
}

#[cfg(not(target_os = "windows"))]
pub mod windows_titlebar {
    pub fn show_system_menu(_hwnd: isize, _x: i32, _y: i32) -> bool {
        false
    }
    
    pub fn enable_snap_layouts(_hwnd: isize) -> bool {
        false
    }
}

/// Window control button SVG icons
impl WindowControl {
    pub const fn svg_content(self) -> &'static str {
        match self {
            WindowControl::Minimize => include_str!("wco/minimize.svg"),
            WindowControl::Maximize => include_str!("wco/maximize.svg"),
            WindowControl::Restore => include_str!("wco/restore.svg"),
            WindowControl::Close => include_str!("wco/close.svg"),
        }
    }
}

/// Window control button widget
pub struct WindowControlButton {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    control_type: WindowControl,
    hover: bool,
    hover_progress: f32,
    active: bool,
    active_progress: f32,
    cached_image: std::cell::RefCell<Option<std::sync::Arc<skia_safe::Image>>>,
}

impl WindowControlButton {
    pub fn new(x: f32, y: f32, width: f32, height: f32, control_type: WindowControl) -> Self {
        Self {
            x,
            y,
            width,
            height,
            control_type,
            hover: false,
            hover_progress: 0.0,
            active: false,
            active_progress: 0.0,
            cached_image: std::cell::RefCell::new(None),
        }
    }
    
    pub fn control_type(&self) -> WindowControl {
        self.control_type
    }
    
    pub fn set_control_type(&mut self, control_type: WindowControl) {
        self.control_type = control_type;
        // Always clear cache to force reload
        *self.cached_image.borrow_mut() = None;
    }
    
    fn load_svg(&self) -> Option<skia_safe::Image> {
        let svg_content = self.control_type.svg_content();
        let opt = usvg::Options::default();
        let tree = usvg::Tree::from_str(svg_content, &opt).ok()?;
        
        let target_size = 10u32; // Icon size
        let mut pixmap = tiny_skia::Pixmap::new(target_size, target_size)?;
        
        let svg_size = tree.size();
        let scale_x = target_size as f32 / svg_size.width();
        let scale_y = target_size as f32 / svg_size.height();
        let scale = scale_x.min(scale_y);
        
        let transform = tiny_skia::Transform::from_scale(scale, scale);
        resvg::render(&tree, transform, &mut pixmap.as_mut());
        
        let image_info = skia_safe::ImageInfo::new(
            (target_size as i32, target_size as i32),
            skia_safe::ColorType::RGBA8888,
            skia_safe::AlphaType::Premul,
            None,
        );
        
        skia_safe::Image::from_raster_data(
            &image_info,
            skia_safe::Data::new_copy(pixmap.data()),
            target_size as usize * 4,
        )
    }
}

impl Widget for WindowControlButton {
    fn draw(&self, canvas: &Canvas, _font_manager: &mut FontManager) {
        let theme = current_theme();
        
        // Background with hover effect
        let mut bg_paint = Paint::default();
        bg_paint.set_anti_alias(true);
        
        if self.control_type == WindowControl::Close {
            // Red background for close button on hover
            let hover_color = Color::from_argb(
                (self.hover_progress * 255.0) as u8,
                220,
                38,
                38,
            );
            bg_paint.set_color(hover_color);
        } else {
            // Subtle gray background for other buttons
            let hover_alpha = (self.hover_progress * 0.1 * 255.0) as u8;
            bg_paint.set_color(Color::from_argb(
                hover_alpha,
                255,
                255,
                255,
            ));
        }
        
        let rect = Rect::from_xywh(self.x, self.y, self.width, self.height);
        canvas.draw_rect(rect, &bg_paint);
        
        // Load and draw icon
        if self.cached_image.borrow().is_none() {
            if let Some(img) = self.load_svg() {
                *self.cached_image.borrow_mut() = Some(std::sync::Arc::new(img));
            }
        }
        
        if let Some(ref image) = *self.cached_image.borrow() {
            let icon_size = 10.0;
            let icon_x = self.x + (self.width - icon_size) / 2.0;
            let icon_y = self.y + (self.height - icon_size) / 2.0;
            
            let scale = 1.0 - (self.active_progress * 0.1);
            
            canvas.save();
            canvas.translate((icon_x + icon_size / 2.0, icon_y + icon_size / 2.0));
            canvas.scale((scale, scale));
            canvas.translate((-icon_size / 2.0, -icon_size / 2.0));
            
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            
            // Icon color
            let icon_color = if self.control_type == WindowControl::Close && self.hover {
                Color::WHITE
            } else {
                theme.foreground
            };
            
            let color_filter = skia_safe::color_filters::blend(
                icon_color,
                skia_safe::BlendMode::SrcIn,
            );
            paint.set_color_filter(color_filter);
            
            let dest_rect = Rect::from_xywh(0.0, 0.0, icon_size, icon_size);
            canvas.draw_image_rect(image.as_ref(), None, dest_rect, &paint);
            
            canvas.restore();
        }
    }
    
    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
    
    fn update_hover(&mut self, x: f32, y: f32) {
        self.hover = self.contains(x, y);
    }
    
    fn update_animation(&mut self, _elapsed: f32) {
        let animation_speed = 0.2;
        
        let target_hover = if self.hover { 1.0 } else { 0.0 };
        if (self.hover_progress - target_hover).abs() > 0.01 {
            self.hover_progress += (target_hover - self.hover_progress) * animation_speed;
        } else {
            self.hover_progress = target_hover;
        }
        
        let target_active = if self.active { 1.0 } else { 0.0 };
        if (self.active_progress - target_active).abs() > 0.01 {
            self.active_progress += (target_active - self.active_progress) * (animation_speed * 2.0);
        } else {
            self.active_progress = target_active;
        }
        
        if self.active && self.active_progress > 0.9 {
            self.active = false;
        }
    }
    
    fn on_click(&mut self) {
        self.active = true;
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Custom titlebar with window controls
pub struct TitleBar {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    title: String,
    minimize_btn: WindowControlButton,
    maximize_btn: WindowControlButton,
    close_btn: WindowControlButton,
    is_maximized: bool,
}

impl TitleBar {
    pub fn new(x: f32, y: f32, width: f32, height: f32, title: &str) -> Self {
        let button_width = 46.0;
        let button_height = height;
        
        let close_x = x + width - button_width;
        let maximize_x = close_x - button_width;
        let minimize_x = maximize_x - button_width;
        
        Self {
            x,
            y,
            width,
            height,
            title: title.to_string(),
            minimize_btn: WindowControlButton::new(minimize_x, y, button_width, button_height, WindowControl::Minimize),
            maximize_btn: WindowControlButton::new(maximize_x, y, button_width, button_height, WindowControl::Maximize),
            close_btn: WindowControlButton::new(close_x, y, button_width, button_height, WindowControl::Close),
            is_maximized: false,
        }
    }
    
    pub fn set_maximized(&mut self, maximized: bool) {
        self.is_maximized = maximized;
        let control_type = if maximized {
            WindowControl::Restore
        } else {
            WindowControl::Maximize
        };
        self.maximize_btn.set_control_type(control_type);
    }
    
    pub fn is_maximized(&self) -> bool {
        self.is_maximized
    }
    
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }
    
    pub fn update_size(&mut self, width: f32) {
        self.width = width;
        let button_width = 46.0;
        
        let close_x = self.x + width - button_width;
        let maximize_x = close_x - button_width;
        let minimize_x = maximize_x - button_width;
        
        self.close_btn.x = close_x;
        self.maximize_btn.x = maximize_x;
        self.minimize_btn.x = minimize_x;
    }
    
    /// Check if a point is in the draggable area (not on buttons)
    pub fn is_draggable_area(&self, x: f32, y: f32) -> bool {
        if !self.contains(x, y) {
            return false;
        }
        
        !self.minimize_btn.contains(x, y) 
            && !self.maximize_btn.contains(x, y) 
            && !self.close_btn.contains(x, y)
    }
    
    /// Get the bounds of the maximize button (for Windows 11 snap layouts)
    pub fn get_maximize_button_bounds(&self) -> (f32, f32, f32, f32) {
        (
            self.maximize_btn.x,
            self.maximize_btn.y,
            self.maximize_btn.width,
            self.maximize_btn.height,
        )
    }
    
    /// Check if clicking on maximize/restore button
    pub fn is_maximize_button(&self, x: f32, y: f32) -> bool {
        self.maximize_btn.contains(x, y)
    }
    
    /// Get which control button was clicked, if any
    pub fn get_clicked_control(&self, x: f32, y: f32) -> Option<WindowControl> {
        if self.minimize_btn.contains(x, y) {
            Some(WindowControl::Minimize)
        } else if self.maximize_btn.contains(x, y) {
            if self.is_maximized {
                Some(WindowControl::Restore)
            } else {
                Some(WindowControl::Maximize)
            }
        } else if self.close_btn.contains(x, y) {
            Some(WindowControl::Close)
        } else {
            None
        }
    }
}

impl Widget for TitleBar {
    fn draw(&self, canvas: &Canvas, font_manager: &mut FontManager) {
        let theme = current_theme();
        
        // Background
        let mut bg_paint = Paint::default();
        bg_paint.set_anti_alias(true);
        bg_paint.set_color(theme.card);
        
        let rect = Rect::from_xywh(self.x, self.y, self.width, self.height);
        canvas.draw_rect(rect, &bg_paint);
        
        // Title text
        let title_x = self.x + 12.0;
        let font_size = 12.0;
        let title_y = self.y + (self.height + font_size) / 2.0 - 4.0;
        
        let font = font_manager.create_font(&self.title, font_size, 600);
        let mut text_paint = Paint::default();
        text_paint.set_anti_alias(true);
        text_paint.set_color(theme.foreground);
        
        canvas.draw_str(&self.title, (title_x, title_y), &font, &text_paint);
        
        // Draw window control buttons
        self.minimize_btn.draw(canvas, font_manager);
        self.maximize_btn.draw(canvas, font_manager);
        self.close_btn.draw(canvas, font_manager);
        
        // Bottom border
        let mut border_paint = Paint::default();
        border_paint.set_anti_alias(true);
        border_paint.set_color(theme.border);
        border_paint.set_stroke_width(1.0);
        border_paint.set_style(skia_safe::PaintStyle::Stroke);
        
        canvas.draw_line(
            (self.x, self.y + self.height),
            (self.x + self.width, self.y + self.height),
            &border_paint,
        );
    }
    
    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
    
    fn update_hover(&mut self, x: f32, y: f32) {
        self.minimize_btn.update_hover(x, y);
        self.maximize_btn.update_hover(x, y);
        self.close_btn.update_hover(x, y);
    }
    
    fn update_animation(&mut self, elapsed: f32) {
        self.minimize_btn.update_animation(elapsed);
        self.maximize_btn.update_animation(elapsed);
        self.close_btn.update_animation(elapsed);
    }
    
    fn on_click(&mut self) {
        // Handled by get_clicked_control
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
