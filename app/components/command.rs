use mikoui::{Widget, FontManager};
use mikoui::theme::current_theme;
use mikoui::components::{Icon, IconSize, CodiconIcons};
use skia_safe::{Canvas, Paint, Rect, Color};

/// Command item in the palette
#[derive(Debug, Clone)]
pub struct CommandItem {
    pub id: u32,
    pub label: String,
    pub description: Option<String>,
    pub icon: Option<&'static str>,
    pub shortcut: Option<String>,
    pub category: String,
}

impl CommandItem {
    pub fn new(id: u32, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            description: None,
            icon: None,
            shortcut: None,
            category: "General".to_string(),
        }
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    pub fn with_icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }
    
    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
    
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }
}

/// Command Palette overlay
pub struct CommandPalette {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    visible: bool,
    search_text: String,
    commands: Vec<CommandItem>,
    filtered_commands: Vec<usize>, // Indices into commands
    selected_index: usize,
    hover_index: Option<usize>,
    scroll_offset: f32,
    animation_progress: f32, // 0.0 to 1.0 for fade in/out
    target_visible: bool,
}

impl CommandPalette {
    const ITEM_HEIGHT: f32 = 44.0;
    const MAX_VISIBLE_ITEMS: usize = 10;
    const PALETTE_WIDTH: f32 = 600.0;
    const INPUT_HEIGHT: f32 = 56.0;
    const ANIMATION_SPEED: f32 = 0.15;
    
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let x = (screen_width - Self::PALETTE_WIDTH) / 2.0;
        let y = screen_height * 0.2; // 20% from top
        
        let commands = Self::create_default_commands();
        let filtered_commands: Vec<usize> = (0..commands.len()).collect();
        
