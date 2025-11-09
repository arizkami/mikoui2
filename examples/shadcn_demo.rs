use mikoui::{
    Badge, Button, Card, Checkbox, ContextMenu, Dropdown, FontManager, Icon, IconSize, Input,
    Label, LucideIcons, MenuItem, ProgressBar, Size, Theme, Variant, Widget,
};
use skia_safe::Color;
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;

struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    widgets: Vec<Box<dyn Widget>>,
    mouse_pos: (f32, f32),
    font_manager: FontManager,
    start_time: Instant,
    focused_input: Option<usize>,
    context_menu: Option<ContextMenu>,
    dropdown_indices: Vec<usize>, // Track which widgets are dropdowns
    open_dropdown: Option<usize>, // Track which dropdown is currently open (z-index 9999)
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
            focused_input: None,
            context_menu: None,
            dropdown_indices: Vec::new(),
            open_dropdown: None,
        }
    }

    fn build_ui(&mut self) {
        self.widgets.clear();
        self.dropdown_indices.clear();

        // Header
        self.widgets.push(Box::new(Label::new(
            40.0,
            40.0,
            "skiacn/ui Design System",
            32.0,
            700,
            Theme::FOREGROUND,
        )));
        self.widgets.push(Box::new(Label::new(
            40.0,
            80.0,
            "Beautifully designed components built with Radix UI principles",
            16.0,
            400,
            Theme::MUTED_FOREGROUND,
        )));

        // Button variants section
        self.widgets.push(Box::new(Card::new(40.0, 120.0, 560.0, 200.0)));
        self.widgets.push(Box::new(Label::new(
            60.0,
            140.0,
            "Button Variants",
            20.0,
            600,
            Theme::FOREGROUND,
        )));

        let mut btn_x = 60.0;
        let btn_y = 180.0;
        
        self.widgets.push(Box::new(
            Button::new(btn_x, btn_y, 120.0, "Default")
                .variant(Variant::Default)
                .size(Size::Md),
        ));
        btn_x += 130.0;
        
        self.widgets.push(Box::new(
            Button::new(btn_x, btn_y, 120.0, "Secondary")
                .variant(Variant::Secondary)
                .size(Size::Md),
        ));
        btn_x += 130.0;
        
        self.widgets.push(Box::new(
            Button::new(btn_x, btn_y, 120.0, "Outline")
                .variant(Variant::Outline)
                .size(Size::Md),
        ));
        btn_x += 130.0;
        
        self.widgets.push(Box::new(
            Button::new(btn_x, btn_y, 120.0, "Ghost")
                .variant(Variant::Ghost)
                .size(Size::Md),
        ));

        // Button sizes
        let mut btn_x = 60.0;
        let btn_y = 240.0;
        
        self.widgets.push(Box::new(
            Button::new(btn_x, btn_y, 100.0, "Small")
                .variant(Variant::Default)
                .size(Size::Sm),
        ));
        btn_x += 110.0;
        
        self.widgets.push(Box::new(
            Button::new(btn_x, btn_y, 100.0, "Medium")
                .variant(Variant::Default)
                .size(Size::Md),
        ));
        btn_x += 110.0;
        
        self.widgets.push(Box::new(
            Button::new(btn_x, btn_y, 100.0, "Large")
                .variant(Variant::Default)
                .size(Size::Lg),
        ));
        btn_x += 110.0;
        
        self.widgets.push(Box::new(
            Button::new(btn_x, btn_y, 120.0, "Destructive")
                .variant(Variant::Destructive)
                .size(Size::Md),
        ));

        // Input section
        self.widgets.push(Box::new(Card::new(620.0, 120.0, 540.0, 240.0)));
        self.widgets.push(Box::new(Label::new(
            640.0,
            140.0,
            "Input & Select",
            20.0,
            600,
            Theme::FOREGROUND,
        )));

        self.widgets.push(Box::new(
            Input::new(640.0, 180.0, 480.0, "Enter your email...")
                .size(Size::Md),
        ));
        
        self.widgets.push(Box::new(
            Input::new(640.0, 240.0, 230.0, "Search...")
                .size(Size::Sm),
        ));
        
        let dropdown_idx = self.widgets.len();
        self.widgets.push(Box::new(
            Dropdown::new(
                890.0,
                240.0,
                230.0,
                "Framework",
                vec![
                    "Next.js".to_string(),
                    "React".to_string(),
                    "Vue".to_string(),
                    "Svelte".to_string(),
                    "Astro".to_string(),
                ],
            )
            .size(Size::Sm),
        ));
        self.dropdown_indices.push(dropdown_idx);
        
        let dropdown_idx = self.widgets.len();
        self.widgets.push(Box::new(
            Dropdown::new(
                640.0,
                290.0,
                480.0,
                "Select a theme",
                vec![
                    "Default".to_string(),
                    "Dark".to_string(),
                    "Light".to_string(),
                    "System".to_string(),
                ],
            )
            .size(Size::Md),
        ));
        self.dropdown_indices.push(dropdown_idx);

        // Checkbox section
        self.widgets.push(Box::new(Card::new(40.0, 380.0, 560.0, 180.0)));
        self.widgets.push(Box::new(Label::new(
            60.0,
            400.0,
            "Checkbox & Selection",
            20.0,
            600,
            Theme::FOREGROUND,
        )));

        self.widgets.push(Box::new(Checkbox::new(
            60.0,
            440.0,
            "Accept terms and conditions",
        )));
        self.widgets.push(Box::new(Checkbox::new(
            60.0,
            480.0,
            "Enable notifications",
        )));
        self.widgets.push(Box::new(
            Checkbox::new(60.0, 520.0, "Disabled option").disabled(true),
        ));

        // Badge section
        self.widgets.push(Box::new(Card::new(620.0, 380.0, 540.0, 180.0)));
        self.widgets.push(Box::new(Label::new(
            640.0,
            400.0,
            "Badges",
            20.0,
            600,
            Theme::FOREGROUND,
        )));

        let mut badge_x = 640.0;
        let badge_y = 440.0;
        
        self.widgets.push(Box::new(
            Badge::new(badge_x, badge_y, "Default").variant(Variant::Default),
        ));
        badge_x += 90.0;
        
        self.widgets.push(Box::new(
            Badge::new(badge_x, badge_y, "Secondary").variant(Variant::Secondary),
        ));
        badge_x += 110.0;
        
        self.widgets.push(Box::new(
            Badge::new(badge_x, badge_y, "Outline").variant(Variant::Outline),
        ));
        badge_x += 90.0;
        
        self.widgets.push(Box::new(
            Badge::new(badge_x, badge_y, "Destructive").variant(Variant::Destructive),
        ));

        // Progress section
        self.widgets.push(Box::new(Card::new(40.0, 580.0, 1120.0, 220.0)));
        self.widgets.push(Box::new(Label::new(
            60.0,
            600.0,
            "Progress Indicators",
            20.0,
            600,
            Theme::FOREGROUND,
        )));

        let mut progress1 = ProgressBar::new(60.0, 640.0, 1040.0, 20.0)
            .with_label("Uploading files... 45%");
        progress1.set_progress(0.45);
        self.widgets.push(Box::new(progress1));

        let mut progress2 = ProgressBar::new(60.0, 680.0, 1040.0, 20.0)
            .with_label("Processing... 78%");
        progress2.set_progress(0.78);
        self.widgets.push(Box::new(progress2));

        let mut progress3 = ProgressBar::new(60.0, 720.0, 1040.0, 20.0)
            .with_label("Complete 100%");
        progress3.set_progress(1.0);
        self.widgets.push(Box::new(progress3));

        // Icons
        self.widgets.push(Box::new(Icon::new(
            60.0,
            760.0,
            LucideIcons::SEARCH,
            IconSize::Medium,
            Theme::FOREGROUND,
        )));
        self.widgets.push(Box::new(Icon::new(
            110.0,
            760.0,
            LucideIcons::USER,
            IconSize::Medium,
            Theme::FOREGROUND,
        )));
        self.widgets.push(Box::new(Icon::new(
            160.0,
            760.0,
            LucideIcons::SETTINGS,
            IconSize::Medium,
            Theme::FOREGROUND,
        )));
        self.widgets.push(Box::new(Icon::new(
            210.0,
            760.0,
            LucideIcons::HEART,
            IconSize::Medium,
            Theme::ERROR,
        )));

        // Context menu demo button
        self.widgets.push(Box::new(
            Button::new(1000.0, 760.0, 160.0, "Right Click Me")
                .variant(Variant::Outline)
                .size(Size::Sm),
        ));

        self.focused_input = None;
        self.context_menu = None;
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

            // shadcn/ui background color
            canvas.clear(Theme::BACKGROUND);

            let elapsed = self.start_time.elapsed().as_secs_f32();
            for widget in &mut self.widgets {
                widget.update_animation(elapsed);
            }

            // Draw all widgets except the open dropdown
            for (idx, widget) in self.widgets.iter_mut().enumerate() {
                if Some(idx) != self.open_dropdown {
                    widget.draw(canvas, &mut self.font_manager);
                }
            }

            // Draw the open dropdown on top (z-index: 9999)
            if let Some(open_idx) = self.open_dropdown {
                if let Some(widget) = self.widgets.get_mut(open_idx) {
                    widget.draw(canvas, &mut self.font_manager);
                }
            }

            // Draw context menu on top of everything (z-index: 10000)
            if let Some(ref mut menu) = self.context_menu {
                menu.update_animation(elapsed);
                menu.draw(canvas, &mut self.font_manager);
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
                .with_title("Miko UI - skiacn/ui Design System")
                .with_inner_size(winit::dpi::LogicalSize::new(
                    WINDOW_WIDTH as i32,
                    WINDOW_HEIGHT as i32,
                ));

            let window = Rc::new(event_loop.create_window(window_attributes).unwrap());
            let context = Context::new(window.clone()).unwrap();
            let surface = Surface::new(&context, window.clone()).unwrap();

            self.window = Some(window.clone());
            self.surface = Some(surface);

            self.build_ui();
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
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (position.x as f32, position.y as f32);

                for widget in &mut self.widgets {
                    widget.update_hover(self.mouse_pos.0, self.mouse_pos.1);
                }

                if let Some(ref mut menu) = self.context_menu {
                    menu.update_hover(self.mouse_pos.0, self.mouse_pos.1);
                }

                // Update open dropdown tracking based on current state
                self.open_dropdown = None;
                for &idx in &self.dropdown_indices {
                    if let Some(dropdown) = self.widgets.get(idx).and_then(|w| {
                        w.as_any().downcast_ref::<Dropdown>()
                    }) {
                        if dropdown.is_open() {
                            self.open_dropdown = Some(idx);
                            break;
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
                // Check if clicking on context menu
                if let Some(ref mut menu) = self.context_menu {
                    if menu.contains(self.mouse_pos.0, self.mouse_pos.1) {
                        menu.on_click();
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                        return;
                    } else {
                        // Hide menu if clicking outside
                        menu.hide();
                    }
                }

                if let Some(prev_index) = self.focused_input {
                    if let Some(input) = self
                        .widgets
                        .get_mut(prev_index)
                        .and_then(|w| w.as_any_mut().downcast_mut::<Input>())
                    {
                        input.set_focused(false);
                    }
                }
                self.focused_input = None;

                // Check if clicking on a dropdown
                let mut clicked_dropdown = None;
                for &idx in &self.dropdown_indices {
                    if let Some(widget) = self.widgets.get(idx) {
                        if widget.contains(self.mouse_pos.0, self.mouse_pos.1) {
                            clicked_dropdown = Some(idx);
                            break;
                        }
                    }
                }

                // Close all dropdowns except the one being clicked
                for &idx in &self.dropdown_indices {
                    if Some(idx) != clicked_dropdown {
                        if let Some(dropdown) = self.widgets.get_mut(idx).and_then(|w| {
                            w.as_any_mut().downcast_mut::<Dropdown>()
                        }) {
                            if dropdown.is_open() {
                                dropdown.close();
                            }
                        }
                    }
                }

                // Update open dropdown tracking
                self.open_dropdown = None;

                for (idx, widget) in self.widgets.iter_mut().enumerate() {
                    if widget.contains(self.mouse_pos.0, self.mouse_pos.1) {
                        widget.on_click();

                        // Track if this dropdown is now open
                        if self.dropdown_indices.contains(&idx) {
                            if let Some(dropdown) = widget.as_any().downcast_ref::<Dropdown>() {
                                if dropdown.is_open() {
                                    self.open_dropdown = Some(idx);
                                }
                            }
                        }

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
                state: ElementState::Pressed,
                button: MouseButton::Right,
                ..
            } => {
                // Show context menu on right click
                let items = vec![
                    MenuItem::new("Copy", 1).with_shortcut("Ctrl+C"),
                    MenuItem::new("Cut", 2).with_shortcut("Ctrl+X"),
                    MenuItem::new("Paste", 3).with_shortcut("Ctrl+V"),
                    MenuItem::separator(),
                    MenuItem::new("Select All", 4).with_shortcut("Ctrl+A"),
                    MenuItem::separator(),
                    MenuItem::new("Delete", 5).with_shortcut("Del"),
                    MenuItem::new("Disabled Item", 6).disabled(),
                ];

                let mut menu = ContextMenu::new(self.mouse_pos.0, self.mouse_pos.1, items);
                menu.show(self.mouse_pos.0, self.mouse_pos.1);
                self.context_menu = Some(menu);

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if let Some(input_index) = self.focused_input {
                        if let Some(input) = self
                            .widgets
                            .get_mut(input_index)
                            .and_then(|w| w.as_any_mut().downcast_mut::<Input>())
                        {
                            // Check for Ctrl/Cmd modifiers
                            let ctrl_pressed = event.state.is_pressed() && 
                                (cfg!(target_os = "macos") && event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Control) ||
                                 cfg!(not(target_os = "macos")) && event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Control));
                            
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
                                PhysicalKey::Code(KeyCode::KeyA) if event.logical_key == winit::keyboard::Key::Character("a".into()) => {
                                    // Ctrl+A / Cmd+A - Select All
                                    input.select_all();
                                }
                                PhysicalKey::Code(KeyCode::KeyC) if event.logical_key == winit::keyboard::Key::Character("c".into()) => {
                                    // Ctrl+C / Cmd+C - Copy
                                    input.copy();
                                }
                                PhysicalKey::Code(KeyCode::KeyX) if event.logical_key == winit::keyboard::Key::Character("x".into()) => {
                                    // Ctrl+X / Cmd+X - Cut
                                    input.cut();
                                }
                                PhysicalKey::Code(KeyCode::KeyV) if event.logical_key == winit::keyboard::Key::Character("v".into()) => {
                                    // Ctrl+V / Cmd+V - Paste
                                    input.paste();
                                }
                                _ => {
                                    if let Some(text) = &event.text {
                                        for c in text.chars() {
                                            // Skip control characters that are handled above
                                            if c != '\u{1}' && c != '\u{3}' && c != '\u{16}' && c != '\u{18}' {
                                                input.handle_char(c);
                                            }
                                        }
                                    }
                                }
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
