use mikoui::{
    Button, ButtonStyle, Checkbox, ContextMenu, Dropdown, Icon, IconSize, Input, Label, 
    LucideIcons, MenuItem, Panel, ProgressBar, Slider, Widget, FontManager, ZedTheme
};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};
use skia_safe::Color;

struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    widgets: Vec<Box<dyn Widget>>,
    mouse_pos: (f32, f32),
    font_manager: FontManager,
    start_time: Instant,
    dragging_slider: Option<usize>,
    focused_input: Option<usize>,
    context_menu_index: Option<usize>,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            surface: None,
            widgets: Vec::new(),
            mouse_pos: (0.0, 0.0),
            font_manager: FontManager::new(),
            start_time: Instant::now(),
            dragging_slider: None,
            focused_input: None,
            context_menu_index: None,
        }
    }

    fn build_ui(&mut self) {
        self.widgets.clear();
        
        // Background panel
        self.widgets.push(Box::new(Panel::new(0.0, 0.0, 800.0, 600.0)));

        // Title
        self.widgets.push(Box::new(Label::new(
            50.0,
            30.0,
            "Miko UI - Direct Skia Example",
            24.0,
            700,
            Color::WHITE,
        )));

        // Subtitle
        self.widgets.push(Box::new(Label::new(
            50.0,
            65.0,
            "Building UI directly with Skia components",
            16.0,
            400,
            ZedTheme::TEXT_DIM,
        )));

        // Input field
        self.widgets.push(Box::new(Input::new(
            50.0,
            100.0,
            300.0,
            40.0,
            "Enter your name...",
        )));

        // Buttons section
        self.widgets.push(Box::new(Label::new(
            50.0,
            160.0,
            "Buttons:",
            18.0,
            600,
            Color::WHITE,
        )));

        // Primary button
        self.widgets.push(Box::new(Button::new(
            50.0,
            190.0,
            120.0,
            40.0,
            "Primary",
            ButtonStyle::Primary,
        )));

        // Secondary button
        self.widgets.push(Box::new(Button::new(
            180.0,
            190.0,
            120.0,
            40.0,
            "Secondary",
            ButtonStyle::Secondary,
        )));

        // Icons section
        self.widgets.push(Box::new(Label::new(
            50.0,
            250.0,
            "Icons:",
            18.0,
            600,
            Color::WHITE,
        )));

        // Icon row
        let icon_y = 280.0;
        let icon_spacing = 50.0;
        
        self.widgets.push(Box::new(Icon::new(
            50.0,
            icon_y,
            LucideIcons::HOUSE,
            IconSize::Medium,
            ZedTheme::PRIMARY,
        )));

        self.widgets.push(Box::new(Icon::new(
            50.0 + icon_spacing,
            icon_y,
            LucideIcons::SEARCH,
            IconSize::Medium,
            ZedTheme::PRIMARY,
        )));

        self.widgets.push(Box::new(Icon::new(
            50.0 + icon_spacing * 2.0,
            icon_y,
            LucideIcons::SETTINGS,
            IconSize::Medium,
            ZedTheme::PRIMARY,
        )));

        self.widgets.push(Box::new(Icon::new(
            50.0 + icon_spacing * 3.0,
            icon_y,
            LucideIcons::USER,
            IconSize::Medium,
            ZedTheme::PRIMARY,
        )));

        self.widgets.push(Box::new(Icon::new(
            50.0 + icon_spacing * 4.0,
            icon_y,
            LucideIcons::HEART,
            IconSize::Medium,
            Color::from_rgb(239, 68, 68), // red
        )));

        // Checkbox
        self.widgets.push(Box::new(Label::new(
            50.0,
            340.0,
            "Checkbox:",
            18.0,
            600,
            Color::WHITE,
        )));

        self.widgets.push(Box::new(Checkbox::new(
            50.0,
            370.0,
            "Enable notifications",
        )));

        self.widgets.push(Box::new(Checkbox::new(
            50.0,
            410.0,
            "Dark mode",
        )));

        // Slider
        self.widgets.push(Box::new(Label::new(
            400.0,
            100.0,
            "Slider:",
            18.0,
            600,
            Color::WHITE,
        )));

        self.widgets.push(Box::new(Slider::new(
            400.0,
            130.0,
            300.0,
            "Volume",
            0.5,
        )));

        // Progress bar
        self.widgets.push(Box::new(Label::new(
            400.0,
            180.0,
            "Progress:",
            18.0,
            600,
            Color::WHITE,
        )));

        self.widgets.push(Box::new(ProgressBar::new(
            400.0,
            210.0,
            300.0,
            0.7,
        )));

        // Dropdown
        self.widgets.push(Box::new(Label::new(
            400.0,
            260.0,
            "Dropdown:",
            18.0,
            600,
            Color::WHITE,
        )));

        self.widgets.push(Box::new(Dropdown::new(
            400.0,
            290.0,
            200.0,
            "Theme",
            vec![
                "Dark".to_string(),
                "Light".to_string(),
                "Auto".to_string(),
            ],
        )));

        // Context menu (initially hidden)
        let menu_items = vec![
            MenuItem::new("Cut", 1).with_shortcut("Ctrl+X"),
            MenuItem::new("Copy", 2).with_shortcut("Ctrl+C"),
            MenuItem::new("Paste", 3).with_shortcut("Ctrl+V"),
            MenuItem::separator(),
            MenuItem::new("Select All", 4).with_shortcut("Ctrl+A"),
            MenuItem::separator(),
            MenuItem::new("Delete", 5).disabled(),
        ];
        self.widgets.push(Box::new(ContextMenu::new(0.0, 0.0, menu_items)));

        // Action buttons at bottom
        self.widgets.push(Box::new(Button::new(
            50.0,
            520.0,
            100.0,
            40.0,
            "Save",
            ButtonStyle::Primary,
        )));

        self.widgets.push(Box::new(Button::new(
            160.0,
            520.0,
            100.0,
            40.0,
            "Cancel",
            ButtonStyle::Secondary,
        )));

        // Store context menu index
        self.context_menu_index = Some(self.widgets.len() - 3);
    }

    fn render(&mut self) {
        if let (Some(window), Some(surface)) = (&self.window, &mut self.surface) {
            let (width, height) = {
                let size = window.inner_size();
                (size.width, size.height)
            };

            if width == 0 || height == 0 {
                return;
            }

            let width_nz = NonZeroU32::new(width).unwrap();
            let height_nz = NonZeroU32::new(height).unwrap();

            surface.resize(width_nz, height_nz).unwrap();

            let mut skia_surface = skia_safe::surfaces::raster_n32_premul((width as i32, height as i32)).unwrap();
            let canvas = skia_surface.canvas();

            // Clear background
            canvas.clear(Color::from_rgb(31, 41, 55)); // gray-800

            // Update animations
            let elapsed = self.start_time.elapsed().as_secs_f32();
            for widget in &mut self.widgets {
                widget.update_animation(elapsed);
            }

            // Draw widgets
            for widget in &mut self.widgets {
                widget.draw(canvas, &mut self.font_manager);
            }

            // Copy to softbuffer
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
                .with_title("Miko UI - Direct Skia")
                .with_inner_size(winit::dpi::LogicalSize::new(800, 600));

            let window = Rc::new(event_loop.create_window(window_attributes).unwrap());
            let context = Context::new(window.clone()).unwrap();
            let surface = Surface::new(&context, window.clone()).unwrap();

            self.window = Some(window.clone());
            self.surface = Some(surface);

            self.build_ui();
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
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (position.x as f32, position.y as f32);

                // Update hover states
                for widget in &mut self.widgets {
                    widget.update_hover(self.mouse_pos.0, self.mouse_pos.1);
                }

                // Handle slider dragging
                if let Some(slider_idx) = self.dragging_slider {
                    if let Some(slider) = self.widgets.get_mut(slider_idx).and_then(|w| w.as_any_mut().downcast_mut::<Slider>()) {
                        let slider_x = slider.x();
                        let slider_width = slider.width();
                        let relative_x = (self.mouse_pos.0 - slider_x).max(0.0).min(slider_width);
                        let new_value = relative_x / slider_width;
                        slider.set_value(new_value);
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
                // Hide context menu if clicking outside
                if let Some(menu_index) = self.context_menu_index {
                    let mut clicked_menu = false;
                    if let Some(menu) = self.widgets.get(menu_index).and_then(|w| w.as_any().downcast_ref::<ContextMenu>()) {
                        if menu.is_visible() && menu.contains(self.mouse_pos.0, self.mouse_pos.1) {
                            clicked_menu = true;
                        }
                    }
                    
                    if !clicked_menu {
                        if let Some(menu) = self.widgets.get_mut(menu_index).and_then(|w| w.as_any_mut().downcast_mut::<ContextMenu>()) {
                            if menu.is_visible() {
                                menu.hide();
                            }
                        }
                    }
                }

                // Clear previous input focus
                if let Some(prev_index) = self.focused_input {
                    if let Some(input) = self.widgets.get_mut(prev_index).and_then(|w| w.as_any_mut().downcast_mut::<Input>()) {
                        input.set_focused(false);
                    }
                }
                self.focused_input = None;

                // Handle clicks
                for (idx, widget) in self.widgets.iter_mut().enumerate() {
                    if widget.contains(self.mouse_pos.0, self.mouse_pos.1) {
                        widget.on_click();
                        
                        // Check if it's a slider
                        if widget.as_any().is::<Slider>() {
                            self.dragging_slider = Some(idx);
                        }
                        
                        // Check if it's an input field
                        if widget.as_any().is::<Input>() {
                            self.focused_input = Some(idx);
                            if let Some(input) = widget.as_any_mut().downcast_mut::<Input>() {
                                input.set_focused(true);
                            }
                        }
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
                self.dragging_slider = None;
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Right,
                ..
            } => {
                // Show context menu on right click
                if let Some(menu_index) = self.context_menu_index {
                    if let Some(menu) = self.widgets.get_mut(menu_index).and_then(|w| w.as_any_mut().downcast_mut::<ContextMenu>()) {
                        menu.show(self.mouse_pos.0, self.mouse_pos.1);
                    }
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    // Handle keyboard input for focused input field
                    if let Some(input_index) = self.focused_input {
                        if let Some(input) = self.widgets.get_mut(input_index).and_then(|w| w.as_any_mut().downcast_mut::<Input>()) {
                            match event.physical_key {
                                PhysicalKey::Code(KeyCode::Backspace) => {
                                    input.handle_backspace();
                                }
                                PhysicalKey::Code(KeyCode::Escape) => {
                                    input.set_focused(false);
                                    self.focused_input = None;
                                }
                                PhysicalKey::Code(KeyCode::Space) => {
                                    input.handle_char(' ');
                                }
                                _ => {
                                    // Handle text from the event
                                    if let Some(text) = &event.text {
                                        for c in text.chars() {
                                            input.handle_char(c);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Hide context menu on Escape
                    if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
                        if let Some(menu_index) = self.context_menu_index {
                            if let Some(menu) = self.widgets.get_mut(menu_index).and_then(|w| w.as_any_mut().downcast_mut::<ContextMenu>()) {
                                menu.hide();
                            }
                        }
                    }

                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
