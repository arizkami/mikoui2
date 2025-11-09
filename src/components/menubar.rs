use skia_safe::{Canvas, Color, Paint, Rect};
use crate::components::{MenuItem, Widget};
use crate::core::FontManager;
use crate::theme::current_theme;

pub struct MenuBarItem {
    pub label: String,
    pub items: Vec<MenuItem>,
}

impl MenuBarItem {
    pub fn new(label: impl Into<String>, items: Vec<MenuItem>) -> Self {
        Self {
            label: label.into(),
            items,
        }
    }
}

pub struct MenuBar {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    menus: Vec<MenuBarItem>,
    active_menu: Option<usize>,
    hover_menu: Option<usize>,
    hover_item: Option<usize>,
    hover_progress: Vec<f32>,
    item_hover_progress: Vec<f32>,
}

impl MenuBar {
    pub fn new(x: f32, y: f32, width: f32, menus: Vec<MenuBarItem>) -> Self {
        let max_items = menus.iter().map(|m| m.items.len()).max().unwrap_or(0);
        let hover_progress = vec![0.0; menus.len()];
        let item_hover_progress = vec![0.0; max_items];
        
        Self {
            x,
            y,
            width,
            height: 40.0,
            menus,
            active_menu: None,
            hover_menu: None,
            hover_item: None,
            hover_progress,
            item_hover_progress,
        }
    }

    fn menu_item_width(&self) -> f32 {
        80.0
    }

    fn menu_item_rect(&self, index: usize) -> Rect {
        let x = self.x + index as f32 * self.menu_item_width();
        Rect::from_xywh(x, self.y, self.menu_item_width(), self.height)
    }

    fn dropdown_rect(&self, menu_index: usize) -> Rect {
        if menu_index >= self.menus.len() {
            return Rect::default();
        }

        let menu_rect = self.menu_item_rect(menu_index);
        let item_height = 32.0;
        let height = self.menus[menu_index].items.iter().map(|item| {
            if item.separator { 9.0 } else { item_height }
        }).sum::<f32>();

        Rect::from_xywh(
            menu_rect.left,
            menu_rect.bottom,
            200.0,
            height,
        )
    }

    fn dropdown_item_rect(&self, menu_index: usize, item_index: usize) -> Rect {
        let dropdown = self.dropdown_rect(menu_index);
        let item_height = 32.0;
        let separator_height = 9.0;

        let mut y = dropdown.top;
        for i in 0..item_index {
            if self.menus[menu_index].items[i].separator {
                y += separator_height;
            } else {
                y += item_height;
            }
        }

        let height = if self.menus[menu_index].items[item_index].separator {
            separator_height
        } else {
            item_height
        };

        Rect::from_xywh(dropdown.left, y, dropdown.width(), height)
    }
}

