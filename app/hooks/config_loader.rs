use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};

/// Configuration loader that auto-detects and parses .rabital config files
pub struct ConfigLoader {
    workspace_path: Option<PathBuf>,
    app_dir: PathBuf,
    settings: Option<EditorSettings>,
    tasks: Option<TasksConfig>,
    debug: Option<DebugConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    #[serde(default)]
    pub editor: EditorConfig,
    #[serde(default)]
    pub languages: std::collections::HashMap<String, LanguageConfig>,
    #[serde(default)]
    pub explorer: ExplorerConfig,
    #[serde(default)]
    pub terminal: TerminalConfig,
    #[serde(default)]
    pub git: GitConfig,
    #[serde(default)]
    pub search: SearchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default = "default_font_size")]
    pub font_size: u32,
    #[serde(default = "default_line_height")]
    pub line_height: f32,
    #[serde(default = "default_tab_size")]
    pub tab_size: u32,
    #[serde(default = "default_true")]
    pub insert_spaces: bool,
    #[serde(default)]
    pub auto_save: bool,
    #[serde(default = "default_auto_save_delay")]
    pub auto_save_delay: u32,
    #[serde(default)]
    pub word_wrap: bool,
    #[serde(default = "default_true")]
    pub show_line_numbers: bool,
    #[serde(default)]
    pub show_minimap: bool,
    #[serde(default = "default_true")]
    pub highlight_current_line: bool,
    #[serde(default)]
    pub format_on_save: bool,
    #[serde(default)]
    pub trim_trailing_whitespace: bool,
    #[serde(default)]
    pub insert_final_newline: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    #[serde(default = "default_tab_size")]
    pub tab_size: u32,
    #[serde(default)]
    pub format_on_save: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorerConfig {
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
    #[serde(default)]
    pub show_hidden_files: bool,
    #[serde(default = "default_true")]
    pub sort_folders_first: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    #[serde(default = "default_shell")]
    pub shell: String,
    #[serde(default = "default_terminal_font_size")]
    pub font_size: u32,
    #[serde(default = "default_true")]
    pub cursor_blink: bool,
    #[serde(default = "default_scrollback")]
    pub scrollback: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    #[serde(default)]
    pub auto_fetch: bool,
    #[serde(default)]
    pub show_inline_blame: bool,
    #[serde(default = "default_true")]
    pub show_gutter_indicators: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default)]
    pub whole_word: bool,
    #[serde(default)]
    pub use_regex: bool,
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasksConfig {
    pub version: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    #[serde(rename = "type")]
    pub task_type: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub group: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    pub version: String,
    pub configurations: Vec<DebugConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfiguration {
    pub name: String,
    #[serde(rename = "type")]
    pub debug_type: String,
    pub request: String,
    pub program: String,
    #[serde(default)]
    pub args: Vec<String>,
}

// Default value functions
fn default_theme() -> String { "dark".to_string() }
fn default_font_family() -> String { "Cascadia Code".to_string() }
fn default_font_size() -> u32 { 14 }
fn default_line_height() -> f32 { 1.5 }
fn default_tab_size() -> u32 { 4 }
fn default_true() -> bool { true }
fn default_auto_save_delay() -> u32 { 1000 }
fn default_shell() -> String { "powershell.exe".to_string() }
fn default_terminal_font_size() -> u32 { 13 }
fn default_scrollback() -> u32 { 10000 }

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            font_family: default_font_family(),
            font_size: default_font_size(),
            line_height: default_line_height(),
            tab_size: default_tab_size(),
            insert_spaces: true,
            auto_save: false,
            auto_save_delay: default_auto_save_delay(),
            word_wrap: false,
            show_line_numbers: true,
            show_minimap: false,
            highlight_current_line: true,
            format_on_save: false,
            trim_trailing_whitespace: false,
            insert_final_newline: false,
        }
    }
}

impl Default for ExplorerConfig {
    fn default() -> Self {
        Self {
            exclude_patterns: vec![
                "build/**".to_string(),
                "target/**".to_string(),
                "node_modules/**".to_string(),
                ".git/**".to_string(),
            ],
            show_hidden_files: false,
            sort_folders_first: true,
        }
    }
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            shell: default_shell(),
            font_size: default_terminal_font_size(),
            cursor_blink: true,
            scrollback: default_scrollback(),
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            auto_fetch: false,
            show_inline_blame: false,
            show_gutter_indicators: true,
        }
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            whole_word: false,
            use_regex: false,
            exclude_patterns: vec![
                "build/**".to_string(),
                "target/**".to_string(),
                "node_modules/**".to_string(),
            ],
        }
    }
}

