use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::io::{Read, Write};

/// Application state that persists between sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub workspace_path: Option<PathBuf>,
    pub window_width: u32,
    pub window_height: u32,
    pub window_x: i32,
    pub window_y: i32,
    pub window_maximized: bool,
    pub theme_mode: String,
    pub left_panel_visible: bool,
    pub left_panel_width: f32,
    pub right_panel_visible: bool,
    pub right_panel_width: f32,
    pub bottom_panel_visible: bool,
    pub bottom_panel_height: f32,
    pub expanded_folders: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            workspace_path: None,
            window_width: 1200,
            window_height: 800,
            window_x: 100,
            window_y: 100,
            window_maximized: false,
            theme_mode: "dark".to_string(),
            left_panel_visible: true,
            left_panel_width: 300.0,
            right_panel_visible: false,
            right_panel_width: 300.0,
            bottom_panel_visible: false,
            bottom_panel_height: 200.0,
            expanded_folders: Vec::new(),
        }
    }
}

impl AppState {
    /// Get the state file path
    fn state_file_path() -> PathBuf {
        // Save in the executable directory
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                return exe_dir.join("currentstate.rbx");
            }
        }
        PathBuf::from("currentstate.rbx")
    }
    
    /// Load state from file
    pub fn load() -> Self {
        let path = Self::state_file_path();
        
        if !path.exists() {
            println!("First run detected - creating new state file at {:?}", path);
            let default_state = Self::default();
            
            // Save the default state to create the file
            if let Err(e) = default_state.save() {
                eprintln!("Failed to create initial state file: {}", e);
            } else {
                println!("Created currentstate.rbx successfully");
            }
            
            return default_state;
        }
        
        match fs::File::open(&path) {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).is_ok() {
                    // Try to deserialize using bincode
                    match bincode::deserialize(&buffer) {
                        Ok(state) => {
                            println!("Loaded state from {:?}", path);
                            state
                        }
                        Err(e) => {
                            eprintln!("Failed to deserialize state: {}", e);
                            Self::default()
                        }
                    }
                } else {
                    Self::default()
                }
            }
            Err(e) => {
                eprintln!("Failed to open state file: {}", e);
                Self::default()
            }
        }
    }
    
    /// Save state to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::state_file_path();
        
        // Serialize using bincode
        let encoded = bincode::serialize(self)?;
        
        let mut file = fs::File::create(&path)?;
        file.write_all(&encoded)?;
        
        println!("Saved state to {:?}", path);
        Ok(())
    }
    
    /// Check if a folder is expanded
    pub fn is_folder_expanded(&self, path: &str) -> bool {
        self.expanded_folders.contains(&path.to_string())
    }
    
    /// Toggle folder expansion state
    pub fn toggle_folder(&mut self, path: &str) {
        let path_str = path.to_string();
        if let Some(pos) = self.expanded_folders.iter().position(|p| p == &path_str) {
            self.expanded_folders.remove(pos);
        } else {
            self.expanded_folders.push(path_str);
        }
    }
    
    /// Expand all folders
    pub fn expand_all_folders(&mut self, paths: Vec<String>) {
        self.expanded_folders = paths;
    }
    
    /// Collapse all folders
    pub fn collapse_all_folders(&mut self) {
        self.expanded_folders.clear();
    }
}