        Self {
            x,
            y,
            width: Self::PALETTE_WIDTH,
            height: Self::INPUT_HEIGHT + (Self::MAX_VISIBLE_ITEMS as f32 * Self::ITEM_HEIGHT) + 8.0,
            visible: false,
            search_text: String::new(),
            commands,
            filtered_commands,
            selected_index: 0,
            hover_index: None,
            scroll_offset: 0.0,
            animation_progress: 0.0,
            target_visible: false,
        }
    }
    
    fn create_default_commands() -> Vec<CommandItem> {
        vec![
            // File commands
            CommandItem::new(1, "File: New File")
                .with_icon(CodiconIcons::FILE)
                .with_shortcut("Ctrl+N")
                .with_category("File"),
            CommandItem::new(2, "File: New Window")
                .with_icon(CodiconIcons::WINDOW)
                .with_shortcut("Ctrl+Shift+N")
                .with_category("File"),
            CommandItem::new(3, "File: Open File")
                .with_icon(CodiconIcons::FOLDER_OPENED)
                .with_shortcut("Ctrl+O")
                .with_category("File"),
            CommandItem::new(4, "File: Open Folder")
                .with_icon(CodiconIcons::FOLDER_OPENED)
                .with_shortcut("Ctrl+K Ctrl+O")
                .with_category("File"),
            CommandItem::new(6, "File: Save")
                .with_icon(CodiconIcons::SAVE)
                .with_shortcut("Ctrl+S")
                .with_category("File"),
            CommandItem::new(7, "File: Save As")
                .with_icon(CodiconIcons::SAVE_AS)
                .with_shortcut("Ctrl+Shift+S")
                .with_category("File"),
            
            // View commands
            CommandItem::new(62, "View: Show Explorer")
                .with_icon(CodiconIcons::FILES)
                .with_shortcut("Ctrl+Shift+E")
                .with_category("View"),
            CommandItem::new(63, "View: Show Search")
                .with_icon(CodiconIcons::SEARCH)
                .with_shortcut("Ctrl+Shift+F")
                .with_category("View"),
            CommandItem::new(64, "View: Show Source Control")
                .with_icon(CodiconIcons::SOURCE_CONTROL)
                .with_shortcut("Ctrl+Shift+G")
                .with_category("View"),
            CommandItem::new(69, "View: Toggle Terminal")
                .with_icon(CodiconIcons::TERMINAL)
                .with_shortcut("Ctrl+`")
                .with_category("View"),
            CommandItem::new(76, "View: Toggle Full Screen")
                .with_icon(CodiconIcons::SCREEN_FULL)
                .with_shortcut("F11")
                .with_category("View"),
            
            // Edit commands
            CommandItem::new(29, "Edit: Find")
                .with_icon(CodiconIcons::SEARCH)
                .with_shortcut("Ctrl+F")
                .with_category("Edit"),
            CommandItem::new(32, "Edit: Replace")
                .with_icon(CodiconIcons::REPLACE)
                .with_shortcut("Ctrl+H")
                .with_category("Edit"),
            CommandItem::new(39, "Edit: Format Document")
                .with_icon(CodiconIcons::SYMBOL_RULER)
                .with_shortcut("Shift+Alt+F")
                .with_category("Edit"),
            
            // Go commands
            CommandItem::new(84, "Go: Go to File")
                .with_icon(CodiconIcons::GO_TO_FILE)
                .with_shortcut("Ctrl+P")
                .with_category("Go"),
            CommandItem::new(91, "Go: Go to Line")
                .with_icon(CodiconIcons::ARROW_RIGHT)
                .with_shortcut("Ctrl+G")
                .with_category("Go"),
            
            // Terminal commands
            CommandItem::new(120, "Terminal: New Terminal")
                .with_icon(CodiconIcons::TERMINAL)
                .with_shortcut("Ctrl+Shift+`")
                .with_category("Terminal"),
        ]
    }
    
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    pub fn is_animating(&self) -> bool {
        let target = if self.target_visible { 1.0 } else { 0.0 };
        (self.animation_progress - target).abs() > 0.01
    }
    
    pub fn show(&mut self) {
        self.target_visible = true;
        self.search_text.clear();
        self.selected_index = 0;
        self.hover_index = None;
        self.scroll_offset = 0.0;
        self.update_filter();
    }
    
    pub fn hide(&mut self) {
        self.target_visible = false;
        self.search_text.clear();
    }
    
    pub fn toggle(&mut self) {
        if self.target_visible {
            self.hide();
        } else {
            self.show();
        }
    }
    
    pub fn update_position(&mut self, screen_width: f32, screen_height: f32) {
        self.x = (screen_width - self.width) / 2.0;
        self.y = screen_height * 0.15;
    }
    
    pub fn handle_key_input(&mut self, key: &str) -> Option<u32> {
        match key {
            "Escape" => {
                self.hide();
                None
            }
            "Enter" => {
                if !self.filtered_commands.is_empty() && self.selected_index < self.filtered_commands.len() {
                    let cmd_index = self.filtered_commands[self.selected_index];
                    let command_id = self.commands[cmd_index].id;
                    self.hide();
                    Some(command_id)
                } else {
                    None
                }
            }
            "ArrowUp" => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                    self.ensure_selected_visible();
                }
                None
            }
            "ArrowDown" => {
                if self.selected_index < self.filtered_commands.len().saturating_sub(1) {
                    self.selected_index += 1;
                    self.ensure_selected_visible();
                }
                None
            }
            "Backspace" => {
                self.search_text.pop();
                self.update_filter();
                None
            }
            _ => {
                // Add character to search
                if key.len() == 1 {
                    self.search_text.push_str(key);
                    self.update_filter();
                }
                None
            }
        }
    }
    
    pub fn add_char(&mut self, c: char) {
        self.search_text.push(c);
        self.update_filter();
    }
    
    pub fn backspace(&mut self) {
        self.search_text.pop();
        self.update_filter();
    }
    
    fn update_filter(&mut self) {
        if self.search_text.is_empty() {
            self.filtered_commands = (0..self.commands.len()).collect();
        } else {
            let search_lower = self.search_text.to_lowercase();
            self.filtered_commands = self.commands
                .iter()
                .enumerate()
                .filter(|(_, cmd)| {
                    cmd.label.to_lowercase().contains(&search_lower) ||
                    cmd.category.to_lowercase().contains(&search_lower) ||
                    cmd.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&search_lower))
                })
                .map(|(i, _)| i)
                .collect();
        }
        
        // Reset selection
        self.selected_index = 0;
        self.scroll_offset = 0.0;
    }
    
    fn ensure_selected_visible(&mut self) {
        let item_y = self.selected_index as f32 * Self::ITEM_HEIGHT;
        let visible_height = Self::MAX_VISIBLE_ITEMS as f32 * Self::ITEM_HEIGHT;
        
        if item_y < self.scroll_offset {
            self.scroll_offset = item_y;
        } else if item_y + Self::ITEM_HEIGHT > self.scroll_offset + visible_height {
            self.scroll_offset = item_y + Self::ITEM_HEIGHT - visible_height;
        }
    }
    
    pub fn get_selected_command(&self) -> Option<u32> {
        if !self.filtered_commands.is_empty() && self.selected_index < self.filtered_commands.len() {
            let cmd_index = self.filtered_commands[self.selected_index];
            Some(self.commands[cmd_index].id)
        } else {
            None
        }
    }
}

