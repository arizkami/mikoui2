#![windows_subsystem="windows"]

mod theme;
mod components;
mod core;

use mikoui::{
    set_theme, FontManager, MenuBar, TitleBar, ThemeColors, ThemeMode, Widget, WindowControl, 
    dwm_windows,
};
use components::ActivityBar;
use core::{create_editor_menus, handle_menu_action};
use theme::{kiro::KiroTheme, vscode::VSCodeTheme, xcode::XcodeTheme};

#[cfg(target_os = "windows")]
use mikoui::core::titlebar::windows_titlebar;
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;
const TITLEBAR_HEIGHT: f32 = 34.0;

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppTheme {
    Kiro,
    VSCode,
    Xcode,
}

impl AppTheme {
    fn get_colors(&self, mode: ThemeMode) -> ThemeColors {
        match (self, mode) {
            (AppTheme::Kiro, ThemeMode::Dark) => KiroTheme::dark(),
            (AppTheme::Kiro, ThemeMode::Light) => KiroTheme::light(),
            (AppTheme::VSCode, ThemeMode::Dark) => VSCodeTheme::dark(),
            (AppTheme::VSCode, ThemeMode::Light) => VSCodeTheme::light(),
            (AppTheme::Xcode, ThemeMode::Dark) => XcodeTheme::dark(),
            (AppTheme::Xcode, ThemeMode::Light) => XcodeTheme::light(),
        }
    }
    
    fn name(&self) -> &str {
        match self {
            AppTheme::Kiro => "Kiro",
            AppTheme::VSCode => "VSCode",
            AppTheme::Xcode => "Xcode",
        }
    }
}

struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    titlebar: Option<TitleBar>,
    menubar: Option<MenuBar>,
    activitybar: Option<ActivityBar>,
    widgets: Vec<Box<dyn Widget>>,
    mouse_pos: (f32, f32),
    font_manager: FontManager,
    start_time: Instant,
    theme_colors: ThemeColors,
    theme_mode: ThemeMode,
    current_theme: AppTheme,
    is_dragging: bool,
    drag_start_pos: Option<(f32, f32)>,
    is_window_maximized: bool,
    #[cfg(target_os = "windows")]
    window_hwnd: Option<isize>,
}

impl App {
    fn new() -> Self {
        let theme_mode = ThemeMode::Dark;
        let current_theme = AppTheme::Kiro;
        let theme_colors = current_theme.get_colors(theme_mode);
        set_theme(theme_colors);
        
        Self {
            window: None,
            surface: None,
            titlebar: None,
            menubar: None,
            activitybar: None,
            widgets: Vec::new(),
            mouse_pos: (0.0, 0.0),
            font_manager: FontManager::new(),
            start_time: Instant::now(),
            theme_colors,
            theme_mode,
            current_theme,
            is_dragging: false,
            drag_start_pos: None,
            is_window_maximized: false,
            #[cfg(target_os = "windows")]
            window_hwnd: None,
        }
    }
    
    fn toggle_theme_mode(&mut self) {
        self.theme_mode = match self.theme_mode {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
        };
        self.apply_theme();
    }
    
    fn set_theme(&mut self, theme: AppTheme) {
        self.current_theme = theme;
        self.apply_theme();
    }
    
