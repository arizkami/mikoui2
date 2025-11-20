use crate::tab::TabManager;
use skia_safe::{Canvas, Color, Font, Paint, Rect};

pub struct TabBar {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    hover_tab: Option<usize>,
    hover_close: Option<usize>,
    hover_progress: Vec<f32>,
}

impl TabBar {
    const TAB_HEIGHT: f32 = 36.0;
    const TAB_MIN_WIDTH: f32 = 120.0;
    const TAB_MAX_WIDTH: f32 = 200.0;
    const CLOSE_BUTTON_SIZE: f32 = 16.0;
    
    pub fn new(x: f32, y: f32, width: f32) -> Self {
        Self {
            x,
            y,
            width,
            height: Self::TAB_HEIGHT,
            hover_tab: None,
            hover_close: None,
            hover_progress: Vec::new(),
        }
    }
    
    pub fn height(&self) -> f32 {
        self.height
    }
    
    pub fn set_bounds(&mut self, x: f32, y: f32, width: f32) {
        self.x = x;
        self.y = y;
        self.width = width;
    }
    
    pub fn draw(&self, canvas: &Canvas, font: &Font, tab_manager: &TabManager) {
        // Background
        let mut bg_paint = Paint::default();
        bg_paint.set_color(Color::from_rgb(37, 37, 38));
        bg_paint.set_anti_alias(true);
        canvas.draw_rect(
            Rect::from_xywh(self.x, self.y, self.width, self.height),
            &bg_paint,
        );
        
        let tab_count = tab_manager.tab_count();
        if tab_count == 0 {
            return;
        }
        
        // Calculate tab width
        let available_width = self.width - 40.0; // Leave space for new tab button
        let tab_width = (available_width / tab_count as f32)
            .max(Self::TAB_MIN_WIDTH)
            .min(Self::TAB_MAX_WIDTH);
        
        // Draw tabs
        for (i, tab) in tab_manager.tabs().iter().enumerate() {
            let tab_x = self.x + (i as f32 * tab_width);
            let is_active = i == tab_manager.active_index();
            let is_hovered = self.hover_tab == Some(i);
            
            self.draw_tab(
                canvas,
                font,
                tab_x,
                tab_width,
                &tab.get_display_title(),
                is_active,
                is_hovered,
                i,
            );
        }
        
        // Bottom border
        let mut border_paint = Paint::default();
        border_paint.set_color(Color::from_rgb(60, 60, 60));
        border_paint.set_stroke_width(1.0);
        canvas.draw_line(
            (self.x, self.y + self.height),
            (self.x + self.width, self.y + self.height),
            &border_paint,
        );
    }
    
    fn draw_tab(
        &self,
        canvas: &Canvas,
        font: &Font,
        x: f32,
        width: f32,
        title: &str,
        is_active: bool,
        is_hovered: bool,
        index: usize,
    ) {
        // Tab background
        let mut tab_paint = Paint::default();
        tab_paint.set_anti_alias(true);
        
        if is_active {
            tab_paint.set_color(Color::from_rgb(30, 30, 30));
        } else if is_hovered {
            let hover_alpha = if index < self.hover_progress.len() {
                (50.0 * self.hover_progress[index]) as u8
            } else {
                0
            };
            tab_paint.set_color(Color::from_argb(hover_alpha, 255, 255, 255));
        }
        
        canvas.draw_rect(
            Rect::from_xywh(x, self.y, width, self.height),
            &tab_paint,
        );
        
        // Active tab indicator
        if is_active {
            let mut indicator_paint = Paint::default();
            indicator_paint.set_color(Color::from_rgb(0, 122, 204));
            indicator_paint.set_anti_alias(true);
            canvas.draw_rect(
                Rect::from_xywh(x, self.y, width, 2.0),
                &indicator_paint,
            );
        }
        
        // Tab title
        let text_x = x + 12.0;
        let text_y = self.y + self.height / 2.0 + 5.0;
        let mut text_paint = Paint::default();
        text_paint.set_color(if is_active {
            Color::from_rgb(255, 255, 255)
        } else {
            Color::from_rgb(170, 170, 170)
        });
        text_paint.set_anti_alias(true);
        
        // Truncate title if too long
        let max_text_width = width - 40.0; // Leave space for close button
        let text_width = font.measure_str(title, None).0;
        let display_title = if text_width > max_text_width {
            let mut truncated = title.to_string();
            while font.measure_str(&truncated, None).0 > max_text_width - 20.0 && !truncated.is_empty() {
                truncated.pop();
            }
            format!("{}...", truncated)
        } else {
            title.to_string()
        };
        
        canvas.draw_str(&display_title, (text_x, text_y), font, &text_paint);
        
        // Close button
        let close_x = x + width - 24.0;
        let close_y = self.y + (self.height - Self::CLOSE_BUTTON_SIZE) / 2.0;
        
        if is_hovered || is_active {
            let is_close_hovered = self.hover_close == Some(index);
            
            // Close button background
            if is_close_hovered {
                let mut close_bg = Paint::default();
                close_bg.set_color(Color::from_rgb(90, 90, 90));
                close_bg.set_anti_alias(true);
                canvas.draw_round_rect(
                    Rect::from_xywh(close_x, close_y, Self::CLOSE_BUTTON_SIZE, Self::CLOSE_BUTTON_SIZE),
                    2.0,
                    2.0,
                    &close_bg,
                );
            }
            
            // Close icon (X)
            let mut close_paint = Paint::default();
            close_paint.set_color(Color::from_rgb(200, 200, 200));
            close_paint.set_stroke_width(1.5);
            close_paint.set_anti_alias(true);
            
            let icon_padding = 4.0;
            canvas.draw_line(
                (close_x + icon_padding, close_y + icon_padding),
                (close_x + Self::CLOSE_BUTTON_SIZE - icon_padding, close_y + Self::CLOSE_BUTTON_SIZE - icon_padding),
                &close_paint,
            );
            canvas.draw_line(
                (close_x + Self::CLOSE_BUTTON_SIZE - icon_padding, close_y + icon_padding),
                (close_x + icon_padding, close_y + Self::CLOSE_BUTTON_SIZE - icon_padding),
                &close_paint,
            );
        }
        
        // Tab separator
        if !is_active {
            let mut separator_paint = Paint::default();
            separator_paint.set_color(Color::from_rgb(60, 60, 60));
            separator_paint.set_stroke_width(1.0);
            canvas.draw_line(
                (x + width, self.y + 8.0),
                (x + width, self.y + self.height - 8.0),
                &separator_paint,
            );
        }
    }
    