impl Widget for CommandPalette {
    fn draw(&self, canvas: &Canvas, font_manager: &mut FontManager) {
        if self.animation_progress <= 0.0 {
            return;
        }
        
        let theme = current_theme();
        let alpha_multiplier = self.animation_progress;
        
        // Draw overlay background with fade
        let mut overlay_paint = Paint::default();
        let overlay_alpha = (120.0 * alpha_multiplier) as u8;
        overlay_paint.set_color(Color::from_argb(overlay_alpha, 0, 0, 0));
        overlay_paint.set_anti_alias(true);
        canvas.draw_rect(
            Rect::from_xywh(0.0, 0.0, 10000.0, 10000.0),
            &overlay_paint,
        );
        
        // Apply scale and position animation
        let scale = 0.95 + (0.05 * alpha_multiplier);
        let offset_y = (1.0 - alpha_multiplier) * -10.0;
        
        canvas.save();
        canvas.translate((self.x + self.width / 2.0, self.y + self.height / 2.0 + offset_y));
        canvas.scale((scale, scale));
        canvas.translate((-(self.width / 2.0), -(self.height / 2.0)));
        
        // Draw palette background with shadow
        let shadow_rect = Rect::from_xywh(
            2.0,
            2.0,
            self.width,
            self.height,
        );
        let mut shadow_paint = Paint::default();
        let shadow_alpha = (80.0 * alpha_multiplier) as u8;
        shadow_paint.set_color(Color::from_argb(shadow_alpha, 0, 0, 0));
        shadow_paint.set_anti_alias(true);
        if let Some(blur) = skia_safe::MaskFilter::blur(skia_safe::BlurStyle::Normal, 16.0, false) {
            shadow_paint.set_mask_filter(blur);
        }
        canvas.draw_round_rect(shadow_rect, 6.0, 6.0, &shadow_paint);
        
        // Palette background - VSCode style
        let palette_rect = Rect::from_xywh(0.0, 0.0, self.width, self.height);
        let mut bg_paint = Paint::default();
        let card = theme.card;
        let bg_alpha = (card.a() as f32 * alpha_multiplier) as u8;
        bg_paint.set_color(Color::from_argb(bg_alpha, card.r(), card.g(), card.b()));
        bg_paint.set_anti_alias(true);
        canvas.draw_round_rect(palette_rect, 6.0, 6.0, &bg_paint);
        
        // Border
        let mut border_paint = Paint::default();
        let border = theme.border;
        let border_alpha = (border.a() as f32 * alpha_multiplier) as u8;
        border_paint.set_color(Color::from_argb(border_alpha, border.r(), border.g(), border.b()));
        border_paint.set_style(skia_safe::PaintStyle::Stroke);
        border_paint.set_stroke_width(1.0);
        border_paint.set_anti_alias(true);
        canvas.draw_round_rect(palette_rect, 6.0, 6.0, &border_paint);
        
        // Draw search input
        let input_padding = 16.0;
        
        // Search icon with alpha
        let muted = theme.muted_foreground;
        let icon_alpha = (muted.a() as f32 * alpha_multiplier) as u8;
        let icon_color = Color::from_argb(icon_alpha, muted.r(), muted.g(), muted.b());
        
        let search_icon = Icon::new(
            input_padding + 4.0,
            20.0,
            CodiconIcons::SEARCH,
            IconSize::Small,
            icon_color,
        );
        search_icon.draw(canvas, font_manager);
        
        // Search text or placeholder
        let text_x = input_padding + 32.0;
        let text_y = 32.0;
        
        if self.search_text.is_empty() {
            let placeholder = "Type a command or search...";
            let font = font_manager.create_font(placeholder, 13.0, 400);
            let mut text_paint = Paint::default();
            let muted = theme.muted_foreground;
            let text_alpha = (muted.a() as f32 * alpha_multiplier) as u8;
            text_paint.set_color(Color::from_argb(text_alpha, muted.r(), muted.g(), muted.b()));
            text_paint.set_anti_alias(true);
            canvas.draw_str(placeholder, (text_x, text_y), &font, &text_paint);
        } else {
            let font = font_manager.create_font(&self.search_text, 13.0, 400);
            let mut text_paint = Paint::default();
            let fg = theme.foreground;
            let text_alpha = (fg.a() as f32 * alpha_multiplier) as u8;
            text_paint.set_color(Color::from_argb(text_alpha, fg.r(), fg.g(), fg.b()));
            text_paint.set_anti_alias(true);
            canvas.draw_str(&self.search_text, (text_x, text_y), &font, &text_paint);
        }
        
        // Draw separator
        let separator_y = Self::INPUT_HEIGHT;
        let mut sep_paint = Paint::default();
        let sep_border = theme.border;
        let sep_alpha = (sep_border.a() as f32 * alpha_multiplier) as u8;
        sep_paint.set_color(Color::from_argb(sep_alpha, sep_border.r(), sep_border.g(), sep_border.b()));
        sep_paint.set_stroke_width(1.0);
        canvas.draw_line(
            (0.0, separator_y),
            (self.width, separator_y),
            &sep_paint,
        );
        
        // Draw command items
        let items_start_y = Self::INPUT_HEIGHT + 4.0;
        let visible_height = self.height - items_start_y - 4.0;
        
        canvas.save();
        let clip_rect = Rect::from_xywh(
            0.0,
            items_start_y,
            self.width,
            visible_height,
        );
        canvas.clip_rect(clip_rect, None, Some(true));
        
        for (i, &cmd_index) in self.filtered_commands.iter().enumerate() {
            let item_y = items_start_y + (i as f32 * Self::ITEM_HEIGHT) - self.scroll_offset;
            
            // Skip if not visible
            if item_y + Self::ITEM_HEIGHT < items_start_y || item_y > items_start_y + visible_height {
                continue;
            }
            
            let command = &self.commands[cmd_index];
            let is_selected = i == self.selected_index;
            let is_hovered = self.hover_index == Some(i);
            
            // Draw selection/hover background - VSCode style
            if is_selected || is_hovered {
                let mut item_bg = Paint::default();
                let base_alpha = if is_selected { 180 } else { 100 };
                let final_alpha = ((base_alpha as f32) * alpha_multiplier) as u8;
                let accent = theme.accent;
                item_bg.set_color(Color::from_argb(final_alpha, accent.r(), accent.g(), accent.b()));
                item_bg.set_anti_alias(true);
                canvas.draw_rect(
                    Rect::from_xywh(
                        0.0,
                        item_y,
                        self.width,
                        Self::ITEM_HEIGHT,
                    ),
                    &item_bg,
                );
            }
            
            // Draw icon with alpha
            if let Some(icon) = command.icon {
                let fg = theme.foreground;
                let icon_alpha = (fg.a() as f32 * alpha_multiplier) as u8;
                let icon_color = Color::from_argb(icon_alpha, fg.r(), fg.g(), fg.b());
                
                let icon_widget = Icon::new(
                    16.0,
                    item_y + 14.0,
                    icon,
                    IconSize::Small,
                    icon_color,
                );
                icon_widget.draw(canvas, font_manager);
            }
            
            // Draw label
            let label_x = 44.0;
            let label_y = item_y + 27.0;
            let font = font_manager.create_font(&command.label, 13.0, 400);
            let mut text_paint = Paint::default();
            let fg = theme.foreground;
            let text_alpha = (fg.a() as f32 * alpha_multiplier) as u8;
            text_paint.set_color(Color::from_argb(text_alpha, fg.r(), fg.g(), fg.b()));
            text_paint.set_anti_alias(true);
            canvas.draw_str(&command.label, (label_x, label_y), &font, &text_paint);
            
            // Draw shortcut - VSCode style with background
            if let Some(ref shortcut) = command.shortcut {
                let font = font_manager.create_font(shortcut, 11.0, 400);
                let text_width = font.measure_str(shortcut, None).0;
                let padding = 6.0;
                let shortcut_x = self.width - 16.0 - text_width - padding * 2.0;
                let shortcut_y = item_y + 26.0;
                
                // Draw shortcut background
                let mut shortcut_bg = Paint::default();
                let bg_alpha = (40.0 * alpha_multiplier) as u8;
                shortcut_bg.set_color(Color::from_argb(bg_alpha, 255, 255, 255));
                shortcut_bg.set_anti_alias(true);
                canvas.draw_round_rect(
                    Rect::from_xywh(
                        shortcut_x - padding,
                        item_y + 12.0,
                        text_width + padding * 2.0,
                        20.0,
                    ),
                    3.0,
                    3.0,
                    &shortcut_bg,
                );
                
                // Draw shortcut text
                let mut shortcut_paint = Paint::default();
                let muted = theme.muted_foreground;
                let shortcut_alpha = (muted.a() as f32 * alpha_multiplier) as u8;
                shortcut_paint.set_color(Color::from_argb(shortcut_alpha, muted.r(), muted.g(), muted.b()));
                shortcut_paint.set_anti_alias(true);
                canvas.draw_str(shortcut, (shortcut_x, shortcut_y), &font, &shortcut_paint);
            }
        }
        
        canvas.restore();
        
        canvas.restore(); // Restore from scale/translate
    }
    
