use skia_safe::{Font, Typeface};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

mod components;
mod theme;

use components::{Button, ButtonStyle, Checkbox, Input, Label, Panel, ProgressBar, Slider, Widget};
use theme::ZedTheme;

struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    widgets: Vec<Box<dyn Widget>>,
    mouse_pos: (f32, f32),
    typeface: Option<Typeface>,
    start_time: Instant,
    dragging_slider: Option<usize>,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            surface: None,
            widgets: Vec::new(),
            mouse_pos: (0.0, 0.0),
            typeface: None,
            start_time: Instant::now(),
            dragging_slider: None,
        }
    }

    fn load_variable_font(&mut self) {
        // Embed Inter Variable font in binary
        const FONT_DATA: &[u8] = include_bytes!("../fonts/InterVariable.ttf");
        
        // Create Data from embedded bytes
        let data = skia_safe::Data::new_copy(FONT_DATA);
        
        // Try to create typeface from data
        if let Some(typeface) = skia_safe::FontMgr::new().new_from_data(&data, None) {
            println!("Loaded embedded Inter Variable font ({} bytes)", FONT_DATA.len());
            
            // Check if it's a variable font
            let variation_count = typeface.variation_design_position().map(|v| v.len()).unwrap_or(0);
            if variation_count > 0 {
                println!("Variable font detected with {} axes", variation_count);
            }
            
            self.typeface = Some(typeface);
            return;
        }
        
        println!("Warning: Failed to load embedded font, using system default");
        // Use system default font
        if let Some(typeface) = skia_safe::FontMgr::new().legacy_make_typeface(None, skia_safe::FontStyle::default()) {
            self.typeface = Some(typeface);
        }
    }

    fn create_variable_font(typeface: &Typeface, size: f32, weight: i32) -> Font {
        // Set variation design coordinates for wght and opsz axes
        // Inter Variable supports:
        // - wght (weight): 100-900
        // - opsz (optical size): 14-32
        let weight_value = weight.clamp(100, 900) as f32;
        let opsz_value = size.clamp(14.0, 32.0);
        
        // Create FourByteTag for axes using from_chars
        let wght_tag = skia_safe::FourByteTag::from_chars('w', 'g', 'h', 't');
        let opsz_tag = skia_safe::FourByteTag::from_chars('o', 'p', 's', 'z');
        
        // Create variation position coordinates
        use skia_safe::font_arguments::variation_position::Coordinate;
        let coordinates = [
            Coordinate {
                axis: wght_tag,
                value: weight_value,
            },
            Coordinate {
                axis: opsz_tag,
                value: opsz_value,
            },
        ];
        
        // Create font arguments with Variable Font axes
        let font_args = skia_safe::FontArguments::new().set_variation_design_position(
            skia_safe::font_arguments::VariationPosition {
                coordinates: &coordinates,
            },
        );
        
        // Create typeface with variation
        let varied_typeface = typeface.clone_with_arguments(&font_args).unwrap_or_else(|| typeface.clone());
        
        // Create font with the varied typeface
        let mut font = Font::from_typeface(&varied_typeface, size);
        
        // Enhanced sub-pixel rendering for Variable Font
        font.set_edging(skia_safe::font::Edging::SubpixelAntiAlias);
        font.set_subpixel(true);
        font.set_linear_metrics(true);
        font.set_hinting(skia_safe::FontHinting::None);
        font.set_force_auto_hinting(false);
        font.set_embedded_bitmaps(false);
        font.set_baseline_snap(false);
        
        // Fine-tune spacing based on optical size
        let spacing_adjustment = if size <= 12.0 {
            1.01 // Slightly more spacing for small text
        } else if size >= 20.0 {
            1.0 // Normal spacing for large text
        } else {
            1.0
        };
        
        font.set_scale_x(spacing_adjustment);
        font
    }

    fn setup_ui(&mut self) {
        // Panel background
        self.widgets.push(Box::new(
            Panel::new(30.0, 30.0, 740.0, 540.0).with_title("Component Showcase"),
        ));

        // Title
        self.widgets.push(Box::new(Label::new(
            50.0,
            80.0,
            "Zed-Style UI Components",
            20.0,
            700,
            ZedTheme::TEXT,
        )));

        // Subtitle
        self.widgets.push(Box::new(Label::new(
            50.0,
            105.0,
            "Built with Skia-Safe in Rust + Variable Font Support",
            14.0,
            500,
            ZedTheme::TEXT_DIM,
        )));

        // Buttons section
        self.widgets.push(Box::new(Label::new(
            50.0,
            145.0,
            "Buttons",
            12.0,
            600,
            ZedTheme::TEXT_DIM,
        )));

        self.widgets.push(Box::new(Button::new(
            50.0,
            160.0,
            140.0,
            36.0,
            "Primary Action",
            ButtonStyle::Primary,
        )));

        self.widgets.push(Box::new(Button::new(
            200.0,
            160.0,
            120.0,
            36.0,
            "Secondary",
            ButtonStyle::Secondary,
        )));

        // Input section
        self.widgets.push(Box::new(Label::new(
            50.0,
            225.0,
            "Text Input",
            12.0,
            600,
            ZedTheme::TEXT_DIM,
        )));

        self.widgets.push(Box::new(Input::new(
            50.0,
            240.0,
            350.0,
            38.0,
            "Type something...",
        )));

        // Checkbox section
        self.widgets.push(Box::new(Label::new(
            50.0,
            305.0,
            "Checkbox",
            12.0,
            600,
            ZedTheme::TEXT_DIM,
        )));

        self.widgets.push(Box::new(Checkbox::new(
            50.0,
            320.0,
            "Enable notifications",
        )));

        // Slider section
        self.widgets.push(Box::new(Label::new(
            50.0,
            365.0,
            "Slider",
            12.0,
            600,
            ZedTheme::TEXT_DIM,
        )));

        self.widgets.push(Box::new(Slider::new(
            50.0,
            380.0,
            300.0,
            "Volume",
            0.7,
        )));

        // Progress bar section
        self.widgets.push(Box::new(Label::new(
            50.0,
            445.0,
            "Progress Bar",
            12.0,
            600,
            ZedTheme::TEXT_DIM,
        )));

        let mut progress = ProgressBar::new(50.0, 460.0, 300.0, 20.0).with_label("Loading...");
        progress.set_progress(0.65);
        self.widgets.push(Box::new(progress));

        // Info label
        self.widgets.push(Box::new(Label::new(
            50.0,
            520.0,
            "Variable Font: wght (100-900) + opsz (14-32) axes",
            11.0,
            400,
            ZedTheme::TEXT_MUTED,
        )));
    }

}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Rc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_title("Skia-Safe VF GUI")
                            .with_inner_size(winit::dpi::LogicalSize::new(800, 600)),
                    )
                    .unwrap(),
            );

            let context = Context::new(window.clone()).unwrap();
            let surface = Surface::new(&context, window.clone()).unwrap();

            self.window = Some(window);
            self.surface = Some(surface);

            self.load_variable_font();
            self.setup_ui();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (position.x as f32, position.y as f32);

                // Update hover states
                for widget in &mut self.widgets {
                    widget.update_hover(self.mouse_pos.0, self.mouse_pos.1);
                }

                // Handle slider dragging
                if let Some(slider_idx) = self.dragging_slider {
                    if let Some(slider) = self.widgets.get_mut(slider_idx) {
                        // Update slider value based on mouse position
                        if let Some(slider) = slider.as_any_mut().downcast_mut::<Slider>() {
                            let relative_x = (self.mouse_pos.0 - slider.x()).max(0.0).min(slider.width());
                            let new_value = relative_x / slider.width();
                            slider.set_value(new_value);
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
                // Handle clicks
                for (idx, widget) in self.widgets.iter_mut().enumerate() {
                    if widget.contains(self.mouse_pos.0, self.mouse_pos.1) {
                        widget.on_click();
                        
                        // Check if it's a slider
                        if widget.as_any().downcast_ref::<Slider>().is_some() {
                            self.dragging_slider = Some(idx);
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
                // Stop dragging
                self.dragging_slider = None;

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    let size = window.inner_size();
                    let width = size.width;
                    let height = size.height;

                    if let Some(surface) = &mut self.surface {
                        surface
                            .resize(
                                NonZeroU32::new(width).unwrap(),
                                NonZeroU32::new(height).unwrap(),
                            )
                            .unwrap();

                        let mut buffer = surface.buffer_mut().unwrap();

                        // Convert u32 buffer to u8 bytes
                        let buffer_slice = unsafe {
                            std::slice::from_raw_parts_mut(
                                buffer.as_mut_ptr() as *mut u8,
                                buffer.len() * 4,
                            )
                        };

                        // Create Skia surface
                        let mut skia_surface = skia_safe::surfaces::wrap_pixels(
                            &skia_safe::ImageInfo::new(
                                (width as i32, height as i32),
                                skia_safe::ColorType::BGRA8888,
                                skia_safe::AlphaType::Premul,
                                None,
                            ),
                            buffer_slice,
                            None,
                            None,
                        )
                        .unwrap();

                        // Draw to canvas
                        let canvas = skia_surface.canvas();
                        canvas.clear(ZedTheme::BG);

                        // Update animations
                        let elapsed = self.start_time.elapsed().as_secs_f32();
                        for widget in &mut self.widgets {
                            widget.update_animation(elapsed);
                        }

                        // Draw all widgets
                        let typeface = self.typeface.clone();
                        for widget in &self.widgets {
                            widget.draw(canvas, &|size, weight| {
                                Self::create_variable_font(typeface.as_ref().unwrap(), size, weight)
                            });
                        }

                        buffer.present().unwrap();
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
