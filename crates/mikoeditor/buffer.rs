use ropey::Rope;
use std::path::PathBuf;

/// Text buffer using Rope for efficient text manipulation
pub struct TextBuffer {
    rope: Rope,
    file_path: Option<PathBuf>,
    modified: bool,
    language: Option<String>,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            file_path: None,
            modified: false,
            language: None,
        }
    }
    
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            file_path: None,
            modified: false,
            language: None,
        }
    }
    
    pub fn from_file(path: PathBuf) -> std::io::Result<Self> {
        let text = std::fs::read_to_string(&path)?;
        let language = Self::detect_language(&path);
        
        Ok(Self {
            rope: Rope::from_str(&text),
            file_path: Some(path),
            modified: false,
            language,
        })
    }
    
    fn detect_language(path: &PathBuf) -> Option<String> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext {
                // Rust
                "rs" => "rust",
                
                // JavaScript/TypeScript
                "js" | "mjs" | "cjs" => "javascript",
                "jsx" => "javascript",
                "ts" | "mts" | "cts" => "typescript",
                "tsx" => "tsx",
                
                // Python
                "py" | "pyw" | "pyi" => "python",
                
                // Web
                "html" | "htm" => "html",
                "css" | "scss" | "sass" | "less" => "css",
                
                // Data formats
                "json" | "jsonc" => "json",
                "xml" => "xml",
                "yaml" | "yml" => "yaml",
                "toml" => "toml",
                
                // Markup
                "md" | "markdown" => "markdown",
                
                // C/C++
                "c" | "h" => "c",
                "cpp" | "cc" | "cxx" | "hpp" | "hxx" => "cpp",
                
                // Java/Kotlin
                "java" => "java",
                "kt" | "kts" => "kotlin",
                
                // Go
                "go" => "go",
                
                // Ruby
                "rb" => "ruby",
                
                // PHP
                "php" => "php",
                
                // Shell
                "sh" | "bash" | "zsh" => "bash",
                
                // Other
                "sql" => "sql",
                "lua" => "lua",
                "vim" => "vim",
                "txt" => "text",
                
                _ => "text",
            })
            .map(String::from)
    }
    
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }
    
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }
    
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx < self.len_lines() {
            Some(self.rope.line(line_idx).to_string())
        } else {
            None
        }
    }
    
    pub fn insert(&mut self, char_idx: usize, text: &str) {
        self.rope.insert(char_idx, text);
        self.modified = true;
    }
    
    pub fn remove(&mut self, start: usize, end: usize) {
        self.rope.remove(start..end);
        self.modified = true;
    }
    
    pub fn is_modified(&self) -> bool {
        self.modified
    }
    
    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }
    
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }
    
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }
    
    pub fn save(&mut self) -> std::io::Result<()> {
        if let Some(ref path) = self.file_path {
            std::fs::write(path, self.to_string())?;
            self.modified = false;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file path set",
            ))
        }
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}