    pub fn update_hover(&mut self, x: f32, y: f32, tab_manager: &TabManager) {
        self.hover_tab = None;
        self.hover_close = None;
        
        if y < self.y || y > self.y + self.height {
            return;
        }
        
        let tab_count = tab_manager.tab_count();
        if tab_count == 0 {
            return;
        }
        
        let available_width = self.width - 40.0;
        let tab_width = (available_width / tab_count as f32)
            .max(Self::TAB_MIN_WIDTH)
            .min(Self::TAB_MAX_WIDTH);
        
        for i in 0..tab_count {
            let tab_x = self.x + (i as f32 * tab_width);
            
            if x >= tab_x && x < tab_x + tab_width {
                self.hover_tab = Some(i);
                
                // Check if hovering over close button
                let close_x = tab_x + tab_width - 24.0;
                let close_y = self.y + (self.height - Self::CLOSE_BUTTON_SIZE) / 2.0;
                
                if x >= close_x && x < close_x + Self::CLOSE_BUTTON_SIZE &&
                   y >= close_y && y < close_y + Self::CLOSE_BUTTON_SIZE {
                    self.hover_close = Some(i);
                }
                
                break;
            }
        }
    }
    
    pub fn update_animation(&mut self, tab_count: usize) {
        // Ensure hover_progress has enough elements
        while self.hover_progress.len() < tab_count {
            self.hover_progress.push(0.0);
        }
        
        // Animate hover states
        for i in 0..tab_count {
            let target = if self.hover_tab == Some(i) { 1.0 } else { 0.0 };
            let animation_speed = 0.2;
            
            if (self.hover_progress[i] - target).abs() > 0.01 {
                self.hover_progress[i] += (target - self.hover_progress[i]) * animation_speed;
            } else {
                self.hover_progress[i] = target;
            }
        }
    }
    
    pub fn get_clicked_tab(&self, x: f32, y: f32, tab_manager: &TabManager) -> Option<usize> {
        if y < self.y || y > self.y + self.height {
            return None;
        }
        
        let tab_count = tab_manager.tab_count();
        if tab_count == 0 {
            return None;
        }
        
        let available_width = self.width - 40.0;
        let tab_width = (available_width / tab_count as f32)
            .max(Self::TAB_MIN_WIDTH)
            .min(Self::TAB_MAX_WIDTH);
        
        for i in 0..tab_count {
            let tab_x = self.x + (i as f32 * tab_width);
            
            if x >= tab_x && x < tab_x + tab_width {
                return Some(i);
            }
        }
        
        None
    }
    
    pub fn get_close_button_clicked(&self, x: f32, y: f32, tab_manager: &TabManager) -> Option<usize> {
        if y < self.y || y > self.y + self.height {
            return None;
        }
        
        let tab_count = tab_manager.tab_count();
        if tab_count == 0 {
            return None;
        }
        
        let available_width = self.width - 40.0;
        let tab_width = (available_width / tab_count as f32)
            .max(Self::TAB_MIN_WIDTH)
            .min(Self::TAB_MAX_WIDTH);
        
        for i in 0..tab_count {
            let tab_x = self.x + (i as f32 * tab_width);
            let close_x = tab_x + tab_width - 24.0;
            let close_y = self.y + (self.height - Self::CLOSE_BUTTON_SIZE) / 2.0;
            
            if x >= close_x && x < close_x + Self::CLOSE_BUTTON_SIZE &&
               y >= close_y && y < close_y + Self::CLOSE_BUTTON_SIZE {
                return Some(i);
            }
        }
        
        None
    }
}
