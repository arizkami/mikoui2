use skia_safe::{Canvas, Color, Paint, Rect};
use crate::components::Widget;
use crate::core::FontManager;
use crate::theme::{current_theme, Theme};

#[derive(Clone)]
pub struct MenuItem {
    pub label: String,
    pub id: usize,
    pub icon: Option<&'static str>,
    pub shortcut: Option<String>,
    pub separator: bool,
    pub disabled: bool,
}

impl MenuItem {
    pub fn new(label: impl Into<String>, id: usize) -> Self {
        Self {
            label: label.into(),
            id,
            icon: None,
            shortcut: None,
            separator: false,
            disabled: false,
        }
    }

    pub fn with_icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn separator() -> Self {
        Self {
            label: String::new(),
            id: 0,
            icon: None,
            shortcut: None,
            separator: true,
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

pub struct ContextMenu {
    x: f32,
    y: f32,
    width: f32,
    items: Vec<MenuItem>,
    visible: bool,
    hover_index: Option<usize>,
    hover_progress: Vec<f32>,
}

impl ContextMenu {
    pub fn new(x: f32, y: f32, items: Vec<MenuItem>) -> Self {
        let hover_progress = vec![0.0; items.len()];
        Self {
            x,
            y,
            width: 200.0,
            items,
            visible: false,
            hover_index: None,
            hover_progress,
        }
    }

    pub fn show(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
        self.hover_index = None;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    fn item_height(&self) -> f32 {
        32.0
    }

    fn separator_height(&self) -> f32 {
        9.0
    }
    
    fn padding_top(&self) -> f32 {
        Theme::SPACE_1
    }
    
    fn padding_bottom(&self) -> f32 {
        Theme::SPACE_1
    }

    fn get_item_rect(&self, index: usize) -> Rect {
        let mut y = self.y + self.padding_top();
        for i in 0..index {
            if self.items[i].separator {
                y += self.separator_height();
            } else {
                y += self.item_height();
            }
        }
        
        let height = if self.items[index].separator {
            self.separator_height()
        } else {
            self.item_height()
        };
        
        Rect::from_xywh(self.x, y, self.width, height)
    }

    fn total_height(&self) -> f32 {
        let items_height: f32 = self.items.iter().map(|item| {
            if item.separator {
                self.separator_height()
            } else {
                self.item_height()
            }
        }).sum();
        items_height + self.padding_top() + self.padding_bottom()
    }
}

impl Widget for ContextMenu {
    fn draw(&self, canvas: &Canvas, font_manager: &mut FontManager) {
        if !self.visible {
            return;
        }

        let total_height = self.total_height();
        let border_radius = Theme::RADIUS_MD;
        let padding = Theme::SPACE_1;
        let colors = current_theme();

        // Draw shadow (shadcn style - subtle)
        let shadow_rect = Rect::from_xywh(self.x, self.y + 4.0, self.width, total_height);
        let mut shadow_paint = Paint::default();
        shadow_paint.set_color(Color::from_argb(30, 0, 0, 0));
        shadow_paint.set_anti_alias(true);
        canvas.draw_round_rect(shadow_rect, border_radius, border_radius, &shadow_paint);

        // Draw background (popover style)
        let bg_rect = Rect::from_xywh(self.x, self.y, self.width, total_height);
        let mut bg_paint = Paint::default();
        bg_paint.set_color(colors.popover);
        bg_paint.set_anti_alias(true);
        canvas.draw_round_rect(bg_rect, border_radius, border_radius, &bg_paint);

        // Draw border
        let mut border_paint = Paint::default();
        border_paint.set_color(colors.border);
        border_paint.set_style(skia_safe::PaintStyle::Stroke);
        border_paint.set_stroke_width(1.0);
        border_paint.set_anti_alias(true);
        canvas.draw_round_rect(
            Rect::from_xywh(
                self.x + 0.5,
                self.y + 0.5,
                self.width - 1.0,
                total_height - 1.0,
            ),
            border_radius,
            border_radius,
            &border_paint,
        );

        // Draw items
        for (i, item) in self.items.iter().enumerate() {
            let item_rect = self.get_item_rect(i);

            if item.separator {
                // Draw separator line (shadcn style)
                let line_y = item_rect.top + item_rect.height() / 2.0;
                let mut line_paint = Paint::default();
                line_paint.set_color(colors.border);
                line_paint.set_stroke_width(1.0);
                line_paint.set_anti_alias(true);
                canvas.draw_line(
                    (item_rect.left + Theme::SPACE_2, line_y),
                    (item_rect.right - Theme::SPACE_2, line_y),
                    &line_paint,
                );
            } else {
                // Draw hover background (shadcn accent style)
                if self.hover_index == Some(i) && !item.disabled {
                    let alpha = (self.hover_progress[i] * 255.0) as u8;
                    let mut hover_paint = Paint::default();
                    let accent = colors.accent;
                    hover_paint.set_color(Color::from_argb(alpha, accent.r(), accent.g(), accent.b()));
                    hover_paint.set_anti_alias(true);
                    canvas.draw_round_rect(
                        Rect::from_xywh(
                            item_rect.left + padding,
                            item_rect.top + 1.0,
                            item_rect.width() - (padding * 2.0),
                            item_rect.height() - 2.0,
                        ),
                        Theme::RADIUS_SM,
                        Theme::RADIUS_SM,
                        &hover_paint,
                    );
                }

                // Draw text
                let text_color = if item.disabled {
                    colors.muted_foreground
                } else {
                    colors.popover_foreground
                };

                let text_x = item_rect.left + Theme::SPACE_2;
                let text_y = item_rect.top + item_rect.height() / 2.0 + 5.0;

                let font = font_manager.create_font(&item.label, Theme::TEXT_SM, 400);
                let mut text_paint = Paint::default();
                text_paint.set_color(text_color);
                text_paint.set_anti_alias(true);
                canvas.draw_str(&item.label, (text_x, text_y), &font, &text_paint);

                // Draw shortcut if present (shadcn style)
                if let Some(ref shortcut) = item.shortcut {
                    let font = font_manager.create_font(shortcut, Theme::TEXT_XS, 400);
                    let text_width = font.measure_str(shortcut, None).0;
                    let shortcut_x = item_rect.right - Theme::SPACE_2 - text_width;
                    let mut text_paint = Paint::default();
                    text_paint.set_color(colors.muted_foreground);
                    text_paint.set_anti_alias(true);
                    canvas.draw_str(shortcut, (shortcut_x, text_y), &font, &text_paint);
                }
            }
        }
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        if !self.visible {
            return false;
        }
        let total_height = self.total_height();
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + total_height
    }

    fn update_hover(&mut self, x: f32, y: f32) {
        if !self.visible {
            return;
        }

        self.hover_index = None;
        for (i, _item) in self.items.iter().enumerate() {
            let rect = self.get_item_rect(i);
            if x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom && !self.items[i].separator {
                self.hover_index = Some(i);
                break;
            }
        }
    }

    fn update_animation(&mut self, _elapsed: f32) {
        let animation_speed = 0.2;
        for i in 0..self.hover_progress.len() {
            let target = if self.hover_index == Some(i) { 1.0 } else { 0.0 };
            if (self.hover_progress[i] - target).abs() > 0.01 {
                self.hover_progress[i] += (target - self.hover_progress[i]) * animation_speed;
            } else {
                self.hover_progress[i] = target;
            }
        }
    }

    fn on_click(&mut self) {
        if let Some(index) = self.hover_index {
            if !self.items[index].disabled {
                println!("Menu item clicked: {} (id: {})", self.items[index].label, self.items[index].id);
                self.hide();
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