impl Widget for MenuBar {
    fn draw(&self, canvas: &Canvas, font_manager: &mut FontManager) {
        // Draw menubar background
        let bg_rect = Rect::from_xywh(self.x, self.y, self.width, self.height);
        let mut bg_paint = Paint::default();
        let colors = current_theme();
        bg_paint.set_color(colors.card);
        bg_paint.set_anti_alias(true);
        canvas.draw_rect(bg_rect, &bg_paint);

        // Draw bottom border
        let mut border_paint = Paint::default();
        border_paint.set_color(colors.border);
        border_paint.set_stroke_width(1.0);
        canvas.draw_line(
            (self.x, self.y + self.height),
            (self.x + self.width, self.y + self.height),
            &border_paint,
        );

        // Draw menu items
        for (i, menu) in self.menus.iter().enumerate() {
            let menu_rect = self.menu_item_rect(i);

            // Draw hover/active background
            if self.active_menu == Some(i) || self.hover_menu == Some(i) {
                let alpha = (self.hover_progress[i] * 50.0) as u8;
                let mut hover_paint = Paint::default();
                let accent = colors.accent;
                hover_paint.set_color(Color::from_argb(alpha, accent.r(), accent.g(), accent.b()));
                hover_paint.set_anti_alias(true);
                canvas.draw_round_rect(
                    Rect::from_xywh(
                        menu_rect.left + 4.0,
                        menu_rect.top + 4.0,
                        menu_rect.width() - 8.0,
                        menu_rect.height() - 8.0,
                    ),
                    4.0,
                    4.0,
                    &hover_paint,
                );
            }

            // Draw menu label
            let font = font_manager.create_font(&menu.label, 14.0, 500);
            let text_width = font.measure_str(&menu.label, None).0;
            let text_x = menu_rect.left + (menu_rect.width() - text_width) / 2.0;
            let text_y = menu_rect.top + menu_rect.height() / 2.0 + 5.0;
            
            let mut text_paint = Paint::default();
            text_paint.set_color(colors.foreground);
            text_paint.set_anti_alias(true);
            canvas.draw_str(&menu.label, (text_x, text_y), &font, &text_paint);
        }

        // Draw active dropdown
        if let Some(menu_index) = self.active_menu {
            if menu_index < self.menus.len() {
                let dropdown_rect = self.dropdown_rect(menu_index);

                // Draw shadow
                let shadow_rect = Rect::from_xywh(
                    dropdown_rect.left + 2.0,
                    dropdown_rect.top + 2.0,
                    dropdown_rect.width(),
                    dropdown_rect.height(),
                );
                let mut shadow_paint = Paint::default();
                shadow_paint.set_color(Color::from_argb(50, 0, 0, 0));
                shadow_paint.set_anti_alias(true);
                canvas.draw_round_rect(shadow_rect, 6.0, 6.0, &shadow_paint);

                // Draw background
                let mut dropdown_bg = Paint::default();
                dropdown_bg.set_color(colors.card);
                dropdown_bg.set_anti_alias(true);
                canvas.draw_round_rect(dropdown_rect, 6.0, 6.0, &dropdown_bg);

                // Draw border
                let mut dropdown_border = Paint::default();
                dropdown_border.set_color(colors.border);
                dropdown_border.set_style(skia_safe::PaintStyle::Stroke);
                dropdown_border.set_stroke_width(1.0);
                dropdown_border.set_anti_alias(true);
                canvas.draw_round_rect(dropdown_rect, 6.0, 6.0, &dropdown_border);

                // Draw menu items
                for (i, item) in self.menus[menu_index].items.iter().enumerate() {
                    let item_rect = self.dropdown_item_rect(menu_index, i);

                    if item.separator {
                        // Draw separator
                        let line_y = item_rect.top + item_rect.height() / 2.0;
                        let mut line_paint = Paint::default();
                        line_paint.set_color(colors.border);
                        line_paint.set_stroke_width(1.0);
                        canvas.draw_line(
                            (item_rect.left + 8.0, line_y),
                            (item_rect.right - 8.0, line_y),
                            &line_paint,
                        );
                    } else {
                        // Draw hover background
                        if self.hover_item == Some(i) && !item.disabled {
                            let alpha = (self.item_hover_progress[i] * 255.0) as u8;
                            let mut hover_paint = Paint::default();
                            let accent = colors.accent;
                            hover_paint.set_color(Color::from_argb(alpha, accent.r(), accent.g(), accent.b()));
                            hover_paint.set_anti_alias(true);
                            canvas.draw_round_rect(
                                Rect::from_xywh(
                                    item_rect.left + 4.0,
                                    item_rect.top + 2.0,
                                    item_rect.width() - 8.0,
                                    item_rect.height() - 4.0,
                                ),
                                4.0,
                                4.0,
                                &hover_paint,
                            );
                        }

                        // Draw text
                        let text_color = if item.disabled {
                            colors.muted_foreground
                        } else {
                            colors.foreground
                        };

                        let text_x = item_rect.left + 12.0;
                        let text_y = item_rect.top + item_rect.height() / 2.0 + 5.0;

                        let font = font_manager.create_font(&item.label, 14.0, 400);
                        let mut text_paint = Paint::default();
                        text_paint.set_color(text_color);
                        text_paint.set_anti_alias(true);
                        canvas.draw_str(&item.label, (text_x, text_y), &font, &text_paint);

                        // Draw shortcut
                        if let Some(ref shortcut) = item.shortcut {
                            let font = font_manager.create_font(shortcut, 12.0, 400);
                            let text_width = font.measure_str(shortcut, None).0;
                            let shortcut_x = item_rect.right - 12.0 - text_width;
                            let mut text_paint = Paint::default();
                            text_paint.set_color(colors.muted_foreground);
                            text_paint.set_anti_alias(true);
                            canvas.draw_str(shortcut, (shortcut_x, text_y), &font, &text_paint);
                        }
                    }
                }
            }
        }
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        // Check menubar
        let bar_rect = Rect::from_xywh(self.x, self.y, self.width, self.height);
        if x >= bar_rect.left && x <= bar_rect.right && y >= bar_rect.top && y <= bar_rect.bottom {
            return true;
        }

        // Check active dropdown
        if let Some(menu_index) = self.active_menu {
            let dropdown_rect = self.dropdown_rect(menu_index);
            return x >= dropdown_rect.left && x <= dropdown_rect.right && y >= dropdown_rect.top && y <= dropdown_rect.bottom;
        }

        false
    }

