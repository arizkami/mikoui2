use skia_safe::{Canvas, Paint, Rect};

use crate::components::Widget;
use crate::theme::{current_theme, lerp_color, with_alpha, Size, Theme};

pub struct Input {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    placeholder: &'static str,
    text: String,
    focused: bool,
    hover: bool,
    hover_progress: f32,
    focus_progress: f32,
    cursor_visible: bool,
    cursor_timer: f32,
    cursor_blink_speed: f32,
    size: Size,
    disabled: bool,
    cursor_pos: usize,
    selection_start: Option<usize>,
    selection_end: Option<usize>,
    clipboard: String, // Simple clipboard storage
}

impl Input {
    pub fn new(x: f32, y: f32, width: f32, placeholder: &'static str) -> Self {
        let size = Size::Md;
        Self {
            x,
            y,
            width,
            height: size.height(),
            placeholder,
            text: String::new(),
            focused: false,
            hover: false,
            hover_progress: 0.0,
            focus_progress: 0.0,
            cursor_visible: true,
            cursor_timer: 0.0,
            cursor_blink_speed: 1.0,
            size,
            disabled: false,
            cursor_pos: 0,
            selection_start: None,
            selection_end: None,
            clipboard: String::new(),
        }
    }
    
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self.height = size.height();
        self
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    // Convert character index to byte index safely
    fn char_to_byte_idx(&self, char_idx: usize) -> usize {
        self.text
            .char_indices()
            .nth(char_idx)
            .map(|(byte_idx, _)| byte_idx)
            .unwrap_or(self.text.len())
    }
    
    // Convert byte index to character index
    fn byte_to_char_idx(&self, byte_idx: usize) -> usize {
        self.text[..byte_idx.min(self.text.len())]
            .chars()
            .count()
    }
    
    // Get character count
    fn char_count(&self) -> usize {
        self.text.chars().count()
    }

    pub fn handle_char(&mut self, c: char) {
        if self.focused && !c.is_control() && !self.disabled {
            // Delete selection if any
            if self.has_selection() {
                self.delete_selection();
            }
            
            let byte_pos = self.char_to_byte_idx(self.cursor_pos);
            self.text.insert(byte_pos, c);
            self.cursor_pos += 1;
            self.clear_selection();
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.focused && !self.disabled {
            if self.has_selection() {
                self.delete_selection();
            } else if self.cursor_pos > 0 {
                let byte_pos = self.char_to_byte_idx(self.cursor_pos - 1);
                self.text.remove(byte_pos);
                self.cursor_pos -= 1;
            }
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_pos = 0;
        self.clear_selection();
    }
    
    pub fn select_all(&mut self) {
        if !self.text.is_empty() {
            self.selection_start = Some(0);
            self.selection_end = Some(self.char_count());
            self.cursor_pos = self.char_count();
        }
    }
    
    pub fn has_selection(&self) -> bool {
        self.selection_start.is_some() && self.selection_end.is_some()
    }
    
    pub fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }
    
    pub fn get_selection(&self) -> Option<(usize, usize)> {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            Some((start.min(end), start.max(end)))
        } else {
            None
        }
    }
    
    pub fn delete_selection(&mut self) {
        if let Some((start, end)) = self.get_selection() {
            let byte_start = self.char_to_byte_idx(start);
            let byte_end = self.char_to_byte_idx(end);
            self.text.drain(byte_start..byte_end);
            self.cursor_pos = start;
            self.clear_selection();
        }
    }
    
    pub fn copy(&mut self) {
        if let Some((start, end)) = self.get_selection() {
            let byte_start = self.char_to_byte_idx(start);
            let byte_end = self.char_to_byte_idx(end);
            self.clipboard = self.text[byte_start..byte_end].to_string();
            println!("Copied: {}", self.clipboard);
        }
    }
    
    pub fn cut(&mut self) {
        if self.has_selection() {
            self.copy();
            self.delete_selection();
        }
    }
    
