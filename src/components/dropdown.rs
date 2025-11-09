use skia_safe::{Canvas, Color, Paint, Rect};
use crate::components::Widget;
use crate::core::FontManager;
use crate::theme::{with_alpha, Size, Theme};

pub struct Dropdown {
    x: f32,
    y: f32,
    width: f32,
    label: String,
    options: Vec<String>,
    selected_index: usize,
    open: bool,
    hover: bool,
    hover_option: Option<usize>,
    hover_progress: f32,
    option_hover_progress: Vec<f32>,
    size: Size,
}

impl Dropdown {
    pub fn new(x: f32, y: f32, width: f32, label: impl Into<String>, options: Vec<String>) -> Self {
        let option_hover_progress = vec![0.0; options.len()];
        Self {
            x,
            y,
            width,
            label: label.into(),
            options,
            selected_index: 0,
            open: false,
            hover: false,
            hover_option: None,
            hover_progress: 0.0,
            option_hover_progress,
            size: Size::Md,
        }
    }
    
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn selected_value(&self) -> &str {
        &self.options[self.selected_index]
    }

    pub fn set_selected(&mut self, index: usize) {
        if index < self.options.len() {
            self.selected_index = index;
        }
    }
    
    pub fn is_open(&self) -> bool {
        self.open
    }
    
    pub fn close(&mut self) {
        self.open = false;
        self.hover_option = None;
    }

    fn button_height(&self) -> f32 {
        self.size.height()
    }

    fn option_height(&self) -> f32 {
        36.0
    }
    
    fn padding_top(&self) -> f32 {
        Theme::SPACE_1
    }
    
    fn padding_bottom(&self) -> f32 {
        Theme::SPACE_1
    }

    fn button_rect(&self) -> Rect {
        Rect::from_xywh(self.x, self.y, self.width, self.button_height())
    }

    fn dropdown_rect(&self) -> Rect {
        let items_height = self.options.len() as f32 * self.option_height();
        let total_height = items_height + self.padding_top() + self.padding_bottom();
        Rect::from_xywh(
            self.x,
            self.y + self.button_height() + Theme::SPACE_1,
            self.width,
            total_height,
        )
    }

    fn option_rect(&self, index: usize) -> Rect {
        let dropdown = self.dropdown_rect();
        Rect::from_xywh(
            dropdown.left,
            dropdown.top + self.padding_top() + index as f32 * self.option_height(),
            dropdown.width(),
            self.option_height(),
        )
    }
}

