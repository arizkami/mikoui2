use mikoui::{
    set_theme, Button, Card, FontManager, Label, TitleBar, ThemeColors,
    Variant, Widget, WindowControl, Size, dwm_windows,
};

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

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const TITLEBAR_HEIGHT: f32 = 32.0;

struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    titlebar: Option<TitleBar>,
    widgets: Vec<Box<dyn Widget>>,
    mouse_pos: (f32, f32),
    font_manager: FontManager,
    start_time: Instant,
    theme_colors: ThemeColors,
    is_dragging: bool,
    drag_start_pos: Option<(f32, f32)>,
    is_window_maximized: bool,
    #[cfg(target_os = "windows")]
    window_hwnd: Option<isize>,
}

impl App {
    fn new() -> Self {
        let theme_colors = ThemeColors::dark();
        set_theme(theme_colors);
        
        Self {
            window: None,
            surface: None,
            titlebar: None,
            widgets: Vec::new(),
            mouse_pos: (0.0, 0.0),
            font_manager: FontManager::new(),
            start_time: Instant::now(),
            theme_colors,
            is_dragging: false,
            drag_start_pos: None,
            is_window_maximized: false,
            #[cfg(target_os = "windows")]
            window_hwnd: None,
        }
    }
    
    fn build_ui(&mut self, width: f32, _height: f32) {
        self.widgets.clear();
        
        // Create titlebar
        let mut titlebar = TitleBar::new(
            0.0,
            0.0,
            width,
            TITLEBAR_HEIGHT,
            "Borderless Window Example",
        );
        
        // Restore maximized state after creating new titlebar
        titlebar.set_maximized(self.is_window_maximized);
        
        self.titlebar = Some(titlebar);
        
        // Enable Windows 11 Snap Layouts
        #[cfg(target_os = "windows")]
        if let (Some(hwnd), Some(ref titlebar)) = (self.window_hwnd, &self.titlebar) {
            let (x, y, w, h) = titlebar.get_maximize_button_bounds();
            dwm_windows::enable_snap_layouts(hwnd, (x as i32, y as i32, w as i32, h as i32));
        }
        
        // Content area below titlebar
        let content_y = TITLEBAR_HEIGHT + 20.0;
        let padding = 32.0;
        
        // Welcome card
        let card_width = width - padding * 2.0;
        let card_height = 200.0;
        self.widgets.push(Box::new(Card::new(
            padding,
            content_y,
            card_width,
            card_height,
        )));
        
        self.widgets.push(Box::new(Label::new(
            padding + 20.0,
            content_y + 20.0,
            "Custom Borderless Window",
            24.0,
            700,
            self.theme_colors.foreground,
        )));
        
        self.widgets.push(Box::new(Label::new(
            padding + 20.0,
            content_y + 60.0,
            "This window has a custom titlebar with window controls.",
            16.0,
            400,
            self.theme_colors.muted_foreground,
        )));
        
        self.widgets.push(Box::new(Label::new(
            padding + 20.0,
            content_y + 90.0,
            "• Drag the titlebar to move the window",
            14.0,
            400,
            self.theme_colors.muted_foreground,
        )));
        
        self.widgets.push(Box::new(Label::new(
            padding + 20.0,
            content_y + 115.0,
            "• Click minimize, maximize, or close buttons",
            14.0,
            400,
            self.theme_colors.muted_foreground,
        )));
        
        self.widgets.push(Box::new(Label::new(
            padding + 20.0,
            content_y + 140.0,
            "• Double-click titlebar to maximize/restore",
            14.0,
            400,
            self.theme_colors.muted_foreground,
        )));
        
        // Action buttons
        let button_y = content_y + card_height + 20.0;
        let button_width = 150.0;
        let button_gap = 12.0;
        
        self.widgets.push(Box::new(
            Button::new(padding, button_y, button_width, "Primary Action")
                .variant(Variant::Default)
                .size(Size::Md),
        ));
        
        self.widgets.push(Box::new(
            Button::new(
                padding + button_width + button_gap,
                button_y,
                button_width,
                "Secondary",
            )
            .variant(Variant::Secondary)
            .size(Size::Md),
        ));
        
        self.widgets.push(Box::new(
            Button::new(
                padding + (button_width + button_gap) * 2.0,
                button_y,
                button_width,
                "Outline",
            )
            .variant(Variant::Outline)
            .size(Size::Md),
        ));
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
            
            // Update and draw widgets
            for widget in &mut self.widgets {
                widget.update_animation(elapsed);
                widget.draw(canvas, &mut self.font_manager);
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
            let window_attributes = Window::default_attributes()
                .with_title("Borderless Window Example")
                .with_inner_size(winit::dpi::LogicalSize::new(
                    WINDOW_WIDTH as i32,
                    WINDOW_HEIGHT as i32,
                ))
                .with_decorations(false); // Remove default window decorations
            
            let window = Rc::new(event_loop.create_window(window_attributes).unwrap());
            
            // Apply Windows DWM effects (rounded corners + shadow)
            if let Ok(handle) = window.window_handle() {
                if let RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
                    let hwnd = win32_handle.hwnd.get() as isize;
                    if dwm_windows::apply_modern_window_style(hwnd) {
                        println!("✓ Applied DWM effects: rounded corners + shadow");
                    } else {
                        println!("⚠ Failed to apply DWM effects");
                    }
                    
                    // Enable Windows 11 Snap Layouts
                    // We'll update this after creating the titlebar
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
    
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    // Detect maximize state by comparing with monitor size
                    if let Some(window) = &self.window {
                        if let Some(monitor) = window.current_monitor() {
                            let monitor_size = monitor.size();
                            let is_maximized = size.width >= monitor_size.width && size.height >= monitor_size.height;
                            
                            // Update state only if it changed
                            if is_maximized != self.is_window_maximized {
                                self.is_window_maximized = is_maximized;
                                if let Some(ref mut titlebar) = self.titlebar {
                                    titlebar.set_maximized(is_maximized);
                                }
                            }
                        }
                    }
                    
                    self.build_ui(size.width as f32, size.height as f32);
                    
                    if let Some(ref mut titlebar) = self.titlebar {
                        titlebar.update_size(size.width as f32);
                    }
                    
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (position.x as f32, position.y as f32);
                
                // Update hover states
                if let Some(ref mut titlebar) = self.titlebar {
                    titlebar.update_hover(self.mouse_pos.0, self.mouse_pos.1);
                }
                
                for widget in &mut self.widgets {
                    widget.update_hover(self.mouse_pos.0, self.mouse_pos.1);
                }
                
                // Handle window dragging
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
                // Check titlebar controls
                if let Some(ref mut titlebar) = self.titlebar {
                    // Check if clicking on maximize/restore button to toggle state
                    if titlebar.is_maximize_button(self.mouse_pos.0, self.mouse_pos.1) {
                        if let Some(window) = &self.window {
                            // Let Windows handle the state change - don't toggle manually
                            let new_state = !self.is_window_maximized;
                            window.set_maximized(new_state);
                            titlebar.on_click();
                        }
                        return;
                    }
                    
                    if let Some(control) = titlebar.get_clicked_control(self.mouse_pos.0, self.mouse_pos.1) {
                        if let Some(window) = &self.window {
                            match control {
                                WindowControl::Minimize => {
                                    window.set_minimized(true);
                                }
                                WindowControl::Close => {
                                    event_loop.exit();
                                }
                                _ => {} // Maximize/Restore handled above
                            }
                            titlebar.on_click();
                            window.request_redraw();
                        }
                        return;
                    }
                    
                    // Check if clicking in draggable area
                    if titlebar.is_draggable_area(self.mouse_pos.0, self.mouse_pos.1) {
                        self.is_dragging = true;
                        self.drag_start_pos = Some(self.mouse_pos);
                        return;
                    }
                }
                
                // Handle widget clicks
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
                // Show system menu on right-click in titlebar
                if let Some(ref titlebar) = self.titlebar {
                    if titlebar.is_draggable_area(self.mouse_pos.0, self.mouse_pos.1) {
                        #[cfg(target_os = "windows")]
                        if let Some(window) = &self.window {
                            if let Ok(handle) = window.window_handle() {
                                if let RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
                                    let hwnd = win32_handle.hwnd.get() as isize;
                                    
                                    // Get screen coordinates for the menu
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
