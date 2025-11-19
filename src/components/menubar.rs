use skia_safe::{Canvas, Color, Paint, Rect, Image, Data};
use crate::components::{MenuItem, Widget};
use crate::core::FontManager;
use crate::theme::current_theme;

// Embed the app logo
const APP_LOGO: &[u8] = include_bytes!("../../app/assets/logo.png");

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
    menu_widths: Vec<f32>,
    active_menu: Option<usize>,
    hover_menu: Option<usize>,
    hover_item: Option<usize>,
    hover_progress: Vec<f32>,
    item_hover_progress: Vec<f32>,
    app_logo: std::cell::RefCell<Option<std::sync::Arc<Image>>>,
}

impl MenuBar {
    pub fn new(x: f32, y: f32, width: f32, menus: Vec<MenuBarItem>) -> Self {
        let max_items = menus.iter().map(|m| m.items.len()).max().unwrap_or(0);
        let hover_progress = vec![0.0; menus.len()];
        let item_hover_progress = vec![0.0; max_items];
        let menu_widths = vec![0.0; menus.len()];
        
        Self {
            x,
            y,
            width,
            height: 34.0,
            menus,
            menu_widths,
            active_menu: None,
            hover_menu: None,
            hover_item: None,
            hover_progress,
            item_hover_progress,
            app_logo: std::cell::RefCell::new(None),
        }
    }
    
    /// Get the ID of the currently hovered menu item (if any)
    pub fn get_clicked_item_id(&self) -> Option<i32> {
        if let Some(menu_index) = self.active_menu {
            if let Some(item_index) = self.hover_item {
                if menu_index < self.menus.len() && item_index < self.menus[menu_index].items.len() {
                    let item = &self.menus[menu_index].items[item_index];
                    if !item.disabled {
                        return Some(item.id as i32);
                    }
                }
            }
        }
        None
    }

    fn load_app_logo(&self) -> Option<Image> {
        let data = Data::new_copy(APP_LOGO);
        Image::from_encoded(data)
    }

    fn get_app_logo(&self) -> Option<std::sync::Arc<Image>> {
        if self.app_logo.borrow().is_none() {
            if let Some(img) = self.load_app_logo() {
                *self.app_logo.borrow_mut() = Some(std::sync::Arc::new(img));
            }
        }
        self.app_logo.borrow().clone()
    }

    const LOGO_SIZE: f32 = 16.0;
    const LOGO_PADDING: f32 = 8.0;

    fn calculate_menu_width(&self, menu_label: &str, font_manager: &mut FontManager) -> f32 {
        let font = font_manager.create_font(menu_label, 11.0, 400);
        let text_width = font.measure_str(menu_label, None).0;
        // Add padding: 12px left + 12px right + 2px spacing
        text_width + 16.0
    }

    fn menu_item_rect(&self, index: usize, font_manager: &mut FontManager) -> Rect {
        // Start after the logo
        let mut x = self.x + Self::LOGO_SIZE + Self::LOGO_PADDING * 2.0;
        for i in 0..index {
            if i < self.menus.len() {
                x += self.calculate_menu_width(&self.menus[i].label, font_manager);
            }
        }
        let width = if index < self.menus.len() {
            self.calculate_menu_width(&self.menus[index].label, font_manager)
        } else {
            60.0
        };
        Rect::from_xywh(x, self.y, width, self.height)
    }
    
    pub fn total_width(&self, font_manager: &mut FontManager) -> f32 {
        let menus_width: f32 = self.menus.iter()
            .map(|menu| self.calculate_menu_width(&menu.label, font_manager))
            .sum();
        // Add logo width and padding
        Self::LOGO_SIZE + Self::LOGO_PADDING * 2.0 + menus_width
    }
    
    pub fn update_hover_with_font(&mut self, x: f32, y: f32, font_manager: &mut FontManager) {
        // Check menubar items with proper width calculation
        self.hover_menu = None;
        
        // Skip hover if over logo area
        let logo_end_x = self.x + Self::LOGO_SIZE + Self::LOGO_PADDING * 2.0;
        if x >= self.x && x < logo_end_x && y >= self.y && y <= self.y + self.height {
            // Over logo, no menu hover
        } else {
            for i in 0..self.menus.len() {
                let menu_rect = self.menu_item_rect(i, font_manager);
                if x >= menu_rect.left && x <= menu_rect.right && y >= menu_rect.top && y <= menu_rect.bottom {
                    self.hover_menu = Some(i);
                    break;
                }
            }
        }

        // Check dropdown items
        self.hover_item = None;
        if let Some(menu_index) = self.active_menu {
            if menu_index < self.menus.len() {
                for i in 0..self.menus[menu_index].items.len() {
                    let item_rect = self.dropdown_item_rect(menu_index, i, font_manager);
                    if x >= item_rect.left && x <= item_rect.right && y >= item_rect.top && y <= item_rect.bottom {
                        if !self.menus[menu_index].items[i].separator {
                            self.hover_item = Some(i);
                        }
                        break;
                    }
                }
            }
        }
    }