    fn contains(&self, x: f32, y: f32) -> bool {
        if !self.visible {
            return false;
        }
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
    
    fn update_hover(&mut self, x: f32, y: f32) {
        if !self.visible || !self.contains(x, y) {
            self.hover_index = None;
            return;
        }
        
        let items_start_y = self.y + Self::INPUT_HEIGHT + 4.0;
        let relative_y = y - items_start_y + self.scroll_offset;
        
        if relative_y >= 0.0 {
            let index = (relative_y / Self::ITEM_HEIGHT) as usize;
            if index < self.filtered_commands.len() {
                self.hover_index = Some(index);
            } else {
                self.hover_index = None;
            }
        } else {
            self.hover_index = None;
        }
    }
    
    fn update_animation(&mut self, _elapsed: f32) {
        // Update animation progress
        let target = if self.target_visible { 1.0 } else { 0.0 };
        
        if (self.animation_progress - target).abs() > 0.01 {
            let delta = (target - self.animation_progress) * Self::ANIMATION_SPEED;
            self.animation_progress += delta;
        } else {
            self.animation_progress = target;
        }
        
        // Update visible state based on animation
        self.visible = self.animation_progress > 0.0;
    }
    
    fn on_click(&mut self) {
        if let Some(index) = self.hover_index {
            self.selected_index = index;
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
