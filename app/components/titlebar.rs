use skia_safe::{Canvas, Color, Paint, Rect};
use mikoui::core::FontManager;
use mikoui::components::{Widget, Icon, IconSize, CodiconIcons};
use mikoui::theme::current_theme;

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

/// Layout toggle button types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutButton {
    LeftPanel,
    BottomPanel,
    RightPanel,
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
                None, // reserved parameter
                hwnd,
                None,
            );
            
            if cmd.as_bool() {
                let _ = SendMessageW(hwnd, WM_SYSCOMMAND, Some(WPARAM(cmd.0 as usize)), Some(LPARAM(0)));
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
    project_name: String,
    minimize_btn: WindowControlButton,
    maximize_btn: WindowControlButton,
    close_btn: WindowControlButton,
    is_maximized: bool,
    show_menubar: bool,
    menubar_width: f32,
    search_text: String,
    search_focused: bool,
    search_icon_hover: bool,
    search_icon_hover_progress: f32,
    command_palette_open: bool,
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
            project_name: title.to_string(), // Use the title parameter as project name
            minimize_btn: WindowControlButton::new(minimize_x, y, button_width, button_height, WindowControl::Minimize),
            maximize_btn: WindowControlButton::new(maximize_x, y, button_width, button_height, WindowControl::Maximize),
            close_btn: WindowControlButton::new(close_x, y, button_width, button_height, WindowControl::Close),
            is_maximized: false,
            show_menubar: false,
            menubar_width: 0.0,
            search_text: String::new(),
            search_focused: false,
            search_icon_hover: false,
            search_icon_hover_progress: 0.0,
            command_palette_open: false,
        }
    }
    
    pub fn set_project_name(&mut self, name: &str) {
        self.project_name = name.to_string();
    }
    
    pub fn with_menubar(mut self, menubar_width: f32) -> Self {
        self.show_menubar = true;
        self.menubar_width = menubar_width;
        self
    }
    
    pub fn menubar_width(&self) -> f32 {
        self.menubar_width
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
    
    pub fn set_command_palette_open(&mut self, open: bool) {
        self.command_palette_open = open;
    }
    
    pub fn is_search_bar_clicked(&self, x: f32, y: f32) -> bool {
        let (search_x, search_y, search_w, search_h) = self.get_search_bar_bounds();
        x >= search_x && x <= search_x + search_w && y >= search_y && y <= search_y + search_h
    }
    
    fn get_search_bar_bounds(&self) -> (f32, f32, f32, f32) {
        let left_start = self.x + self.menubar_width + 16.0;
        let right_end = self.minimize_btn.x - 16.0;
        let layout_buttons_width = 100.0;
        let layout_start_pos = right_end - layout_buttons_width;
        let available_width = layout_start_pos - left_start;
        
        let max_search_width = 400.0;
        let search_height = 26.0;
        let search_center_x = left_start + available_width / 2.0;
        let search_start = search_center_x - max_search_width / 2.0;
        let center_y = self.y + self.height / 2.0;
        
        (search_start, center_y - search_height / 2.0, max_search_width, search_height)
    }
    
    fn get_search_icon_bounds(&self) -> (f32, f32, f32, f32) {
        let (search_x, search_y, _search_w, search_h) = self.get_search_bar_bounds();
        let icon_size = 16.0;
        let icon_x = search_x + 8.0;
        let icon_y = search_y + (search_h - icon_size) / 2.0;
        (icon_x, icon_y, icon_size, icon_size)
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
    
    /// Check if a point is in the draggable area (not on buttons or menubar)
    pub fn is_draggable_area(&self, x: f32, y: f32) -> bool {
        // Must be within titlebar bounds
        if !self.contains(x, y) {
            return false;
        }
        
        // Exclude window control buttons
        if self.minimize_btn.contains(x, y) 
            || self.maximize_btn.contains(x, y) 
            || self.close_btn.contains(x, y) {
            return false;
        }
        
        // Exclude menubar area if enabled (only the left portion where menu items are)
        // Add some padding to make it easier to drag
        if self.show_menubar && x >= self.x && x <= self.x + self.menubar_width {
            return false;
        }
        
        // Everything else in the titlebar is draggable
        true
    }
    
    /// Check if a point is in the menubar area
    pub fn is_menubar_area(&self, x: f32, y: f32) -> bool {
        if !self.show_menubar {
            return false;
        }
        
        self.contains(x, y) && x >= self.x && x <= self.x + self.menubar_width
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
    
    /// Get which layout button was clicked, if any
    pub fn get_clicked_layout_button(&self, x: f32, y: f32) -> Option<LayoutButton> {
        let right_end = self.minimize_btn.x - 16.0;
        let layout_buttons_width = 100.0;
        let layout_button_size = 28.0;
        let layout_button_gap = 4.0;
        let layout_start = right_end - layout_buttons_width + 8.0;
        let center_y = self.y + self.height / 2.0;
        
        // Check each layout button
        for i in 0..3 {
            let button_x = layout_start + (i as f32 * (layout_button_size + layout_button_gap));
            let button_rect = Rect::from_xywh(
                button_x,
                center_y - layout_button_size / 2.0,
                layout_button_size,
                layout_button_size,
            );
            
            if x >= button_rect.left && x <= button_rect.right 
                && y >= button_rect.top && y <= button_rect.bottom {
                return match i {
                    0 => Some(LayoutButton::LeftPanel),
                    1 => Some(LayoutButton::BottomPanel),
                    2 => Some(LayoutButton::RightPanel),
                    _ => None,
                };
            }
        }
        
        None
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
        
        // Calculate available space
        let left_start = self.x + self.menubar_width + 16.0;
        let right_end = self.minimize_btn.x - 16.0;
        let center_y = self.y + self.height / 2.0;
        
        // Layout toggle buttons width (positioned on the right)
        let layout_buttons_width = 100.0;
        let layout_start_pos = right_end - layout_buttons_width;
        
        // Center section: Navigation buttons + Search bar
        let nav_button_size = 24.0;
        let nav_button_gap = 4.0;
        let gap_between_nav_and_search = 8.0;
        let max_search_width = 400.0;
        let search_height = 26.0;
        
        // Calculate available width for centering
        let available_width = layout_start_pos - left_start;
        
        // Center the search bar in the available space
        let search_center_x = left_start + available_width / 2.0;
        let search_start = search_center_x - max_search_width / 2.0;
        
        // Position navigation buttons to the left of the search bar
        let forward_x = search_start - gap_between_nav_and_search - nav_button_size;
        let back_x = forward_x - nav_button_gap - nav_button_size;
        
        // Draw back button background
        // let back_rect = Rect::from_xywh(back_x, center_y - nav_button_size / 2.0, nav_button_size, nav_button_size);
        // let mut nav_paint = Paint::default();
        // nav_paint.set_anti_alias(true);
        // nav_paint.set_color(theme.muted);
        // canvas.draw_round_rect(back_rect, 4.0, 4.0, &nav_paint);
        
        // Draw back icon
        let back_icon = Icon::new(
            back_x + 0.0,
            center_y - 8.0,
            CodiconIcons::CHEVRON_LEFT,
            IconSize::Small,
            theme.muted_foreground,
        );
        back_icon.draw(canvas, font_manager);
        
        // Draw forward button background
        // let forward_rect = Rect::from_xywh(forward_x, center_y - nav_button_size / 2.0, nav_button_size, nav_button_size);
        // canvas.draw_round_rect(forward_rect, 4.0, 4.0, &nav_paint);
        
        // Draw forward icon
        let forward_icon = Icon::new(
            forward_x + 0.0,
            center_y - 8.0,
            CodiconIcons::CHEVRON_RIGHT,
            IconSize::Small,
            theme.muted_foreground,
        );
        forward_icon.draw(canvas, font_manager);
        
        // Search bar - already calculated above
        let search_width = max_search_width;
        let search_rect = Rect::from_xywh(
            search_start,
            center_y - search_height / 2.0,
            search_width,
            search_height,
        );
        
        // Calculate opacity based on command palette state
        let search_opacity = if self.command_palette_open { 0.0 } else { 1.0 };
        
        // Draw hover background on entire search bar
        if self.search_icon_hover_progress > 0.0 {
            let hover_alpha = (30.0 * self.search_icon_hover_progress * search_opacity) as u8;
            let mut hover_paint = Paint::default();
            hover_paint.set_anti_alias(true);
            let muted = theme.muted;
            hover_paint.set_color(Color::from_argb(hover_alpha, muted.r(), muted.g(), muted.b()));
            canvas.draw_round_rect(search_rect, 4.0, 4.0, &hover_paint);
        }
        
        // Search bar background
        let mut search_bg = Paint::default();
        search_bg.set_anti_alias(true);
        let input_color = theme.input;
        let bg_alpha = (input_color.a() as f32 * search_opacity) as u8;
        search_bg.set_color(Color::from_argb(bg_alpha, input_color.r(), input_color.g(), input_color.b()));
        canvas.draw_round_rect(search_rect, 4.0, 4.0, &search_bg);
        
        // Search bar border
        let mut search_border = Paint::default();
        search_border.set_anti_alias(true);
        let border_color = theme.border;
        let border_alpha = (border_color.a() as f32 * search_opacity) as u8;
        search_border.set_color(Color::from_argb(border_alpha, border_color.r(), border_color.g(), border_color.b()));
        search_border.set_style(skia_safe::PaintStyle::Stroke);
        search_border.set_stroke_width(1.0);
        canvas.draw_round_rect(search_rect, 4.0, 4.0, &search_border);
        
        // Draw search icon inside the search bar
        let (icon_x, icon_y, _icon_w, _icon_h) = self.get_search_icon_bounds();
        let muted_fg = theme.muted_foreground;
        let icon_alpha = (muted_fg.a() as f32 * search_opacity) as u8;
        let icon_color = Color::from_argb(icon_alpha, muted_fg.r(), muted_fg.g(), muted_fg.b());
        let search_icon = Icon::new(
            icon_x,
            icon_y,
            CodiconIcons::SEARCH,
            IconSize::Small,
            icon_color,
        );
        search_icon.draw(canvas, font_manager);
        
        // Project name and search text
        let search_font = font_manager.create_font(&self.project_name, 12.0, 400);
        let mut search_text_paint = Paint::default();
        search_text_paint.set_anti_alias(true);
        let fg_color = theme.foreground;
        let text_alpha = (fg_color.a() as f32 * search_opacity) as u8;
        search_text_paint.set_color(Color::from_argb(text_alpha, fg_color.r(), fg_color.g(), fg_color.b()));
        canvas.draw_str(
            &self.project_name,
            (search_start + 36.0, center_y + 4.0),
            &search_font,
            &search_text_paint,
        );
        
        // Layout toggle buttons - positioned on the right
        let layout_button_size = 28.0;
        let layout_button_gap = 4.0;
        let layout_start = right_end - layout_buttons_width + 8.0;
        
        // Layout button icons: sidebar-left, sidebar-right, panel-bottom
        let layout_icons = [
            CodiconIcons::LAYOUT_SIDEBAR_LEFT,
            CodiconIcons::LAYOUT_PANEL,
            CodiconIcons::LAYOUT_SIDEBAR_RIGHT,
        ];
        
        for (i, icon) in layout_icons.iter().enumerate() {
            let button_x = layout_start + (i as f32 * (layout_button_size + layout_button_gap));
            let button_rect = Rect::from_xywh(
                button_x,
                center_y - layout_button_size / 2.0,
                layout_button_size,
                layout_button_size,
            );
            
            // Button background
            // let mut layout_paint = Paint::default();
            // layout_paint.set_anti_alias(true);
            // layout_paint.set_color(theme.muted);
            // canvas.draw_round_rect(button_rect, 4.0, 4.0, &layout_paint);
            
            // Button icon
            let layout_icon = Icon::new(
                button_x + 6.0,
                center_y - 8.0,
                icon,
                IconSize::Small,
                theme.muted_foreground,
            );
            layout_icon.draw(canvas, font_manager);
        }
        
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
        
        // Update search bar hover (entire search bar is hoverable)
        self.search_icon_hover = self.is_search_bar_clicked(x, y);
    }
    
    fn update_animation(&mut self, elapsed: f32) {
        self.minimize_btn.update_animation(elapsed);
        self.maximize_btn.update_animation(elapsed);
        self.close_btn.update_animation(elapsed);
        
        // Animate search icon hover
        let target = if self.search_icon_hover { 1.0 } else { 0.0 };
        let animation_speed = 0.2;
        if (self.search_icon_hover_progress - target).abs() > 0.01 {
            self.search_icon_hover_progress += (target - self.search_icon_hover_progress) * animation_speed;
        } else {
            self.search_icon_hover_progress = target;
        }
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