    fn apply_theme(&mut self) {
        self.theme_colors = self.current_theme.get_colors(self.theme_mode);
        set_theme(self.theme_colors);
        
        let size = self.window.as_ref().map(|w| w.inner_size());
        if let Some(size) = size {
            self.build_ui(size.width as f32, size.height as f32);
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }
    
    fn build_ui(&mut self, width: f32, _height: f32) {
        self.widgets.clear();
        
        // Create menubar with comprehensive editor menu structure
        let menus = create_editor_menus();
        
        // Create menubar first to calculate width
        let menubar = MenuBar::new(0.0, 0.0, width, menus);
        let menubar_width = menubar.total_width(&mut self.font_manager);
        self.menubar = Some(menubar);
        
        // Create titlebar with menubar
        // Get project name from current directory
        let project_name = if let Ok(current_dir) = std::env::current_dir() {
            if let Some(folder_name) = current_dir.file_name() {
                folder_name.to_string_lossy().to_string()
            } else {
                "Untitled".to_string()
            }
        } else {
            "Untitled".to_string()
        };
        
        let mut titlebar = TitleBar::new(0.0, 0.0, width, TITLEBAR_HEIGHT, &project_name)
            .with_menubar(menubar_width);
        titlebar.set_maximized(self.is_window_maximized);
        self.titlebar = Some(titlebar);
        
        // Enable Windows 11 Snap Layouts
        #[cfg(target_os = "windows")]
        if let (Some(hwnd), Some(ref titlebar)) = (self.window_hwnd, &self.titlebar) {
            let (x, y, w, h) = titlebar.get_maximize_button_bounds();
            dwm_windows::enable_snap_layouts(hwnd, (x as i32, y as i32, w as i32, h as i32));
        }
        
        // Create activity bar
        let activitybar = ActivityBar::new(0.0, TITLEBAR_HEIGHT, _height - TITLEBAR_HEIGHT);
        self.activitybar = Some(activitybar);
        
        // Content area is now blank - ready for your application content
    }
    
    fn handle_button_click(&mut self, _x: f32, _y: f32) {
        // No demo buttons - add your custom button handling here
    }
    
    fn get_clicked_menu_item_id(&self) -> Option<i32> {
        if let Some(ref menubar) = self.menubar {
            // Use Any trait to access MenuBar internals
            use std::any::Any;
            let menubar_any = menubar.as_any();
            if let Some(mb) = menubar_any.downcast_ref::<MenuBar>() {
                // Access private fields through reflection isn't possible in Rust
                // We need to add a public method to MenuBar instead
                return mb.get_clicked_item_id();
            }
        }
        None
    }
    
    fn get_window_title(&self) -> String {
        // Try to get current folder name from environment
        if let Ok(current_dir) = std::env::current_dir() {
            if let Some(folder_name) = current_dir.file_name() {
                if let Some(name) = folder_name.to_str() {
                    return format!("{} - Rabital", name);
                }
            }
        }
        
        // Default to "Untitled"
        "Untitled - Rabital".to_string()
    }
    
    #[cfg(target_os = "windows")]
    fn load_window_icon(&self) -> Option<winit::window::Icon> {
        // Load icon from embedded bytes
        const ICON_DATA: &[u8] = include_bytes!("assets/icon.ico");
        
        // Try to parse and load the icon
        match Self::parse_ico(ICON_DATA) {
            Ok(icon) => {
                println!("Icon loaded successfully");
                Some(icon)
            }
            Err(e) => {
                eprintln!("Failed to load icon: {}", e);
                // Try fallback: create a simple colored icon
                Self::create_fallback_icon().ok()
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    fn parse_ico(data: &[u8]) -> Result<winit::window::Icon, Box<dyn std::error::Error>> {
        // Try to load as ICO using image crate
        let img = image::load_from_memory_with_format(data, image::ImageFormat::Ico)?;
        
        // Convert to RGBA8
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        
        println!("Icon dimensions: {}x{}", width, height);
        
        // Create winit icon
        winit::window::Icon::from_rgba(rgba.into_raw(), width, height)
            .map_err(|e| e.into())
    }
    
    #[cfg(target_os = "windows")]
    fn create_fallback_icon() -> Result<winit::window::Icon, Box<dyn std::error::Error>> {
        // Create a simple 32x32 blue icon as fallback
        let size = 32;
        let mut rgba = vec![0u8; (size * size * 4) as usize];
        
        for y in 0..size {
            for x in 0..size {
                let idx = ((y * size + x) * 4) as usize;
                // Blue color with alpha
                rgba[idx] = 66;      // R
                rgba[idx + 1] = 135; // G
                rgba[idx + 2] = 245; // B
                rgba[idx + 3] = 255; // A
            }
        }
        
        winit::window::Icon::from_rgba(rgba, size, size)
            .map_err(|e| e.into())
    }
    
    fn render(&mut self) {
        if let (Some(window), Some(surface)) = (&self.window, &mut self.surface) {
            let size = window.inner_size();
            let (width, height) = (size.width, size.height);
            
            if width == 0 || height == 0 {
                return;
            }
            
            let width_nz = NonZeroU32::new(width).unwrap();
            let height_nz = NonZeroU32::new(height).unwrap();
            surface.resize(width_nz, height_nz).unwrap();
            
            let mut skia_surface =
                skia_safe::surfaces::raster_n32_premul((width as i32, height as i32)).unwrap();
            let canvas = skia_surface.canvas();
            
            canvas.clear(self.theme_colors.background);
            
            let elapsed = self.start_time.elapsed().as_secs_f32();
            
            // Update and draw titlebar
            if let Some(ref mut titlebar) = self.titlebar {
                titlebar.update_animation(elapsed);
                titlebar.draw(canvas, &mut self.font_manager);
            }
            
            // Update menubar animation but draw it in two passes
            if let Some(ref mut menubar) = self.menubar {
                menubar.update_animation(elapsed);
            }
            
            // Draw menubar items (without dropdown)
            if let Some(ref menubar) = self.menubar {
                menubar.draw_menubar_only(canvas, &mut self.font_manager);
            }
            
            // Update and draw activity bar
            if let Some(ref mut activitybar) = self.activitybar {
                activitybar.update_animation(elapsed);
                activitybar.draw(canvas, &mut self.font_manager);
            }
            
            // Update and draw widgets
            for widget in &mut self.widgets {
                widget.update_animation(elapsed);
                widget.draw(canvas, &mut self.font_manager);
            }
            
            // Draw menubar dropdown on top of everything
            if let Some(ref menubar) = self.menubar {
                menubar.draw_dropdown_only(canvas, &mut self.font_manager);
            }
            
            let image = skia_surface.image_snapshot();
            if let Some(pixels) = image.peek_pixels() {
                let mut buffer = surface.buffer_mut().unwrap();
                let src = pixels.bytes().unwrap();
                
                for y in 0..height as usize {
                    for x in 0..width as usize {
                        let idx = (y * width as usize + x) * 4;
                        let b = src[idx] as u32;
                        let g = src[idx + 1] as u32;
                        let r = src[idx + 2] as u32;
                        let a = src[idx + 3] as u32;
                        buffer[y * width as usize + x] = (a << 24) | (r << 16) | (g << 8) | b;
                    }
                }
                
                buffer.present().unwrap();
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            // Determine window title based on current folder/file
            let title = self.get_window_title();
            
            let window_attributes = Window::default_attributes()
                .with_title(&title)
                .with_inner_size(winit::dpi::LogicalSize::new(
                    WINDOW_WIDTH as i32,
                    WINDOW_HEIGHT as i32,
                ))
                .with_decorations(false)
                .with_resizable(true);
            
            // Set window icon
            #[cfg(target_os = "windows")]
            let window_attributes = {
                if let Some(icon) = self.load_window_icon() {
                    window_attributes.with_window_icon(Some(icon))
                } else {
                    window_attributes
                }
            };
            
            let window = Rc::new(event_loop.create_window(window_attributes).unwrap());
            
            // Apply Windows DWM effects
            if let Ok(handle) = window.window_handle() {
                if let RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
                    let hwnd = win32_handle.hwnd.get() as isize;
                    dwm_windows::apply_modern_window_style(hwnd);
                    self.window_hwnd = Some(hwnd);
                }
            }
            
            let context = Context::new(window.clone()).unwrap();
            let surface = Surface::new(&context, window.clone()).unwrap();
            
            self.window = Some(window.clone());
            self.surface = Some(surface);
            
            let size = window.inner_size();
            self.build_ui(size.width as f32, size.height as f32);
        }
    }
    
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    // Check if maximized
                    if let Some(window) = &self.window {
                        if let Some(monitor) = window.current_monitor() {
                            let monitor_size = monitor.size();
                            // Consider maximized if size is close to monitor size (within 10px)
                            let is_maximized = (size.width as i32 - monitor_size.width as i32).abs() < 10 
                                && (size.height as i32 - monitor_size.height as i32).abs() < 10;
                            
                            if is_maximized != self.is_window_maximized {
                                self.is_window_maximized = is_maximized;
                                if let Some(ref mut titlebar) = self.titlebar {
                                    titlebar.set_maximized(is_maximized);
                                }
                            }
                        }
                    }
                    
                    // Rebuild UI with new size
                    self.build_ui(size.width as f32, size.height as f32);
                    
                    // Update titlebar size
                    if let Some(ref mut titlebar) = self.titlebar {
                        titlebar.update_size(size.width as f32);
                    }
                    
                    // Request redraw
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (position.x as f32, position.y as f32);
                
                if let Some(ref mut titlebar) = self.titlebar {
                    titlebar.update_hover(self.mouse_pos.0, self.mouse_pos.1);
                }
                
                if let Some(ref mut menubar) = self.menubar {
                    menubar.update_hover_with_font(self.mouse_pos.0, self.mouse_pos.1, &mut self.font_manager);
                }
                
                if let Some(ref mut activitybar) = self.activitybar {
                    activitybar.update_hover(self.mouse_pos.0, self.mouse_pos.1);
                }
                
                for widget in &mut self.widgets {
                    widget.update_hover(self.mouse_pos.0, self.mouse_pos.1);
                }
                
                if self.is_dragging {
                    if let (Some(window), Some(drag_start)) = (&self.window, self.drag_start_pos) {
                        let delta_x = self.mouse_pos.0 - drag_start.0;
                        let delta_y = self.mouse_pos.1 - drag_start.1;
                        
                        if let Ok(current_pos) = window.outer_position() {
                            let new_x = current_pos.x + delta_x as i32;
                            let new_y = current_pos.y + delta_y as i32;
                            let _ = window.set_outer_position(winit::dpi::PhysicalPosition::new(new_x, new_y));
                        }
                    }
                }
                
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                // Check titlebar controls first
                if let Some(ref mut titlebar) = self.titlebar {
                    if titlebar.is_maximize_button(self.mouse_pos.0, self.mouse_pos.1) {
                        if let Some(window) = &self.window {
                            let new_state = !self.is_window_maximized;
                            window.set_maximized(new_state);
                            titlebar.on_click();
                        }
                        return;
                    }
                    
                    if let Some(control) = titlebar.get_clicked_control(self.mouse_pos.0, self.mouse_pos.1) {
                        if let Some(window) = &self.window {
                            match control {
                                WindowControl::Minimize => window.set_minimized(true),
                                WindowControl::Close => event_loop.exit(),
                                _ => {}
                            }
                            titlebar.on_click();
                            window.request_redraw();
                        }
                        return;
                    }
                }
                
                // Check menubar
                let clicked_item_id = self.get_clicked_menu_item_id();
                
                if let Some(ref mut menubar) = self.menubar {
                    if menubar.contains(self.mouse_pos.0, self.mouse_pos.1) {
                        menubar.on_click();
                        
                        // Handle the menu action if an item was clicked
                        if let Some(item_id) = clicked_item_id {
                            handle_menu_action(item_id);
                        }
                        
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                        return;
                    }
                }
                
                // Check activity bar
                if let Some(ref mut activitybar) = self.activitybar {
                    if activitybar.contains(self.mouse_pos.0, self.mouse_pos.1) {
                        activitybar.on_click();
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                        return;
                    }
                }
                
                // Check if draggable area (titlebar but not menubar or buttons)
                if let Some(ref titlebar) = self.titlebar {
                    if titlebar.is_draggable_area(self.mouse_pos.0, self.mouse_pos.1) {
                        // Don't start dragging if window is maximized
                        if !self.is_window_maximized {
                            self.is_dragging = true;
                            self.drag_start_pos = Some(self.mouse_pos);
                        }
                        return;
                    }
                }
                
                // Handle button clicks
                self.handle_button_click(self.mouse_pos.0, self.mouse_pos.1);
                
                for widget in &mut self.widgets {
                    if widget.contains(self.mouse_pos.0, self.mouse_pos.1) {
                        widget.on_click();
                    }
                }
                
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Left,
                ..
            } => {
                self.is_dragging = false;
                self.drag_start_pos = None;
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Right,
                ..
            } => {
                if let Some(ref titlebar) = self.titlebar {
                    if titlebar.is_draggable_area(self.mouse_pos.0, self.mouse_pos.1) {
                        #[cfg(target_os = "windows")]
                        if let Some(window) = &self.window {
                            if let Ok(handle) = window.window_handle() {
                                if let RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
                                    let hwnd = win32_handle.hwnd.get() as isize;
                                    
                                    if let Ok(pos) = window.outer_position() {
                                        let screen_x = pos.x + self.mouse_pos.0 as i32;
                                        let screen_y = pos.y + self.mouse_pos.1 as i32;
                                        
                                        windows_titlebar::show_system_menu(hwnd, screen_x, screen_y);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
