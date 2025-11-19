// MikoTerminal - Terminal emulator for Rabital
// Inspired by Windows Terminal

pub mod terminal;
pub mod pty;
pub mod renderer;

pub use terminal::Terminal;
pub use pty::PtySession;
pub use renderer::TerminalRenderer;

/// Terminal configuration
#[derive(Debug, Clone)]
pub struct TerminalConfig {
    pub shell: String,
    pub font_size: f32,
    pub rows: u16,
    pub cols: u16,
    pub scrollback_limit: usize,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            shell: Self::default_shell(),
            font_size: 14.0,
            rows: 24,
            cols: 80,
            scrollback_limit: 10000,
        }
    }
}

impl TerminalConfig {
    /// Get the default shell for the current platform
    pub fn default_shell() -> String {
        #[cfg(target_os = "windows")]
        {
            "powershell.exe".to_string()
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            std::env::var("SHELL").unwrap_or_else(|_| "bash".to_string())
        }
    }
}