    pub fn paste(&mut self) {
        if !self.clipboard.is_empty() && !self.disabled {
            if self.has_selection() {
                self.delete_selection();
            }
            
            for c in self.clipboard.chars() {
                let byte_pos = self.char_to_byte_idx(self.cursor_pos);
                self.text.insert(byte_pos, c);
                self.cursor_pos += 1;
            }
            println!("Pasted: {}", self.clipboard);
        }
    }
    
    // Get character index from mouse x position (for mouse selection)
    pub fn get_char_index_at_x(&self, mouse_x: f32, font_manager: &mut crate::core::FontManager) -> usize {
        if self.text.is_empty() {
            return 0;
        }
        
        let padding = self.size.padding_x();
        let font_size = self.size.font_size();
        let text_x = self.x + padding;
        let relative_x = mouse_x - text_x;
        
        if relative_x <= 0.0 {
            return 0;
        }
        
        let font = font_manager.create_font(&self.text, font_size, 400);
        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);
        
        let mut closest_idx = 0;
        let mut closest_dist = f32::MAX;
        
        for (char_idx, _) in self.text.char_indices() {
            let byte_idx = self.char_to_byte_idx(self.byte_to_char_idx(char_idx));
            let substr = &self.text[..byte_idx];
            let (width, _) = font.measure_str(substr, Some(&paint));
            let dist = (width - relative_x).abs();
            
            if dist < closest_dist {
                closest_dist = dist;
                closest_idx = self.byte_to_char_idx(char_idx);
            }
        }
        
        // Check end position
        let (full_width, _) = font.measure_str(&self.text, Some(&paint));
        let end_dist = (full_width - relative_x).abs();
        if end_dist < closest_dist {
            closest_idx = self.char_count();
        }
        
        closest_idx
    }
    
    pub fn start_selection(&mut self, char_idx: usize) {
        self.cursor_pos = char_idx;
        self.selection_start = Some(char_idx);
        self.selection_end = Some(char_idx);
    }
    
    pub fn update_selection(&mut self, char_idx: usize) {
        if self.selection_start.is_some() {
            self.selection_end = Some(char_idx);
            self.cursor_pos = char_idx;
        }
    }
}