impl Widget for Dropdown {
    fn draw(&self, canvas: &Canvas, font_manager: &mut FontManager) {
        let button_rect = self.button_rect();

        // Draw button background
        let mut bg_paint = Paint::default();
        bg_paint.set_color(Theme::BACKGROUND);
        bg_paint.set_anti_alias(true);
        canvas.draw_round_rect(button_rect, Theme::RADIUS_MD, Theme::RADIUS_MD, &bg_paint);

        // Draw border with focus ring
        let border_color = if self.open {
            Theme::RING
        } else {
            Theme::BORDER
        };

        let mut border_paint = Paint::default();
        border_paint.set_color(border_color);
        border_paint.set_style(skia_safe::PaintStyle::Stroke);
        border_paint.set_stroke_width(1.0);
        border_paint.set_anti_alias(true);
        canvas.draw_round_rect(
            Rect::from_xywh(
                button_rect.left + 0.5,
                button_rect.top + 0.5,
                button_rect.width() - 1.0,
                button_rect.height() - 1.0,
            ),
            Theme::RADIUS_MD,
            Theme::RADIUS_MD,
            &border_paint,
        );
        
        // Focus ring when open
        if self.open {
            let mut ring_paint = Paint::default();
            ring_paint.set_color(with_alpha(Theme::RING, 100));
            ring_paint.set_style(skia_safe::PaintStyle::Stroke);
            ring_paint.set_stroke_width(3.0);
            ring_paint.set_anti_alias(true);
            canvas.draw_round_rect(
                Rect::from_xywh(
                    button_rect.left - 1.5,
                    button_rect.top - 1.5,
                    button_rect.width() + 3.0,
                    button_rect.height() + 3.0,
                ),
                Theme::RADIUS_MD + 1.5,
                Theme::RADIUS_MD + 1.5,
                &ring_paint,
            );
        }

        // Draw selected value
        let padding_x = self.size.padding_x();
        let font_size = self.size.font_size();
        let text_x = button_rect.left + padding_x;
        let text_y = button_rect.top + button_rect.height() / 2.0 + (font_size * 0.3);
        
        let font = font_manager.create_font(self.selected_value(), font_size, 400);
        let mut text_paint = Paint::default();
        text_paint.set_color(Theme::FOREGROUND);
        text_paint.set_anti_alias(true);
        canvas.draw_str(self.selected_value(), (text_x, text_y), &font, &text_paint);

        // Draw arrow icon
        let arrow_x = button_rect.right - 24.0;
        let arrow_y = button_rect.top + button_rect.height() / 2.0;
        let arrow_size = 8.0;
        
        let mut arrow_paint = Paint::default();
        arrow_paint.set_color(Theme::MUTED_FOREGROUND);
        arrow_paint.set_style(skia_safe::PaintStyle::Stroke);
        arrow_paint.set_stroke_width(2.0);
        arrow_paint.set_anti_alias(true);

        if self.open {
            // Up arrow
            canvas.draw_line((arrow_x - arrow_size / 2.0, arrow_y + 2.0), (arrow_x, arrow_y - 2.0), &arrow_paint);
            canvas.draw_line((arrow_x, arrow_y - 2.0), (arrow_x + arrow_size / 2.0, arrow_y + 2.0), &arrow_paint);
        } else {
            // Down arrow
            canvas.draw_line((arrow_x - arrow_size / 2.0, arrow_y - 2.0), (arrow_x, arrow_y + 2.0), &arrow_paint);
            canvas.draw_line((arrow_x, arrow_y + 2.0), (arrow_x + arrow_size / 2.0, arrow_y - 2.0), &arrow_paint);
        }

        // Draw dropdown menu if open
        if self.open {
            let dropdown_rect = self.dropdown_rect();

            // Draw shadow (shadcn style)
            let shadow_rect = Rect::from_xywh(
                dropdown_rect.left,
                dropdown_rect.top + 4.0,
                dropdown_rect.width(),
                dropdown_rect.height(),
            );
            let mut shadow_paint = Paint::default();
            shadow_paint.set_color(with_alpha(Theme::BACKGROUND, 30));
            shadow_paint.set_anti_alias(true);
            canvas.draw_round_rect(shadow_rect, Theme::RADIUS_MD, Theme::RADIUS_MD, &shadow_paint);

            // Draw background (popover style)
            let mut dropdown_bg = Paint::default();
            dropdown_bg.set_color(Theme::POPOVER);
            dropdown_bg.set_anti_alias(true);
            canvas.draw_round_rect(dropdown_rect, Theme::RADIUS_MD, Theme::RADIUS_MD, &dropdown_bg);

            // Draw border
            let mut dropdown_border = Paint::default();
            dropdown_border.set_color(Theme::BORDER);
            dropdown_border.set_style(skia_safe::PaintStyle::Stroke);
            dropdown_border.set_stroke_width(1.0);
            dropdown_border.set_anti_alias(true);
            canvas.draw_round_rect(
                Rect::from_xywh(
                    dropdown_rect.left + 0.5,
                    dropdown_rect.top + 0.5,
                    dropdown_rect.width() - 1.0,
                    dropdown_rect.height() - 1.0,
                ),
                Theme::RADIUS_MD,
                Theme::RADIUS_MD,
                &dropdown_border,
            );

            // Draw options
            for (i, option) in self.options.iter().enumerate() {
                let option_rect = self.option_rect(i);

                // Draw hover background (shadcn accent style)
                if self.hover_option == Some(i) {
                    let alpha = (self.option_hover_progress[i] * 255.0) as u8;
                    let mut hover_paint = Paint::default();
                    hover_paint.set_color(Color::from_argb(
                        alpha,
                        Theme::ACCENT.r(),
                        Theme::ACCENT.g(),
                        Theme::ACCENT.b(),
                    ));
                    hover_paint.set_anti_alias(true);
                    canvas.draw_round_rect(
                        Rect::from_xywh(
                            option_rect.left + Theme::SPACE_1,
                            option_rect.top + 1.0,
                            option_rect.width() - (Theme::SPACE_1 * 2.0),
                            option_rect.height() - 2.0,
                        ),
                        Theme::RADIUS_SM,
                        Theme::RADIUS_SM,
                        &hover_paint,
                    );
                }

                // Draw selected indicator
                if i == self.selected_index {
                    let check_x = option_rect.right - 20.0;
                    let check_y = option_rect.top + option_rect.height() / 2.0;
                    
                    let mut check_paint = Paint::default();
                    check_paint.set_color(Theme::PRIMARY);
                    check_paint.set_style(skia_safe::PaintStyle::Stroke);
                    check_paint.set_stroke_width(2.0);
                    check_paint.set_anti_alias(true);
                    
                    canvas.draw_line((check_x - 4.0, check_y), (check_x - 1.0, check_y + 3.0), &check_paint);
                    canvas.draw_line((check_x - 1.0, check_y + 3.0), (check_x + 4.0, check_y - 4.0), &check_paint);
                }

                // Draw option text
                let option_text_x = option_rect.left + Theme::SPACE_2;
                let option_text_y = option_rect.top + option_rect.height() / 2.0 + 5.0;
                
                let font = font_manager.create_font(option, Theme::TEXT_SM, 400);
                let mut text_paint = Paint::default();
                text_paint.set_color(Theme::POPOVER_FOREGROUND);
                text_paint.set_anti_alias(true);
                canvas.draw_str(option, (option_text_x, option_text_y), &font, &text_paint);
            }
        }
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        let button_rect = self.button_rect();
        if x >= button_rect.left && x <= button_rect.right && y >= button_rect.top && y <= button_rect.bottom {
            return true;
        }
        
        if self.open {
            let dropdown_rect = self.dropdown_rect();
            return x >= dropdown_rect.left && x <= dropdown_rect.right && y >= dropdown_rect.top && y <= dropdown_rect.bottom;
        }
        
        false
    }

