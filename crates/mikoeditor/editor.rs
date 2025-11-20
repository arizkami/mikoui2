use crate::tab::{EditorTab, TabManager};
use crate::tabbar::TabBar;
use crate::syntax::TokenType;
use skia_safe::{Canvas, Color, Font, Paint, Rect};

pub struct Editor {
    tab_manager: TabManager,
    tab_bar: TabBar,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    line_height: f32,
    gutter_width: f32,
    cursor_blink_time: f32,
    show_cursor: bool,
}

impl Editor {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let tab_bar = TabBar::new(x, y, width);
        
        Self {
            tab_manager: TabManager::new(),
            tab_bar,
            x,
            y,
            width,
            height,
            line_height: 22.0,
            gutter_width: 60.0,
            cursor_blink_time: 0.0,
            show_cursor: true,
        }
    }
    
    pub fn tab_manager(&self) -> &TabManager {
        &self.tab_manager
    }
    
    pub fn tab_manager_mut(&mut self) -> &mut TabManager {
        &mut self.tab_manager
    }
    
    pub fn open_file(&mut self, path: std::path::PathBuf) -> std::io::Result<()> {
        self.tab_manager.add_tab_from_file(path)?;
        Ok(())
    }
    
    pub fn new_tab(&mut self) {
        self.tab_manager.add_tab();
    }
    
    pub fn close_active_tab(&mut self) {
        self.tab_manager.close_active_tab();
    }
    
    pub fn next_tab(&mut self) {
        self.tab_manager.next_tab();
    }
    
    pub fn previous_tab(&mut self) {
        self.tab_manager.previous_tab();
    }
    
    pub fn draw(&self, canvas: &Canvas, ui_font: &Font, mono_font: &Font) {
        // Draw tab bar with UI font
        let tab_bar_height = self.tab_bar.height();
        self.tab_bar.draw(canvas, ui_font, &self.tab_manager);
        
        // Editor content area (below tab bar)
        let content_y = self.y + tab_bar_height;
        let content_height = self.height - tab_bar_height;
        
        // Background
        let mut bg_paint = Paint::default();
        bg_paint.set_color(Color::from_rgb(30, 30, 30));
        bg_paint.set_anti_alias(true);
        canvas.draw_rect(
            Rect::from_xywh(self.x, content_y, self.width, content_height),
            &bg_paint,
        );
        
        // Get active tab
        if let Some(tab) = self.tab_manager.get_active_tab() {
            // Gutter background
            let mut gutter_paint = Paint::default();
            gutter_paint.set_color(Color::from_rgb(40, 40, 40));
            gutter_paint.set_anti_alias(true);
            canvas.draw_rect(
                Rect::from_xywh(self.x, content_y, self.gutter_width, content_height),
                &gutter_paint,
            );
            
            // Draw line numbers and text
            let visible_lines = (content_height / self.line_height).ceil() as usize;
            let start_line = (tab.scroll_offset / self.line_height) as usize;
            let end_line = (start_line + visible_lines).min(tab.buffer.len_lines());
            
            // Get syntax highlights
            let highlights = tab.highlighter.get_highlights(&tab.buffer.to_string());
            
            for line_idx in start_line..end_line {
                let y_pos = content_y + (line_idx as f32 * self.line_height) - tab.scroll_offset + 17.0;
                
                // Current line highlight
                if line_idx == tab.cursor_line {
                    let mut current_line_paint = Paint::default();
                    current_line_paint.set_color(Color::from_argb(20, 255, 255, 255));
                    current_line_paint.set_anti_alias(true);
                    canvas.draw_rect(
                        Rect::from_xywh(self.x, y_pos - 15.0, self.width, self.line_height),
                        &current_line_paint,
                    );
                }
                
                // Line number
                let line_num = format!("{}", line_idx + 1);
                let line_num_width = mono_font.measure_str(&line_num, None).0;
                let line_num_x = self.x + self.gutter_width - line_num_width - 15.0;
                
                let mut line_num_paint = Paint::default();
                line_num_paint.set_color(if line_idx == tab.cursor_line {
                    Color::from_rgb(200, 200, 200)
                } else {
                    Color::from_rgb(100, 100, 100)
                });
                line_num_paint.set_anti_alias(true);
                canvas.draw_str(&line_num, (line_num_x, y_pos), mono_font, &line_num_paint);
                
                // Line text with syntax highlighting
                if let Some(mut line_text) = tab.buffer.line(line_idx) {
                    // Remove trailing newline characters to prevent rendering issues
                    line_text = line_text.trim_end_matches('\n').trim_end_matches('\r').to_string();
                    
                    let text_x = self.x + self.gutter_width + 10.0;
                    
                    // Calculate line start byte offset
                    let mut line_start_byte = 0;
                    for i in 0..line_idx {
                        if let Some(l) = tab.buffer.line(i) {
                            line_start_byte += l.as_bytes().len();
                        }
                    }
                    let line_end_byte = line_start_byte + line_text.as_bytes().len();
                    
                    // Draw text with syntax highlighting
                    let mut current_x = text_x;
                    let mut last_pos = 0;
                    
                    for (start, end, token_type) in &highlights {
                        // Check if this highlight is in the current line
                        if *end <= line_start_byte || *start >= line_end_byte {
                            continue;
                        }
                        
                        let highlight_start = (*start).saturating_sub(line_start_byte);
                        let highlight_end = (*end - line_start_byte).min(line_text.len());
                        
                        // Draw text before highlight
                        if last_pos < highlight_start {
                            let text_before = &line_text[last_pos..highlight_start];
                            let mut text_paint = Paint::default();
                            text_paint.set_color(Color::from_rgb(220, 220, 220));
                            text_paint.set_anti_alias(true);
                            canvas.draw_str(text_before, (current_x, y_pos), mono_font, &text_paint);
                            current_x += mono_font.measure_str(text_before, None).0;
                        }
                        
                        // Draw highlighted text
                        if highlight_start < highlight_end && highlight_end <= line_text.len() {
                            let highlighted_text = &line_text[highlight_start..highlight_end];
                            let mut highlight_paint = Paint::default();
                            highlight_paint.set_color(self.get_token_color(*token_type));
                            highlight_paint.set_anti_alias(true);
                            canvas.draw_str(highlighted_text, (current_x, y_pos), mono_font, &highlight_paint);
                            current_x += mono_font.measure_str(highlighted_text, None).0;
                            last_pos = highlight_end;
                        }
                    }
                    
                    // Draw remaining text
                    if last_pos < line_text.len() {
                        let remaining_text = &line_text[last_pos..];
                        let mut text_paint = Paint::default();
                        text_paint.set_color(Color::from_rgb(220, 220, 220));
                        text_paint.set_anti_alias(true);
                        canvas.draw_str(remaining_text, (current_x, y_pos), mono_font, &text_paint);
                    }
                }
            }
            
            // Draw cursor with blink
            if self.show_cursor && tab.cursor_line >= start_line && tab.cursor_line < end_line {
                let cursor_y = content_y + (tab.cursor_line as f32 * self.line_height) - tab.scroll_offset + 2.0;
                
                // Calculate cursor X position based on actual text width
                let mut cursor_x = self.x + self.gutter_width + 10.0;
                if let Some(line) = tab.buffer.line(tab.cursor_line) {
                    let line_char_count = line.chars().count();
                    if tab.cursor_column > 0 && tab.cursor_column <= line_char_count {
                        // Get text before cursor by character count, not byte index
                        let text_before_cursor: String = line.chars().take(tab.cursor_column).collect();
                        cursor_x += mono_font.measure_str(&text_before_cursor, None).0;
                    }
                }
                
                let mut cursor_paint = Paint::default();
                cursor_paint.set_color(Color::from_rgb(255, 255, 255));
                cursor_paint.set_anti_alias(true);
                canvas.draw_rect(
                    Rect::from_xywh(cursor_x, cursor_y, 2.0, self.line_height - 4.0),
                    &cursor_paint,
                );
            }
            
            // Draw status bar at the bottom
            let status_bar_height = 24.0;
            let status_bar_y = content_y + content_height - status_bar_height;
            
            // Status bar background
            let mut status_bg = Paint::default();
            status_bg.set_color(Color::from_rgb(0, 122, 204));
            status_bg.set_anti_alias(true);
            canvas.draw_rect(
                Rect::from_xywh(self.x, status_bar_y, self.width, status_bar_height),
                &status_bg,
            );
            
            // Language indicator
            let lang_text = tab.get_language_display();
            let mut status_paint = Paint::default();
            status_paint.set_color(Color::from_rgb(255, 255, 255));
            status_paint.set_anti_alias(true);
            canvas.draw_str(
                &lang_text,
                (self.x + 10.0, status_bar_y + 16.0),
                ui_font,
                &status_paint,
            );
            
            // Cursor position
            let cursor_info = format!("Ln {}, Col {}", tab.cursor_line + 1, tab.cursor_column + 1);
            let cursor_info_width = ui_font.measure_str(&cursor_info, None).0;
            canvas.draw_str(
                &cursor_info,
                (self.x + self.width - cursor_info_width - 10.0, status_bar_y + 16.0),
                ui_font,
                &status_paint,
            );
        }
    }
    
    pub fn set_bounds(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
        self.tab_bar.set_bounds(x, y, width);
    }
    
    pub fn update_hover(&mut self, x: f32, y: f32) {
        self.tab_bar.update_hover(x, y, &self.tab_manager);
    }
    
    pub fn update_animation(&mut self, elapsed: f32) {
        self.tab_bar.update_animation(self.tab_manager.tab_count());
        
        // Cursor blink animation
        self.cursor_blink_time += elapsed;
        if self.cursor_blink_time >= 1.0 {
            self.cursor_blink_time = 0.0;
        }
        self.show_cursor = self.cursor_blink_time < 0.5;
    }
    
    pub fn insert_char(&mut self, c: char) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            // Calculate character index from cursor position (using char count, not bytes)
            let mut char_idx = 0;
            for line_idx in 0..tab.cursor_line {
                if let Some(line) = tab.buffer.line(line_idx) {
                    char_idx += line.chars().count();  // Count characters, not bytes
                }
            }
            char_idx += tab.cursor_column;
            
            tab.buffer.insert(char_idx, &c.to_string());
            tab.cursor_column += 1;
            
            // Re-parse for syntax highlighting
            tab.highlighter.parse(&tab.buffer.to_string());
            
            // Reset cursor blink
            self.cursor_blink_time = 0.0;
            self.show_cursor = true;
        }
    }
    
    pub fn delete_char(&mut self) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            if tab.cursor_column > 0 {
                // Calculate character index from cursor position (using char count, not bytes)
                let mut char_idx = 0;
                for line_idx in 0..tab.cursor_line {
                    if let Some(line) = tab.buffer.line(line_idx) {
                        char_idx += line.chars().count();  // Count characters, not bytes
                    }
                }
                
                // Find the actual character position to delete
                if let Some(current_line) = tab.buffer.line(tab.cursor_line) {
                    // Get the character at cursor_column - 1
                    let chars_before: Vec<char> = current_line.chars().take(tab.cursor_column).collect();
                    if !chars_before.is_empty() {
                        char_idx += chars_before.len() - 1;
                        
                        tab.buffer.remove(char_idx, char_idx + 1);
                        tab.cursor_column -= 1;
                        
                        // Re-parse for syntax highlighting
                        tab.highlighter.parse(&tab.buffer.to_string());
                        
                        // Reset cursor blink
                        self.cursor_blink_time = 0.0;
                        self.show_cursor = true;
                    }
                }
            } else if tab.cursor_line > 0 {
                // Delete newline - merge with previous line
                let prev_line_len = tab.buffer.line(tab.cursor_line - 1)
                    .map(|l| l.chars().count())  // Count characters, not bytes
                    .unwrap_or(0);
                
                let mut char_idx = 0;
                for line_idx in 0..tab.cursor_line {
                    if let Some(line) = tab.buffer.line(line_idx) {
                        char_idx += line.chars().count();  // Count characters, not bytes
                    }
                }
                
                if char_idx > 0 {
                    tab.buffer.remove(char_idx - 1, char_idx);
                    tab.cursor_line -= 1;
                    tab.cursor_column = prev_line_len;
                    
                    // Re-parse for syntax highlighting
                    tab.highlighter.parse(&tab.buffer.to_string());
                    
                    // Reset cursor blink
                    self.cursor_blink_time = 0.0;
                    self.show_cursor = true;
                }
            }
        }
    }
    
    pub fn insert_newline(&mut self) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            let mut char_idx = 0;
            for line_idx in 0..tab.cursor_line {
                if let Some(line) = tab.buffer.line(line_idx) {
                    char_idx += line.len();
                }
            }
            char_idx += tab.cursor_column;
            
            tab.buffer.insert(char_idx, "\n");
            tab.cursor_line += 1;
            tab.cursor_column = 0;
            
            // Re-parse for syntax highlighting
            tab.highlighter.parse(&tab.buffer.to_string());
            
            // Reset cursor blink
            self.cursor_blink_time = 0.0;
            self.show_cursor = true;
        }
    }
    
    pub fn move_cursor_left(&mut self) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            if tab.cursor_column > 0 {
                tab.cursor_column -= 1;
            } else if tab.cursor_line > 0 {
                tab.cursor_line -= 1;
                if let Some(line) = tab.buffer.line(tab.cursor_line) {
                    tab.cursor_column = line.chars().count();  // Count characters, not bytes
                }
            }
            self.cursor_blink_time = 0.0;
            self.show_cursor = true;
        }
    }
    
    pub fn move_cursor_right(&mut self) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            if let Some(line) = tab.buffer.line(tab.cursor_line) {
                let line_len = line.chars().count();  // Count characters, not bytes
                if tab.cursor_column < line_len {
                    tab.cursor_column += 1;
                } else if tab.cursor_line < tab.buffer.len_lines() - 1 {
                    tab.cursor_line += 1;
                    tab.cursor_column = 0;
                }
            }
            self.cursor_blink_time = 0.0;
            self.show_cursor = true;
        }
    }
    
    pub fn move_cursor_up(&mut self) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            if tab.cursor_line > 0 {
                tab.cursor_line -= 1;
                if let Some(line) = tab.buffer.line(tab.cursor_line) {
                    let line_len = line.chars().count();  // Count characters, not bytes
                    tab.cursor_column = tab.cursor_column.min(line_len);
                }
            }
            self.cursor_blink_time = 0.0;
            self.show_cursor = true;
        }
    }
    
    pub fn move_cursor_down(&mut self) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            if tab.cursor_line < tab.buffer.len_lines() - 1 {
                tab.cursor_line += 1;
                if let Some(line) = tab.buffer.line(tab.cursor_line) {
                    let line_len = line.chars().count();  // Count characters, not bytes
                    tab.cursor_column = tab.cursor_column.min(line_len);
                }
            }
            self.cursor_blink_time = 0.0;
            self.show_cursor = true;
        }
    }
    
    pub fn handle_click(&mut self, x: f32, y: f32) -> bool {
        // Check if clicking on close button
        if let Some(tab_index) = self.tab_bar.get_close_button_clicked(x, y, &self.tab_manager) {
            self.tab_manager.close_tab(tab_index);
            return true;
        }
        
        // Check if clicking on tab
        if let Some(tab_index) = self.tab_bar.get_clicked_tab(x, y, &self.tab_manager) {
            self.tab_manager.set_active_tab(tab_index);
            return true;
        }
        
        false
    }
    
    pub fn scroll(&mut self, delta: f32) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            tab.scroll_offset = (tab.scroll_offset + delta).max(0.0);
            let content_height = self.height - self.tab_bar.height();
            let max_scroll = (tab.buffer.len_lines() as f32 * self.line_height) - content_height;
            tab.scroll_offset = tab.scroll_offset.min(max_scroll.max(0.0));
        }
    }
    
    fn get_token_color(&self, token_type: TokenType) -> Color {
        match token_type {
            TokenType::Keyword => Color::from_rgb(197, 134, 192),      // Purple
            TokenType::Function => Color::from_rgb(220, 220, 170),     // Yellow
            TokenType::Type => Color::from_rgb(78, 201, 176),          // Cyan
            TokenType::String => Color::from_rgb(206, 145, 120),       // Orange
            TokenType::Number => Color::from_rgb(181, 206, 168),       // Light green
            TokenType::Comment => Color::from_rgb(106, 153, 85),       // Green
            TokenType::Operator => Color::from_rgb(180, 180, 180),     // Light gray
            TokenType::Punctuation => Color::from_rgb(180, 180, 180),  // Light gray
            TokenType::Variable => Color::from_rgb(156, 220, 254),     // Light blue
            TokenType::Property => Color::from_rgb(156, 220, 254),     // Light blue
            TokenType::Parameter => Color::from_rgb(156, 220, 254),    // Light blue
            TokenType::Constant => Color::from_rgb(79, 193, 255),      // Blue
            TokenType::Text => Color::from_rgb(220, 220, 220),         // White
        }
    }
    
    pub fn select_all(&mut self) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            tab.selection_start = Some((0, 0));
            let last_line = tab.buffer.len_lines().saturating_sub(1);
            let last_col = tab.buffer.line(last_line)
                .map(|l| l.chars().count())
                .unwrap_or(0);
            tab.selection_end = Some((last_line, last_col));
            tab.cursor_line = last_line;
            tab.cursor_column = last_col;
        }
    }
    
    pub fn get_selected_text(&self) -> Option<String> {
        if let Some(tab) = self.tab_manager.get_active_tab() {
            if let (Some(start), Some(end)) = (tab.selection_start, tab.selection_end) {
                let (start_line, start_col) = start;
                let (end_line, end_col) = end;
                
                let mut text = String::new();
                for line_idx in start_line..=end_line {
                    if let Some(line) = tab.buffer.line(line_idx) {
                        let line_chars: Vec<char> = line.chars().collect();
                        
                        if line_idx == start_line && line_idx == end_line {
                            // Single line selection
                            let start = start_col.min(line_chars.len());
                            let end = end_col.min(line_chars.len());
                            text.push_str(&line_chars[start..end].iter().collect::<String>());
                        } else if line_idx == start_line {
                            // First line
                            let start = start_col.min(line_chars.len());
                            text.push_str(&line_chars[start..].iter().collect::<String>());
                        } else if line_idx == end_line {
                            // Last line
                            let end = end_col.min(line_chars.len());
                            text.push_str(&line_chars[..end].iter().collect::<String>());
                        } else {
                            // Middle lines
                            text.push_str(&line);
                        }
                    }
                }
                return Some(text);
            }
        }
        None
    }
    
    pub fn delete_selection(&mut self) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            if let (Some(start), Some(end)) = (tab.selection_start, tab.selection_end) {
                let (start_line, start_col) = start;
                let (end_line, end_col) = end;
                
                // Calculate character indices
                let mut start_idx = 0;
                for line_idx in 0..start_line {
                    if let Some(line) = tab.buffer.line(line_idx) {
                        start_idx += line.chars().count();
                    }
                }
                start_idx += start_col;
                
                let mut end_idx = 0;
                for line_idx in 0..end_line {
                    if let Some(line) = tab.buffer.line(line_idx) {
                        end_idx += line.chars().count();
                    }
                }
                end_idx += end_col;
                
                // Delete the range
                tab.buffer.remove(start_idx, end_idx);
                
                // Update cursor position
                tab.cursor_line = start_line;
                tab.cursor_column = start_col;
                
                // Clear selection
                tab.selection_start = None;
                tab.selection_end = None;
                
                // Re-parse for syntax highlighting
                tab.highlighter.parse(&tab.buffer.to_string());
            }
        }
    }
    
    pub fn insert_text(&mut self, text: &str) {
        // Delete selection if any
        self.delete_selection();
        
        // Insert text character by character
        for c in text.chars() {
            if c == '\n' || c == '\r' {
                self.insert_newline();
            } else if c == '\t' {
                self.insert_char(' ');
                self.insert_char(' ');
                self.insert_char(' ');
                self.insert_char(' ');
            } else if !c.is_control() {
                self.insert_char(c);
            }
        }
    }
}