impl Widget for Input {
    fn draw(&self, canvas: &Canvas, font_manager: &mut crate::core::FontManager) {
        let border_radius = Theme::RADIUS_MD;
        let padding = self.size.padding_x();
        let font_size = self.size.font_size();
        let colors = current_theme();

        // Background color
        let base_bg = colors.background;
        let current_bg = if self.disabled {
            with_alpha(base_bg, 128)
        } else {
            base_bg
        };

        // Draw background
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(current_bg);

        canvas.draw_round_rect(
            Rect::from_xywh(self.x, self.y, self.width, self.height),
            border_radius,
            border_radius,
            &paint,
        );

        // Border color with focus ring
        let border_color = if self.disabled {
            with_alpha(colors.input, 128)
        } else if self.focus_progress > 0.0 {
            lerp_color(colors.input, colors.ring, self.focus_progress)
        } else {
            colors.input
        };

        let mut border_paint = Paint::default();
        border_paint.set_anti_alias(true);
        border_paint.set_style(skia_safe::PaintStyle::Stroke);
        border_paint.set_color(border_color);
        border_paint.set_stroke_width(1.0);

        canvas.draw_round_rect(
            Rect::from_xywh(
                self.x + 0.5,
                self.y + 0.5,
                self.width - 1.0,
                self.height - 1.0,
            ),
            border_radius,
            border_radius,
            &border_paint,
        );

        // Focus ring (shadcn style)
        if self.focus_progress > 0.3 && !self.disabled {
            let ring_opacity = (self.focus_progress - 0.3) * 0.5;
            let mut ring_paint = Paint::default();
            ring_paint.set_anti_alias(true);
            ring_paint.set_style(skia_safe::PaintStyle::Stroke);
            ring_paint.set_color(with_alpha(colors.ring, (ring_opacity * 255.0) as u8));
            ring_paint.set_stroke_width(3.0);

            canvas.draw_round_rect(
                Rect::from_xywh(
                    self.x - 1.5,
                    self.y - 1.5,
                    self.width + 3.0,
                    self.height + 3.0,
                ),
                border_radius + 1.5,
                border_radius + 1.5,
                &ring_paint,
            );
        }

        // Text or placeholder
        let display_text = if self.text.is_empty() {
            self.placeholder
        } else {
            &self.text
        };
        
        let font_weight = 400;
        let font = font_manager.create_font(display_text, font_size, font_weight);

        let text_color = if self.disabled {
            with_alpha(colors.muted_foreground, 128)
        } else if self.text.is_empty() {
            colors.muted_foreground
        } else {
            colors.foreground
        };

        let mut text_paint = Paint::default();
        text_paint.set_anti_alias(true);
        text_paint.set_color(text_color);

        let text_x = self.x + padding;
        let text_y = self.y + self.height / 2.0 + (font_size * 0.3);

        // Draw selection highlight
        if self.has_selection() && !self.text.is_empty() {
            if let Some((start, end)) = self.get_selection() {
                let byte_start = self.char_to_byte_idx(start);
                let byte_end = self.char_to_byte_idx(end);
                
                let before_text = &self.text[..byte_start];
                let selected_text = &self.text[byte_start..byte_end];
                
                let (before_width, _) = font.measure_str(before_text, Some(&text_paint));
                let (selected_width, _) = font.measure_str(selected_text, Some(&text_paint));
                
                let selection_x = text_x + before_width;
                let selection_y = self.y + Theme::SPACE_2;
                let selection_height = self.height - (Theme::SPACE_2 * 2.0);
                
                // Draw selection background (shadcn style - primary color with opacity)
                let mut selection_paint = Paint::default();
                selection_paint.set_anti_alias(true);
                selection_paint.set_color(with_alpha(colors.primary, 80));
                
                canvas.draw_rect(
                    Rect::from_xywh(selection_x, selection_y, selected_width, selection_height),
                    &selection_paint,
                );
            }
        }

        canvas.draw_str(display_text, (text_x, text_y), &font, &text_paint);

        // Cursor
        if self.focused && self.cursor_visible && !self.disabled && !self.has_selection() {
            let cursor_x = if self.text.is_empty() {
                text_x
            } else {
                let byte_pos = self.char_to_byte_idx(self.cursor_pos.min(self.char_count()));
                let before_cursor = &self.text[..byte_pos];
                let (before_width, _) = font.measure_str(before_cursor, Some(&text_paint));
                text_x + before_width
            };

            let mut cursor_paint = Paint::default();
            cursor_paint.set_anti_alias(true);
            cursor_paint.set_color(colors.foreground);
            cursor_paint.set_stroke_width(1.5);

            let cursor_padding = Theme::SPACE_2;
            canvas.draw_line(
                (cursor_x, self.y + cursor_padding),
                (cursor_x, self.y + self.height - cursor_padding),
                &cursor_paint,
            );
        }
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    fn update_hover(&mut self, x: f32, y: f32) {
        self.hover = self.contains(x, y);
    }

    fn update_animation(&mut self, elapsed: f32) {
        let animation_speed = 0.15;

        // Hover animation
        let target_hover = if self.hover { 1.0 } else { 0.0 };
        if (self.hover_progress - target_hover).abs() > 0.01 {
            self.hover_progress += (target_hover - self.hover_progress) * animation_speed;
        } else {
            self.hover_progress = target_hover;
        }

        // Focus animation
        let target_focus = if self.focused { 1.0 } else { 0.0 };
        if (self.focus_progress - target_focus).abs() > 0.01 {
            self.focus_progress += (target_focus - self.focus_progress) * (animation_speed * 1.5);
        } else {
            self.focus_progress = target_focus;
        }

        // Cursor blink (faster when focused)
        self.cursor_timer = elapsed;
        let blink_speed = if self.focused { 2.5 } else { 2.0 };
        self.cursor_visible = (elapsed * blink_speed).sin() > 0.0;
    }

    fn on_click(&mut self) {
        if !self.disabled {
            self.focused = true;
            println!("Input focused");
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
