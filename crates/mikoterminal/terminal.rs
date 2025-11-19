use crate::{PtySession, TerminalConfig};
use std::collections::VecDeque;

/// Terminal cell
#[derive(Debug, Clone)]
pub struct Cell {
    pub ch: char,
    pub fg_color: (u8, u8, u8),
    pub bg_color: (u8, u8, u8),
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg_color: (255, 255, 255),
            bg_color: (0, 0, 0),
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

/// Terminal buffer
pub struct Terminal {
    config: TerminalConfig,
    pty: Option<PtySession>,
    buffer: Vec<Vec<Cell>>,
    scrollback: VecDeque<Vec<Cell>>,
    cursor_row: usize,
    cursor_col: usize,
    scroll_offset: usize,
}

impl Terminal {
    /// Create a new terminal
    pub fn new(config: TerminalConfig) -> Self {
        let rows = config.rows as usize;
        let cols = config.cols as usize;
        
        let buffer = vec![vec![Cell::default(); cols]; rows];
        
        Self {
            config,
            pty: None,
            buffer,
            scrollback: VecDeque::new(),
            cursor_row: 0,
            cursor_col: 0,
            scroll_offset: 0,
        }
    }
    
    /// Start the terminal with PTY
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let pty = PtySession::new(
            &self.config.shell,
            self.config.rows,
            self.config.cols,
        )?;
        
        self.pty = Some(pty);
        
        // Show welcome message
        let welcome = format!(
            "Rabital Terminal\r\nShell: {}\r\n\r\nTerminal integration coming soon...\r\n\r\n",
            self.config.shell
        );
        self.process_output(welcome.as_bytes());
        
        Ok(())
    }
    
    /// Update terminal - read from PTY and update buffer
    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref pty) = self.pty {
            let data = pty.read()?;
            if !data.is_empty() {
                self.process_output(&data);
            }
        }
        Ok(())
    }
    
    /// Process output from PTY
    fn process_output(&mut self, data: &[u8]) {
        // Simple text processing (no ANSI escape codes for now)
        let text = String::from_utf8_lossy(data);
        
        for ch in text.chars() {
            match ch {
                '\n' => {
                    self.cursor_col = 0;
                    self.cursor_row += 1;
                    if self.cursor_row >= self.buffer.len() {
                        self.scroll_up();
                    }
                }
                '\r' => {
                    self.cursor_col = 0;
                }
                '\t' => {
                    // Tab = 4 spaces
                    for _ in 0..4 {
                        self.put_char(' ');
                    }
                }
                ch if ch.is_control() => {
                    // Ignore other control characters for now
                }
                ch => {
                    self.put_char(ch);
                }
            }
        }
    }
    
    /// Put a character at cursor position
    fn put_char(&mut self, ch: char) {
        if self.cursor_row < self.buffer.len() && self.cursor_col < self.buffer[0].len() {
            self.buffer[self.cursor_row][self.cursor_col].ch = ch;
            self.cursor_col += 1;
            
            if self.cursor_col >= self.buffer[0].len() {
                self.cursor_col = 0;
                self.cursor_row += 1;
                if self.cursor_row >= self.buffer.len() {
                    self.scroll_up();
                }
            }
        }
    }
    
    /// Scroll buffer up by one line
    fn scroll_up(&mut self) {
        if let Some(first_line) = self.buffer.first().cloned() {
            self.scrollback.push_back(first_line);
            
            // Limit scrollback
            while self.scrollback.len() > self.config.scrollback_limit {
                self.scrollback.pop_front();
            }
        }
        
        // Shift buffer up
        self.buffer.remove(0);
        self.buffer.push(vec![Cell::default(); self.buffer[0].len()]);
        self.cursor_row = self.buffer.len() - 1;
    }
    
    /// Send input to terminal
    pub fn send_input(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut pty) = self.pty {
            pty.write(text.as_bytes())?;
        }
        Ok(())
    }
    
    /// Get terminal buffer
    pub fn buffer(&self) -> &[Vec<Cell>] {
        &self.buffer
    }
    
    /// Get cursor position
    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_row, self.cursor_col)
    }
    
    /// Resize terminal
    pub fn resize(&mut self, rows: u16, cols: u16) -> Result<(), Box<dyn std::error::Error>> {
        self.config.rows = rows;
        self.config.cols = cols;
        
        // Resize buffer
        self.buffer.resize(rows as usize, vec![Cell::default(); cols as usize]);
        for row in &mut self.buffer {
            row.resize(cols as usize, Cell::default());
        }
        
        // Resize PTY
        if let Some(ref mut pty) = self.pty {
            pty.resize(rows, cols)?;
        }
        
        Ok(())
    }
}