    fn update_hover(&mut self, x: f32, y: f32) {
        // Check menubar items
        self.hover_menu = None;
        for i in 0..self.menus.len() {
            let menu_rect = self.menu_item_rect(i);
            if x >= menu_rect.left && x <= menu_rect.right && y >= menu_rect.top && y <= menu_rect.bottom {
                self.hover_menu = Some(i);
                break;
            }
        }

        // Check dropdown items
        self.hover_item = None;
        if let Some(menu_index) = self.active_menu {
            if menu_index < self.menus.len() {
                for i in 0..self.menus[menu_index].items.len() {
                    let item_rect = self.dropdown_item_rect(menu_index, i);
                    if x >= item_rect.left && x <= item_rect.right && y >= item_rect.top && y <= item_rect.bottom && !self.menus[menu_index].items[i].separator {
                        self.hover_item = Some(i);
                        break;
                    }
                }
            }
        }
    }

    fn update_animation(&mut self, _elapsed: f32) {
        let animation_speed = 0.2;

        // Menu hover animations
        for i in 0..self.hover_progress.len() {
            let target = if self.hover_menu == Some(i) || self.active_menu == Some(i) {
                1.0
            } else {
                0.0
            };
            if (self.hover_progress[i] - target).abs() > 0.01 {
                self.hover_progress[i] += (target - self.hover_progress[i]) * animation_speed;
            } else {
                self.hover_progress[i] = target;
            }
        }

        // Item hover animations
        for i in 0..self.item_hover_progress.len() {
            let target = if self.hover_item == Some(i) { 1.0 } else { 0.0 };
            if (self.item_hover_progress[i] - target).abs() > 0.01 {
                self.item_hover_progress[i] += (target - self.item_hover_progress[i]) * animation_speed;
            } else {
                self.item_hover_progress[i] = target;
            }
        }
    }

    fn on_click(&mut self) {
        // Check if clicking on menubar item
        if let Some(menu_index) = self.hover_menu {
            if self.active_menu == Some(menu_index) {
                self.active_menu = None;
            } else {
                self.active_menu = Some(menu_index);
            }
            return;
        }

        // Check if clicking on dropdown item
        if let Some(menu_index) = self.active_menu {
            if let Some(item_index) = self.hover_item {
                if menu_index < self.menus.len() && item_index < self.menus[menu_index].items.len() {
                    let item = &self.menus[menu_index].items[item_index];
                    if !item.disabled {
                        println!("Menu item clicked: {} (id: {})", item.label, item.id);
                        self.active_menu = None;
                    }
                }
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