    fn dropdown_rect(&self, menu_index: usize, font_manager: &mut FontManager) -> Rect {
        if menu_index >= self.menus.len() {
            return Rect::default();
        }

        let menu_rect = self.menu_item_rect(menu_index, font_manager);
        let item_height = 32.0;
        let height = self.menus[menu_index].items.iter().map(|item| {
            if item.separator { 9.0 } else { item_height }
        }).sum::<f32>();

        // Calculate the maximum width needed for all items
        let mut max_width: f32 = 180.0; // Minimum width
        for item in &self.menus[menu_index].items {
            if !item.separator {
                // Measure label width
                let label_font = font_manager.create_font(&item.label, 12.0, 400);
                let label_width = label_font.measure_str(&item.label, None).0;
                
                // Measure shortcut width if present
                let shortcut_width = if let Some(ref shortcut) = item.shortcut {
                    let shortcut_font = font_manager.create_font(shortcut, 12.0, 400);
                    shortcut_font.measure_str(shortcut, None).0 + 24.0 // Add gap between label and shortcut
                } else {
                    0.0
                };
                
                // Total width: left padding + label + shortcut + right padding
                let total_width = 12.0 + label_width + shortcut_width + 12.0;
                max_width = max_width.max(total_width);
            }
        }

        Rect::from_xywh(
            menu_rect.left,
            menu_rect.bottom,
            max_width,
            height,
        )
    }

