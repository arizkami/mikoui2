use mikoui::{
    set_theme, Badge, Button, Card, Checkbox, ContextMenu, Dropdown, FontManager, Icon, IconSize,
    Input, Label, LucideIcons, MenuItem, ProgressBar, ProgressSize, Size, Skeleton, Theme,
    ThemeColors, ThemeMode, Variant, Widget,
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
    theme_mode: ThemeMode,
    theme_colors: ThemeColors,
    theme_button_index: Option<usize>, // Track the theme toggle button
    viewport_size: (f32, f32),
}

impl App {
    fn new() -> Self {
        let theme_mode = ThemeMode::Dark;
        let theme_colors = ThemeColors::dark();
        // Initialize global theme
        set_theme(theme_colors);
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
            theme_mode,
            theme_colors,
            theme_button_index: None,
            viewport_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
        }
    }
    
    fn toggle_theme(&mut self) {
        self.theme_mode = match self.theme_mode {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
        };
        self.theme_colors = match self.theme_mode {
            ThemeMode::Dark => ThemeColors::dark(),
            ThemeMode::Light => ThemeColors::light(),
        };
        // Set global theme for components
        set_theme(self.theme_colors.clone());
        self.build_ui();
    }

    fn build_ui(&mut self) {
        self.widgets.clear();
        self.dropdown_indices.clear();

        // Header
        let padding = 32.0;
        self.widgets.push(Box::new(Label::new(
            padding,
            padding,
            "MikoUI Design System",
            32.0,
            700,
            self.theme_colors.foreground,
        )));
        self.widgets.push(Box::new(Label::new(
            padding,
            padding + 40.0,
            "Beautifully designed components built with skia principles",
            16.0,
            400,
            self.theme_colors.muted_foreground,
        )));
        
        // Theme toggle button with icon
        let theme_icon = match self.theme_mode {
            ThemeMode::Dark => LucideIcons::SUN,
            ThemeMode::Light => LucideIcons::MOON,
        };
        self.theme_button_index = Some(self.widgets.len());
        let icon_x = self.viewport_size.0 - padding - 40.0;
        self.widgets.push(Box::new(Icon::new(
            icon_x,
            padding + 5.0,
            theme_icon,
            IconSize::Large,
            self.theme_colors.foreground,
        )));

        let content_width = (self.viewport_size.0 - padding * 2.0).max(320.0);
        let column_spacing = 24.0;
        let section_spacing = 24.0;
        let two_columns = content_width >= 900.0;
        let column_width = if two_columns {
            (content_width - column_spacing) / 2.0
        } else {
            content_width
        };
        let mut column_y = [padding + 110.0, padding + 110.0];
        let mut next_slot = |preferred: Option<usize>, height: f32, span_full: bool| {
            if span_full || !two_columns {
                let start_y = column_y[0].max(column_y[1]);
                column_y = [start_y + height + section_spacing; 2];
                (padding, start_y, content_width)
            } else {
                let mut column = preferred.unwrap_or_else(|| {
                    if column_y[0] <= column_y[1] {
                        0
                    } else {
                        1
                    }
                });
                if column > 1 {
                    column = if column_y[0] <= column_y[1] { 0 } else { 1 };
                }
                let start_y = column_y[column];
                column_y[column] = start_y + height + section_spacing;
                let x = if column == 0 {
                    padding
                } else {
                    padding + column_width + column_spacing
                };
                (x, start_y, column_width)
            }
        };

        // Button variants section
        let row_gap = 56.0;
        let button_section_width = if two_columns { column_width } else { content_width };
        let button_columns_guess = if button_section_width < 460.0 { 2 } else { 4 };
        let variant_rows = (4 + button_columns_guess - 1) / button_columns_guess;
        let size_rows = (4 + button_columns_guess - 1) / button_columns_guess;
        let button_card_height =
            70.0 + (variant_rows + size_rows) as f32 * row_gap + section_spacing;
        let (card_x, card_y, card_width) = next_slot(Some(0), button_card_height, false);
        self.widgets
            .push(Box::new(Card::new(card_x, card_y, card_width, button_card_height)));
        self.widgets.push(Box::new(Label::new(
            card_x + 20.0,
            card_y + 20.0,
            "Button Variants",
            20.0,
            600,
            self.theme_colors.foreground,
        )));

        let inner_x = card_x + 20.0;
        let button_columns = if card_width < 460.0 { 2 } else { 4 };
        let button_gap = 12.0;
        let button_width = (card_width - 40.0
            - button_gap * (button_columns as f32 - 1.0))
            / button_columns as f32;

        let variant_buttons = [
            ("Default", Variant::Default, Size::Md),
            ("Secondary", Variant::Secondary, Size::Md),
            ("Outline", Variant::Outline, Size::Md),
            ("Ghost", Variant::Ghost, Size::Md),
        ];
        let size_buttons = [
            ("Small", Variant::Default, Size::Sm),
            ("Medium", Variant::Default, Size::Md),
            ("Large", Variant::Default, Size::Lg),
            ("Destructive", Variant::Destructive, Size::Md),
        ];

        let mut current_y = card_y + 70.0;
        for group in [variant_buttons.as_slice(), size_buttons.as_slice()] {
            for chunk in group.chunks(button_columns) {
                let mut btn_x = inner_x;
                for (label, variant, size) in chunk {
                    self.widgets.push(Box::new(
                        Button::new(btn_x, current_y, button_width, label)
                            .variant(*variant)
                            .size(*size),
                    ));
                    btn_x += button_width + button_gap;
                }
                current_y += row_gap;
            }
        }

        // Input section
        let input_card_height = 260.0;
        let (input_x, input_y, input_width) = next_slot(Some(1), input_card_height, false);
        self.widgets
            .push(Box::new(Card::new(input_x, input_y, input_width, input_card_height)));
        self.widgets.push(Box::new(Label::new(
            input_x + 20.0,
            input_y + 20.0,
            "Input & Select",
            20.0,
            600,
            self.theme_colors.foreground,
        )));

        let inner_width = input_width - 40.0;
        let field_x = input_x + 20.0;
        self.widgets.push(Box::new(
            Input::new(field_x, input_y + 70.0, inner_width, "Enter your email...").size(Size::Md),
        ));

        let field_gap = 20.0;
        let half_width = (inner_width - field_gap) / 2.0;
        self.widgets.push(Box::new(
            Input::new(field_x, input_y + 130.0, half_width, "Search...").size(Size::Sm),
        ));

        let dropdown_idx = self.widgets.len();
        self.widgets.push(Box::new(
            Dropdown::new(
                field_x + half_width + field_gap,
                input_y + 130.0,
                half_width,
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
                field_x,
                input_y + 185.0,
                inner_width,
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
        let checkbox_height = 190.0;
        let (checkbox_x, checkbox_y, checkbox_width) =
            next_slot(Some(0), checkbox_height, !two_columns);
        self.widgets.push(Box::new(Card::new(
            checkbox_x,
            checkbox_y,
            checkbox_width,
            checkbox_height,
        )));
        self.widgets.push(Box::new(Label::new(
            checkbox_x + 20.0,
            checkbox_y + 20.0,
            "Checkbox & Selection",
            20.0,
            600,
            self.theme_colors.foreground,
        )));

        self.widgets.push(Box::new(Checkbox::new(
            checkbox_x + 20.0,
            checkbox_y + 70.0,
            "Accept terms and conditions",
        )));
        self.widgets.push(Box::new(Checkbox::new(
            checkbox_x + 20.0,
            checkbox_y + 105.0,
            "Enable notifications",
        )));
        self.widgets.push(Box::new(
            Checkbox::new(checkbox_x + 20.0, checkbox_y + 140.0, "Disabled option").disabled(true),
        ));

        // Badge section
        let badge_height = 180.0;
        let (badge_x, badge_y, badge_width) = next_slot(Some(1), badge_height, !two_columns);
        self.widgets
            .push(Box::new(Card::new(badge_x, badge_y, badge_width, badge_height)));
        self.widgets.push(Box::new(Label::new(
            badge_x + 20.0,
            badge_y + 20.0,
            "Badges",
            20.0,
            600,
            self.theme_colors.foreground,
        )));

        let badge_inner_x = badge_x + 20.0;
        let badge_inner_y = badge_y + 70.0;
        let badge_gap = 16.0;
        let badge_columns = if badge_width > 420.0 { 4 } else { 2 };
        let cell_width =
            (badge_width - 40.0 - badge_gap * (badge_columns as f32 - 1.0)) / badge_columns as f32;

        let badge_variants = [
            ("Default", Variant::Default),
            ("Secondary", Variant::Secondary),
            ("Outline", Variant::Outline),
            ("Destructive", Variant::Destructive),
        ];

        for (index, (label, variant)) in badge_variants.iter().enumerate() {
            let row = index / badge_columns;
            let column = index % badge_columns;
            let x = badge_inner_x + column as f32 * (cell_width + badge_gap);
            let y = badge_inner_y + row as f32 * 40.0;
            self.widgets
                .push(Box::new(Badge::new(x, y, label).variant(*variant)));
        }

        // Skeleton section
        let skeleton_height = 220.0;
        let (skeleton_x, skeleton_y, skeleton_width) =
            next_slot(Some(0), skeleton_height, !two_columns);
        self.widgets.push(Box::new(Card::new(
            skeleton_x,
            skeleton_y,
            skeleton_width,
            skeleton_height,
        )));
        self.widgets.push(Box::new(Label::new(
            skeleton_x + 20.0,
            skeleton_y + 20.0,
            "Skeletons & Pulse",
            20.0,
            600,
            self.theme_colors.foreground,
        )));

        self.widgets.push(Box::new(Skeleton::new_circle(
            skeleton_x + 20.0,
            skeleton_y + 70.0,
            64.0,
        )));
        let sk_inner_x = skeleton_x + 100.0;
        let sk_inner_width = skeleton_width - 120.0;
        self.widgets.push(Box::new(
            Skeleton::new(sk_inner_x, skeleton_y + 70.0, sk_inner_width, 16.0)
                .border_radius(Theme::RADIUS_SM),
        ));
        self.widgets.push(Box::new(
            Skeleton::new(sk_inner_x, skeleton_y + 96.0, sk_inner_width * 0.85, 16.0)
                .border_radius(Theme::RADIUS_SM)
                .pulse_speed(2.0),
        ));
        self.widgets.push(Box::new(
            Skeleton::new(sk_inner_x, skeleton_y + 122.0, sk_inner_width * 0.7, 16.0)
                .border_radius(Theme::RADIUS_SM),
        ));
        self.widgets.push(Box::new(
            Skeleton::new(skeleton_x + 20.0, skeleton_y + 160.0, skeleton_width - 40.0, 28.0)
                .border_radius(Theme::RADIUS_MD),
        ));

        // Progress section
        let progress_height = 220.0;
        let (progress_x, progress_y, progress_width) =
            next_slot(Some(1), progress_height, !two_columns);
        self.widgets.push(Box::new(Card::new(
            progress_x,
            progress_y,
            progress_width,
            progress_height,
        )));
        self.widgets.push(Box::new(Label::new(
            progress_x + 20.0,
            progress_y + 20.0,
            "Progress Indicators",
            20.0,
            600,
            self.theme_colors.foreground,
        )));

        let progress_inner_x = progress_x + 20.0;
        let progress_inner_width = progress_width - 40.0;
        let mut progress_y_cursor = progress_y + 70.0;

        let mut progress_xs =
            ProgressBar::new(progress_inner_x, progress_y_cursor, progress_inner_width)
                .size(ProgressSize::Xs);
        progress_xs.set_progress(0.35);
        self.widgets.push(Box::new(progress_xs));

        progress_y_cursor += 18.0;
        let mut progress_sm =
            ProgressBar::new(progress_inner_x, progress_y_cursor, progress_inner_width)
                .size(ProgressSize::Sm);
        progress_sm.set_progress(0.55);
        self.widgets.push(Box::new(progress_sm));

        progress_y_cursor += 24.0;
        let mut progress_md =
            ProgressBar::new(progress_inner_x, progress_y_cursor, progress_inner_width)
                .size(ProgressSize::Md);
        progress_md.set_progress(0.68);
        self.widgets.push(Box::new(progress_md));

        progress_y_cursor += 32.0;
        let mut progress_lg =
            ProgressBar::new(progress_inner_x, progress_y_cursor, progress_inner_width)
                .size(ProgressSize::Lg)
                .with_label("Uploading... 78%");
        progress_lg.set_progress(0.78);
        self.widgets.push(Box::new(progress_lg));

        progress_y_cursor += 40.0;
        let mut progress_xl =
            ProgressBar::new(progress_inner_x, progress_y_cursor, progress_inner_width)
                .size(ProgressSize::Xl)
                .with_label("Complete 100%");
        progress_xl.set_progress(1.0);
        self.widgets.push(Box::new(progress_xl));

        // Icons and context-menu demo row (full width)
        let icons_row_height = if content_width < 600.0 { 120.0 } else { 80.0 };
        let (row_x, row_y, row_width) = next_slot(None, icons_row_height, true);
        let mut icon_x = row_x + 20.0;
        let icon_y = row_y + 20.0;
        for (icon, color) in [
            (LucideIcons::SEARCH, self.theme_colors.foreground),
            (LucideIcons::USER, self.theme_colors.foreground),
            (LucideIcons::SETTINGS, self.theme_colors.foreground),
            (LucideIcons::HEART, self.theme_colors.destructive),
        ] {
            self.widgets
                .push(Box::new(Icon::new(icon_x, icon_y, icon, IconSize::Medium, color)));
            icon_x += 50.0;
        }

        let button_width = 180.0;
        let stack_button = row_width < 520.0;
        let button_x = if stack_button {
            row_x + 20.0
        } else {
            row_x + row_width - button_width - 20.0
        };
        self.widgets.push(Box::new(
            Button::new(
                button_x,
                if stack_button { row_y + 60.0 } else { row_y + 18.0 },
                button_width,
                "Right Click Me",
            )
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

            // Dynamic theme background color
            canvas.clear(self.theme_colors.background);

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
                .with_title("Miko UI - MikoUI Design System")
                .with_inner_size(winit::dpi::LogicalSize::new(
                    WINDOW_WIDTH as i32,
                    WINDOW_HEIGHT as i32,
                ));

            let window = Rc::new(event_loop.create_window(window_attributes).unwrap());
            let inner = window.inner_size();
            self.viewport_size = (inner.width as f32, inner.height as f32);
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
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    self.viewport_size = (size.width as f32, size.height as f32);
                    self.build_ui();
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
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
                        // Check if clicking theme toggle button
                        if Some(idx) == self.theme_button_index {
                            self.toggle_theme();
                            if let Some(window) = &self.window {
                                window.request_redraw();
                            }
                            return;
                        }
                        
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
