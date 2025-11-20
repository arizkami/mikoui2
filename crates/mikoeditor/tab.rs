use crate::buffer::TextBuffer;
use crate::syntax::SyntaxHighlighter;
use std::path::PathBuf;

/// Represents a single editor tab
pub struct EditorTab {
    pub id: usize,
    pub buffer: TextBuffer,
    pub highlighter: SyntaxHighlighter,
    pub scroll_offset: f32,
    pub cursor_line: usize,
    pub cursor_column: usize,
    pub title: String,
    pub selection_start: Option<(usize, usize)>, // (line, column)
    pub selection_end: Option<(usize, usize)>,   // (line, column)
}

impl EditorTab {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            buffer: TextBuffer::new(),
            highlighter: SyntaxHighlighter::new(),
            scroll_offset: 0.0,
            cursor_line: 0,
            cursor_column: 0,
            title: "Untitled".to_string(),
            selection_start: None,
            selection_end: None,
        }
    }
    
    pub fn from_file(id: usize, path: PathBuf) -> std::io::Result<Self> {
        let buffer = TextBuffer::from_file(path.clone())?;
        let mut highlighter = SyntaxHighlighter::new();
        
        // Set up syntax highlighting
        if let Some(lang) = buffer.language() {
            let _ = highlighter.set_language(lang);
            highlighter.parse(&buffer.to_string());
        }
        
        let title = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();
        
        Ok(Self {
            id,
            buffer,
            highlighter,
            scroll_offset: 0.0,
            cursor_line: 0,
            cursor_column: 0,
            title,
            selection_start: None,
            selection_end: None,
        })
    }
    
    pub fn from_text(id: usize, text: &str, title: String) -> Self {
        let buffer = TextBuffer::from_str(text);
        let mut highlighter = SyntaxHighlighter::new();
        highlighter.parse(text);
        
        Self {
            id,
            buffer,
            highlighter,
            scroll_offset: 0.0,
            cursor_line: 0,
            cursor_column: 0,
            title,
            selection_start: None,
            selection_end: None,
        }
    }
    
    pub fn is_modified(&self) -> bool {
        self.buffer.is_modified()
    }
    
    pub fn get_display_title(&self) -> String {
        if self.is_modified() {
            format!("● {}", self.title)
        } else {
            self.title.clone()
        }
    }
    
    pub fn get_language_display(&self) -> String {
        self.buffer.language()
            .map(|lang| match lang {
                "rust" => "Rust",
                "javascript" => "JavaScript",
                "typescript" => "TypeScript",
                "tsx" => "TSX",
                "python" => "Python",
                "json" => "JSON",
                "html" => "HTML",
                "css" => "CSS",
                "markdown" => "Markdown",
                "yaml" => "YAML",
                "toml" => "TOML",
                "c" => "C",
                "cpp" => "C++",
                "java" => "Java",
                "go" => "Go",
                "ruby" => "Ruby",
                "php" => "PHP",
                "bash" => "Shell",
                "sql" => "SQL",
                _ => "Text",
            })
            .unwrap_or("Text")
            .to_string()
    }
}

/// Manages multiple editor tabs
pub struct TabManager {
    tabs: Vec<EditorTab>,
    active_tab: usize,
    next_id: usize,
}

impl TabManager {
    pub fn new() -> Self {
        let mut manager = Self {
            tabs: Vec::new(),
            active_tab: 0,
            next_id: 0,
        };
        
        // Create initial welcome tab
        manager.add_tab_with_text(
            "// Welcome to Rabital Editor\n// Press Ctrl+N to create a new file\n// Press Ctrl+O to open a file\n\nfn main() {\n    println!(\"Hello, world!\");\n}\n",
            "Welcome".to_string(),
        );
        
        manager
    }
    
    pub fn add_tab(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        let tab = EditorTab::new(id);
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
        
        id
    }
    
    pub fn add_tab_with_text(&mut self, text: &str, title: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        let tab = EditorTab::from_text(id, text, title);
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
        
        id
    }
    
    pub fn add_tab_from_file(&mut self, path: PathBuf) -> std::io::Result<usize> {
        let id = self.next_id;
        self.next_id += 1;
        
        let tab = EditorTab::from_file(id, path)?;
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
        
        Ok(id)
    }
    
    pub fn close_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.tabs.remove(index);
            
            // Adjust active tab
            if self.tabs.is_empty() {
                // Create a new empty tab if all tabs are closed
                self.add_tab();
            } else if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            }
            
            true
        } else {
            false
        }
    }
    
    pub fn close_active_tab(&mut self) {
        self.close_tab(self.active_tab);
    }
    
    pub fn set_active_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
        }
    }
    
    pub fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
        }
    }
    
    pub fn previous_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = if self.active_tab == 0 {
                self.tabs.len() - 1
            } else {
                self.active_tab - 1
            };
        }
    }
    
    pub fn get_active_tab(&self) -> Option<&EditorTab> {
        self.tabs.get(self.active_tab)
    }
    
    pub fn get_active_tab_mut(&mut self) -> Option<&mut EditorTab> {
        self.tabs.get_mut(self.active_tab)
    }
    
    pub fn get_tab(&self, index: usize) -> Option<&EditorTab> {
        self.tabs.get(index)
    }
    
    pub fn get_tab_mut(&mut self, index: usize) -> Option<&mut EditorTab> {
        self.tabs.get_mut(index)
    }
    
    pub fn tabs(&self) -> &[EditorTab] {
        &self.tabs
    }
    
    pub fn active_index(&self) -> usize {
        self.active_tab
    }
    
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}