    fn update_hover(&mut self, x: f32, y: f32) {
        let button_rect = self.button_rect();
        self.hover = x >= button_rect.left && x <= button_rect.right && y >= button_rect.top && y <= button_rect.bottom;

        if self.open {
            self.hover_option = None;
            for i in 0..self.options.len() {
                let option_rect = self.option_rect(i);
                if x >= option_rect.left && x <= option_rect.right && y >= option_rect.top && y <= option_rect.bottom {
                    self.hover_option = Some(i);
                    break;
                }
            }
        }
    }

    fn update_animation(&mut self, _elapsed: f32) {
        let animation_speed = 0.2;
        
        // Button hover animation
        let target_hover = if self.hover { 1.0 } else { 0.0 };
        if (self.hover_progress - target_hover).abs() > 0.01 {
            self.hover_progress += (target_hover - self.hover_progress) * animation_speed;
        } else {
            self.hover_progress = target_hover;
        }

        // Option hover animations
        for i in 0..self.option_hover_progress.len() {
            let target = if self.hover_option == Some(i) { 1.0 } else { 0.0 };
            if (self.option_hover_progress[i] - target).abs() > 0.01 {
                self.option_hover_progress[i] += (target - self.option_hover_progress[i]) * animation_speed;
            } else {
                self.option_hover_progress[i] = target;
            }
        }
    }

    fn on_click(&mut self) {
        if self.hover {
            if self.open {
                // Clicking on an option
                if let Some(index) = self.hover_option {
                    self.selected_index = index;
                    println!("Dropdown selected: {}", self.options[index]);
                }
                self.open = false;
            } else {
                // Open dropdown
                self.open = true;
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