impl ConfigLoader {
    /// Create a new config loader
    pub fn new() -> Self {
        let app_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));
        
        Self {
            workspace_path: None,
            app_dir,
            settings: None,
            tasks: None,
            debug: None,
        }
    }
    
    /// Set the workspace path and auto-load configs
    pub fn set_workspace(&mut self, path: PathBuf) {
        self.workspace_path = Some(path.clone());
        self.load_configs();
    }
    
    /// Get the shared directory path (global configs)
    pub fn get_shared_dir(&self) -> PathBuf {
        self.app_dir.join("shared")
    }
    
    /// Get the global themes directory
    pub fn get_themes_dir(&self) -> PathBuf {
        self.get_shared_dir().join("themes")
    }
    
    /// Get the global config directory
    pub fn get_config_dir(&self) -> PathBuf {
        self.get_shared_dir().join("config")
    }
    
    /// Load all configuration files
    fn load_configs(&mut self) {
        if let Some(ref workspace) = self.workspace_path {
            let rabital_dir = workspace.join(".rabital");
            
            if rabital_dir.exists() {
                println!("Found .rabital directory at: {}", rabital_dir.display());
                
                // Load settings.yml
                self.load_settings(&rabital_dir);
                
                // Load tasks.yml
                self.load_tasks(&rabital_dir);
                
                // Load debug.yml
                self.load_debug(&rabital_dir);
            } else {
                println!("No .rabital directory found, using defaults");
                self.load_global_settings();
            }
        }
    }
    
    /// Load settings from .rabital/settings.yml or global config
    fn load_settings(&mut self, rabital_dir: &Path) {
        let settings_path = rabital_dir.join("settings.yml");
        
        if settings_path.exists() {
            match fs::read_to_string(&settings_path) {
                Ok(content) => {
                    match serde_yaml::from_str::<EditorSettings>(&content) {
                        Ok(settings) => {
                            println!("Loaded settings from: {}", settings_path.display());
                            self.settings = Some(settings);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse settings.yml: {}", e);
                            self.load_global_settings();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read settings.yml: {}", e);
                    self.load_global_settings();
                }
            }
        } else {
            self.load_global_settings();
        }
    }
    
    /// Load global settings from shared/config/setting.yml
    fn load_global_settings(&mut self) {
        let global_settings_path = self.get_config_dir().join("setting.yml");
        
        if global_settings_path.exists() {
            match fs::read_to_string(&global_settings_path) {
                Ok(content) => {
                    match serde_yaml::from_str::<EditorSettings>(&content) {
                        Ok(settings) => {
                            println!("Loaded global settings from: {}", global_settings_path.display());
                            self.settings = Some(settings);
                            return;
                        }
                        Err(e) => {
                            eprintln!("Failed to parse global setting.yml: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read global setting.yml: {}", e);
                }
            }
        }
        
        // Use default settings
        self.settings = Some(EditorSettings {
            editor: EditorConfig::default(),
            languages: std::collections::HashMap::new(),
            explorer: ExplorerConfig::default(),
            terminal: TerminalConfig::default(),
            git: GitConfig::default(),
            search: SearchConfig::default(),
        });
    }
    
    /// Load tasks from .rabital/tasks.yml
    fn load_tasks(&mut self, rabital_dir: &Path) {
        let tasks_path = rabital_dir.join("tasks.yml");
        
        if tasks_path.exists() {
            match fs::read_to_string(&tasks_path) {
                Ok(content) => {
                    match serde_yaml::from_str::<TasksConfig>(&content) {
                        Ok(tasks) => {
                            println!("Loaded tasks from: {}", tasks_path.display());
                            self.tasks = Some(tasks);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse tasks.yml: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read tasks.yml: {}", e);
                }
            }
        }
    }
    
    /// Load debug config from .rabital/debug.yml
    fn load_debug(&mut self, rabital_dir: &Path) {
        let debug_path = rabital_dir.join("debug.yml");
        
        if debug_path.exists() {
            match fs::read_to_string(&debug_path) {
                Ok(content) => {
                    match serde_yaml::from_str::<DebugConfig>(&content) {
                        Ok(debug) => {
                            println!("Loaded debug config from: {}", debug_path.display());
                            self.debug = Some(debug);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse debug.yml: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read debug.yml: {}", e);
                }
            }
        }
    }
    
    /// Get the loaded settings
    pub fn get_settings(&self) -> Option<&EditorSettings> {
        self.settings.as_ref()
    }
    
    /// Get the loaded tasks
    pub fn get_tasks(&self) -> Option<&TasksConfig> {
        self.tasks.as_ref()
    }
    
    /// Get the loaded debug config
    pub fn get_debug(&self) -> Option<&DebugConfig> {
        self.debug.as_ref()
    }
    
    /// List available themes from shared/themes directory
    pub fn list_themes(&self) -> Vec<String> {
        let themes_dir = self.get_themes_dir();
        let mut themes = Vec::new();
        
        if let Ok(entries) = fs::read_dir(themes_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".yml") {
                        themes.push(name.trim_end_matches(".yml").to_string());
                    }
                }
            }
        }
        
        themes.sort();
        themes
    }
    
    /// Load a theme by name from shared/themes
    pub fn load_theme(&self, theme_name: &str) -> Option<String> {
        let theme_path = self.get_themes_dir().join(format!("{}.yml", theme_name));
        
        if theme_path.exists() {
            fs::read_to_string(&theme_path).ok()
        } else {
            None
        }
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}