    fn dropdown_item_rect(&self, menu_index: usize, item_index: usize, font_manager: &mut FontManager) -> Rect {
        let dropdown = self.dropdown_rect(menu_index, font_manager);
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

impl MenuBar {
    /// Draw only the menubar items (not the dropdown)
    pub fn draw_menubar_only(&self, canvas: &Canvas, font_manager: &mut FontManager) {
        let colors = current_theme();

        // Draw app logo
        if let Some(logo) = self.get_app_logo() {
            let logo_x = self.x + Self::LOGO_PADDING;
            let logo_y = self.y + (self.height - Self::LOGO_SIZE) / 2.0;
            
            let dest_rect = Rect::from_xywh(logo_x, logo_y, Self::LOGO_SIZE, Self::LOGO_SIZE);
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            canvas.draw_image_rect(logo.as_ref(), None, dest_rect, &paint);
        }

        // Draw menu items
        for (i, menu) in self.menus.iter().enumerate() {
            let menu_rect = self.menu_item_rect(i, font_manager);

            // Draw hover/active background
            if self.active_menu == Some(i) || self.hover_menu == Some(i) {
                let alpha = (self.hover_progress[i] * 80.0) as u8;
                let mut hover_paint = Paint::default();
                let muted = colors.muted;
                hover_paint.set_color(Color::from_argb(alpha, muted.r(), muted.g(), muted.b()));
                hover_paint.set_anti_alias(true);
                canvas.draw_rect(menu_rect, &hover_paint);
            }

            // Draw menu label
            let font_size = 12.0;
            let font = font_manager.create_font(&menu.label, font_size, 400);
            let text_width = font.measure_str(&menu.label, None).0;
            let text_x = menu_rect.left + (menu_rect.width() - text_width) / 2.0;
            let text_y = menu_rect.top + (menu_rect.height() + font_size) / 2.0 - 2.0;
            
            let mut text_paint = Paint::default();
            text_paint.set_color(colors.foreground);
            text_paint.set_anti_alias(true);
            canvas.draw_str(&menu.label, (text_x, text_y), &font, &text_paint);
        }
    }

    /// Draw only the dropdown menu (on top of everything)
    pub fn draw_dropdown_only(&self, canvas: &Canvas, font_manager: &mut FontManager) {
        let colors = current_theme();

        // Draw active dropdown
        if let Some(menu_index) = self.active_menu {
            if menu_index < self.menus.len() {
                let dropdown_rect = self.dropdown_rect(menu_index, font_manager);

                // Draw shadow with blur effect
                let shadow_rect = Rect::from_xywh(
                    dropdown_rect.left + 2.0,
                    dropdown_rect.top + 2.0,
                    dropdown_rect.width(),
                    dropdown_rect.height(),
                );
                let mut shadow_paint = Paint::default();
                shadow_paint.set_color(Color::from_argb(80, 0, 0, 0));
                shadow_paint.set_anti_alias(true);
                
                // Add blur effect to shadow
                if let Some(blur) = skia_safe::MaskFilter::blur(
                    skia_safe::BlurStyle::Normal,
                    8.0,
                    false,
                ) {
                    shadow_paint.set_mask_filter(blur);
                }
                canvas.draw_round_rect(shadow_rect, 6.0, 6.0, &shadow_paint);

                // Mica Effect: Multi-layer translucent background
                let card_color = colors.card;
                
                // Layer 1: Base translucent layer (Mica base)
                let mut base_layer = Paint::default();
                base_layer.set_color(Color::from_argb(
                    200, // 78% opacity for Mica effect
                    card_color.r(),
                    card_color.g(),
                    card_color.b(),
                ));
                base_layer.set_anti_alias(true);
                canvas.draw_round_rect(dropdown_rect, 6.0, 6.0, &base_layer);
                
                // Layer 2: Subtle tint overlay for depth
                let mut tint_layer = Paint::default();
                let tint_alpha = 15; // Very subtle tint
                tint_layer.set_color(Color::from_argb(
                    tint_alpha,
                    card_color.r().saturating_add(10),
                    card_color.g().saturating_add(10),
                    card_color.b().saturating_add(10),
                ));
                tint_layer.set_anti_alias(true);
                canvas.draw_round_rect(dropdown_rect, 6.0, 6.0, &tint_layer);
                
                // Layer 3: Noise texture for Mica material feel
                // Create a subtle noise pattern
                canvas.save();
                let rrect = skia_safe::RRect::new_rect_xy(dropdown_rect, 6.0, 6.0);
                canvas.clip_rrect(rrect, None, Some(true));
                
                let mut noise_paint = Paint::default();
                noise_paint.set_anti_alias(true);
                
                // Draw subtle noise dots
                for y in (dropdown_rect.top as i32..dropdown_rect.bottom as i32).step_by(3) {
                    for x in (dropdown_rect.left as i32..dropdown_rect.right as i32).step_by(3) {
                        // Pseudo-random noise based on position
                        let noise_val = ((x * 7 + y * 13) % 255) as u8;
                        if noise_val > 200 { // Only draw ~20% of pixels for subtle effect
                            let alpha = (noise_val as f32 / 255.0 * 8.0) as u8; // Very low opacity
                            noise_paint.set_color(Color::from_argb(alpha, 255, 255, 255));
                            canvas.draw_circle((x as f32, y as f32), 0.5, &noise_paint);
                        }
                    }
                }
                
                canvas.restore();

                // Draw border
                let mut dropdown_border = Paint::default();
                dropdown_border.set_color(colors.border);
                dropdown_border.set_style(skia_safe::PaintStyle::Stroke);
                dropdown_border.set_stroke_width(1.0);
                dropdown_border.set_anti_alias(true);
                canvas.draw_round_rect(dropdown_rect, 6.0, 6.0, &dropdown_border);

                // Draw menu items
                for (i, item) in self.menus[menu_index].items.iter().enumerate() {
                    let item_rect = self.dropdown_item_rect(menu_index, i, font_manager);

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

                        let font = font_manager.create_font(&item.label, 12.0, 400);
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
}

impl Widget for MenuBar {
    fn draw(&self, canvas: &Canvas, font_manager: &mut FontManager) {
        // Draw both menubar and dropdown (for backward compatibility)
        self.draw_menubar_only(canvas, font_manager);
        self.draw_dropdown_only(canvas, font_manager);
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        // Check menubar - only the area where menu items are
        if y >= self.y && y <= self.y + self.height {
            // Check if within any menu item bounds (approximate)
            let mut current_x = self.x;
            for menu in &self.menus {
                let estimated_width = menu.label.len() as f32 * 8.0 + 26.0;
                if x >= current_x && x <= current_x + estimated_width {
                    return true;
                }
                current_x += estimated_width;
            }
        }

        // Check active dropdown
        if self.active_menu.is_some() {
            // Assume dropdown is below menubar
            return y > self.y + self.height && y < self.y + self.height + 400.0 && x < 250.0;
        }

        false
    }

    fn update_hover(&mut self, x: f32, y: f32) {
        // Simplified version for Widget trait - use update_hover_with_font instead
        self.hover_menu = None;
        let mut current_x = self.x;
        
        for i in 0..self.menus.len() {
            // Estimate width based on label length
            let estimated_width = self.menus[i].label.len() as f32 * 8.0 + 26.0;
            if x >= current_x && x <= current_x + estimated_width && y >= self.y && y <= self.y + self.height {
                self.hover_menu = Some(i);
                break;
            }
            current_x += estimated_width;
        }

        // Check dropdown items - simplified
        self.hover_item = None;
        if let Some(menu_index) = self.active_menu {
            if menu_index < self.menus.len() && y > self.y + self.height {
                let item_height = 32.0;
                let relative_y = y - (self.y + self.height);
                let mut accumulated_height = 0.0;
                
                for i in 0..self.menus[menu_index].items.len() {
                    let height = if self.menus[menu_index].items[i].separator { 9.0 } else { item_height };
                    if relative_y >= accumulated_height && relative_y < accumulated_height + height {
                        if !self.menus[menu_index].items[i].separator {
                            self.hover_item = Some(i);
                        }
                        break;
                    }
                    accumulated_height += height;
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
