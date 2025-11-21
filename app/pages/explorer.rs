use mikoui::{Widget, FontManager};
use mikoui::theme::current_theme;
use mikoui::components::{Icon, IconSize, CodiconIcons};
use skia_safe::{Canvas, Color, Paint, Rect};
use std::path::{Path, PathBuf};
use std::fs;

/// File tree item
#[derive(Debug, Clone)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub depth: usize,
    pub children: Vec<FileItem>,
}

impl FileItem {
    pub fn new(path: PathBuf, depth: usize) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        
        let is_dir = path.is_dir();
        
        Self {
            name,
            path,
            is_dir,
            is_expanded: false,
            depth,
            children: Vec::new(),
        }
    }
    
    pub fn load_children(&mut self) {
        if !self.is_dir || !self.children.is_empty() {
            return;
        }
        
        if let Ok(entries) = fs::read_dir(&self.path) {
            let mut items: Vec<FileItem> = entries
                .filter_map(|e| e.ok())
                .map(|e| FileItem::new(e.path(), self.depth + 1))
                .collect();
            
            // Sort: directories first, then alphabetically
            items.sort_by(|a, b| {
                match (a.is_dir, b.is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                }
            });
            
            self.children = items;
        }
    }
}

/// File Explorer
pub struct Explorer {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    root_path: PathBuf,
    items: Vec<FileItem>,
    scroll_offset: f32,
    hover_index: Option<usize>,
    expanded_paths: Vec<String>,
    // Scrollbar state
    scrollbar_width: f32,
    scrollbar_hover: bool,
    scrollbar_dragging: bool,
    drag_start_y: f32,
    drag_start_offset: f32,
}

impl Explorer {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            root_path: PathBuf::new(),
            items: Vec::new(),
            scroll_offset: 0.0,
            hover_index: None,
            expanded_paths: Vec::new(),
            scrollbar_width: 8.0,
            scrollbar_hover: false,
            scrollbar_dragging: false,
            drag_start_y: 0.0,
            drag_start_offset: 0.0,
        }
    }
    
    pub fn new_with_path(x: f32, y: f32, width: f32, height: f32, root_path: PathBuf) -> Self {
        println!("Explorer::new_with_path called with: {}", root_path.display());
        println!("Path exists: {}", root_path.exists());
        println!("Path is_dir: {}", root_path.is_dir());
        
        let mut explorer = Self {
            x,
            y,
            width,
            height,
            root_path: root_path.clone(),
            items: Vec::new(),
            scroll_offset: 0.0,
            hover_index: None,
            expanded_paths: Vec::new(),
            scrollbar_width: 8.0,
            scrollbar_hover: false,
            scrollbar_dragging: false,
            drag_start_y: 0.0,
            drag_start_offset: 0.0,
        };
        
        explorer.load_root();
        println!("Explorer created with {} items", explorer.items.len());
        explorer
    }
    
    pub fn set_root_path(&mut self, path: PathBuf) {
        self.root_path = path;
        self.items.clear();
        self.expanded_paths.clear();
        self.load_root();
    }
    
    pub fn has_root(&self) -> bool {
        !self.root_path.as_os_str().is_empty()
    }
    
    pub fn get_root_name(&self) -> String {
        if let Some(folder_name) = self.root_path.file_name() {
            folder_name.to_string_lossy().to_string()
        } else {
            self.root_path.to_string_lossy().to_string()
        }
    }
    
    /// Expand all folders
    pub fn expand_all(&mut self) {
        self.expanded_paths.clear();
        Self::expand_all_recursive(&mut self.items, &mut self.expanded_paths);
    }
    
    fn expand_all_recursive(items: &mut [FileItem], expanded_paths: &mut Vec<String>) {
        for item in items {
            if item.is_dir {
                item.is_expanded = true;
                if item.children.is_empty() {
                    item.load_children();
                }
                expanded_paths.push(item.path.to_string_lossy().to_string());
                Self::expand_all_recursive(&mut item.children, expanded_paths);
            }
        }
    }
    
    /// Collapse all folders
    pub fn collapse_all(&mut self) {
        self.expanded_paths.clear();
        Self::collapse_all_recursive(&mut self.items);
    }
    
    fn collapse_all_recursive(items: &mut [FileItem]) {
        for item in items {
            if item.is_dir {
                item.is_expanded = false;
                Self::collapse_all_recursive(&mut item.children);
            }
        }
    }
    
    /// Get list of expanded folder paths
    pub fn get_expanded_paths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        Self::collect_expanded_paths(&self.items, &mut paths);
        paths
    }
    
    fn collect_expanded_paths(items: &[FileItem], paths: &mut Vec<String>) {
        for item in items {
            if item.is_dir && item.is_expanded {
                paths.push(item.path.to_string_lossy().to_string());
                Self::collect_expanded_paths(&item.children, paths);
            }
        }
    }
    
    /// Restore expanded state from paths
    pub fn restore_expanded_state(&mut self, paths: &[String]) {
        self.expanded_paths = paths.to_vec();
        Self::restore_expanded_recursive(&mut self.items, paths);
    }
    
    fn restore_expanded_recursive(items: &mut [FileItem], paths: &[String]) {
        for item in items {
            if item.is_dir {
                let path_str = item.path.to_string_lossy().to_string();
                if paths.contains(&path_str) {
                    item.is_expanded = true;
                    if item.children.is_empty() {
                        item.load_children();
                    }
                    Self::restore_expanded_recursive(&mut item.children, paths);
                }
            }
        }
    }
    
    fn load_root(&mut self) {
        if !self.has_root() {
            println!("Explorer: No root path set");
            return;
        }
        
        println!("Explorer: Loading root from: {}", self.root_path.display());
        
        if !self.root_path.exists() {
            eprintln!("Explorer: Root path does not exist: {}", self.root_path.display());
            return;
        }
        
        // Load root directory contents directly without showing the root folder itself
        if let Ok(entries) = fs::read_dir(&self.root_path) {
            let mut items: Vec<FileItem> = entries
                .filter_map(|e| e.ok())
                .map(|e| FileItem::new(e.path(), 0)) // Start at depth 0
                .collect();
            
            // Sort: directories first, then alphabetically
            items.sort_by(|a, b| {
                match (a.is_dir, b.is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                }
            });
            
            self.items = items;
            println!("Explorer: Loaded {} items", self.items.len());
        } else {
            eprintln!("Explorer: Failed to read directory: {}", self.root_path.display());
        }
    }
    
    fn get_visible_items(&self) -> Vec<&FileItem> {
        let mut visible = Vec::new();
        
        fn collect_visible<'a>(item: &'a FileItem, visible: &mut Vec<&'a FileItem>) {
            visible.push(item);
            if item.is_expanded {
                for child in &item.children {
                    collect_visible(child, visible);
                }
            }
        }
        
        // Collect all root-level items and their expanded children
        for item in &self.items {
            collect_visible(item, &mut visible);
        }
        
        visible
    }
    
    fn get_item_icon(&self, item: &FileItem) -> &'static str {
        if item.is_dir {
            if item.is_expanded {
                CodiconIcons::FOLDER_OPENED
            } else {
                CodiconIcons::FOLDER
            }
        } else {
            // Determine icon based on file extension
            if let Some(ext) = item.path.extension().and_then(|e| e.to_str()) {
                match ext {
                    "rs" => CodiconIcons::FILE_CODE,
                    "toml" | "yml" | "yaml" | "json" => CodiconIcons::SETTINGS_GEAR,
                    "md" => CodiconIcons::BOOK,
                    "txt" => CodiconIcons::FILE_TEXT,
                    "png" | "jpg" | "jpeg" | "gif" | "svg" => CodiconIcons::FILE_MEDIA,
                    _ => CodiconIcons::FILE,
                }
            } else {
                CodiconIcons::FILE
            }
        }
    }
    
    pub fn toggle_item(&mut self, index: usize) {
        let visible = self.get_visible_items();
        if index >= visible.len() {
            return;
        }
        
        // Find and toggle the item at the given visible index
        let mut current_index = 0;
        for item in &mut self.items {
            if Self::toggle_at_index(item, index, &mut current_index) {
                return;
            }
        }
    }
    
    fn toggle_at_index(item: &mut FileItem, target_index: usize, current_index: &mut usize) -> bool {
        // Check if this is the target item
        if *current_index == target_index {
            item.is_expanded = !item.is_expanded;
            if item.is_expanded && item.children.is_empty() {
                item.load_children();
            }
            return true;
        }
        
        *current_index += 1;
        
        // If this item is expanded, check its children
        if item.is_expanded {
            for child in &mut item.children {
                if Self::toggle_at_index(child, target_index, current_index) {
                    return true;
                }
            }
        }
        
        false
    }
    
    pub fn set_bounds(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }
    
    fn get_scrollbar_rect(&self) -> Rect {
        let item_height = 28.0;
        let visible_items = self.get_visible_items();
        let total_height = visible_items.len() as f32 * item_height;
        
        if total_height <= self.height {
            return Rect::from_xywh(0.0, 0.0, 0.0, 0.0); // No scrollbar needed
        }
        
        let scrollbar_height = (self.height / total_height * self.height).max(30.0);
        let max_scroll = total_height - self.height;
        let scroll_ratio = if max_scroll > 0.0 {
            self.scroll_offset / max_scroll
        } else {
            0.0
        };
        let scrollbar_y = self.y + (self.height - scrollbar_height) * scroll_ratio;
        
        Rect::from_xywh(
            self.x + self.width - self.scrollbar_width - 2.0,
            scrollbar_y,
            self.scrollbar_width,
            scrollbar_height,
        )
    }
    
    pub fn is_over_scrollbar(&self, x: f32, y: f32) -> bool {
        let rect = self.get_scrollbar_rect();
        rect.width() > 0.0 && x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom
    }
    
    pub fn start_scrollbar_drag(&mut self, y: f32) {
        self.scrollbar_dragging = true;
        self.drag_start_y = y;
        self.drag_start_offset = self.scroll_offset;
    }
    
    pub fn stop_scrollbar_drag(&mut self) {
        self.scrollbar_dragging = false;
    }
    
    pub fn handle_scrollbar_drag(&mut self, y: f32) {
        if !self.scrollbar_dragging {
            return;
        }
        
        let item_height = 28.0;
        let visible_items = self.get_visible_items();
        let total_height = visible_items.len() as f32 * item_height;
        let max_scroll = (total_height - self.height).max(0.0);
        
        if max_scroll <= 0.0 {
            return;
        }
        
        let delta_y = y - self.drag_start_y;
        let scroll_ratio = delta_y / self.height;
        let delta_scroll = scroll_ratio * total_height;
        
        self.scroll_offset = (self.drag_start_offset + delta_scroll).clamp(0.0, max_scroll);
    }
    
    pub fn is_scrollbar_dragging(&self) -> bool {
        self.scrollbar_dragging
    }
    
    pub fn scroll(&mut self, delta: f32) {
        let item_height = 28.0;
        let visible_items = self.get_visible_items();
        let total_height = visible_items.len() as f32 * item_height;
        let max_scroll = (total_height - self.height).max(0.0);
        
        self.scroll_offset = (self.scroll_offset + delta).clamp(0.0, max_scroll);
    }
}

impl Widget for Explorer {
    fn draw(&self, canvas: &Canvas, font_manager: &mut FontManager) {
        let theme = current_theme();
        
        // Show welcome message if no folder is open
        if !self.has_root() {
            let welcome_text = "No folder opened";
            let font = font_manager.create_font(welcome_text, 14.0, 400);
            let mut text_paint = Paint::default();
            text_paint.set_color(theme.muted_foreground);
            text_paint.set_anti_alias(true);
            
            canvas.draw_str(
                welcome_text,
                (self.x + 16.0, self.y + 40.0),
                &font,
                &text_paint,
            );
            return;
        }
        
        let item_height = 28.0;
        let indent_size = 16.0;
        
        let visible_items = self.get_visible_items();
        
        for (i, item) in visible_items.iter().enumerate() {
            let y = self.y + (i as f32 * item_height) - self.scroll_offset;
            
            // Skip if not visible
            if y + item_height < self.y || y > self.y + self.height {
                continue;
            }
            
            let x = self.x + (item.depth as f32 * indent_size);
            
            // Hover background
            if self.hover_index == Some(i) {
                let mut hover_paint = Paint::default();
                hover_paint.set_color(theme.muted);
                hover_paint.set_anti_alias(true);
                
                let hover_rect = Rect::from_xywh(
                    self.x,
                    y,
                    self.width,
                    item_height,
                );
                canvas.draw_rect(hover_rect, &hover_paint);
            }
            
            // Chevron for directories
            if item.is_dir {
                let chevron_icon = if item.is_expanded {
                    CodiconIcons::CHEVRON_DOWN
                } else {
                    CodiconIcons::CHEVRON_RIGHT
                };
                
                let chevron = Icon::new(
                    x + 2.0,
                    y + 6.0,
                    chevron_icon,
                    IconSize::Small,
                    theme.muted_foreground,
                );
                chevron.draw(canvas, font_manager);
            }
            
            // File/folder icon
            let icon_x = x + if item.is_dir { 18.0 } else { 4.0 };
            let file_icon = Icon::new(
                icon_x,
                y + 6.0,
                self.get_item_icon(item),
                IconSize::Small,
                theme.foreground,
            );
            file_icon.draw(canvas, font_manager);
            
            // File name
            let text_x = icon_x + 20.0;
            let font = font_manager.create_font(&item.name, 13.0, 400);
            let mut text_paint = Paint::default();
            text_paint.set_color(theme.foreground);
            text_paint.set_anti_alias(true);
            
            canvas.draw_str(
                &item.name,
                (text_x, y + 18.0),
                &font,
                &text_paint,
            );
        }
        
        // Draw scrollbar if needed
        let scrollbar_rect = self.get_scrollbar_rect();
        if scrollbar_rect.width() > 0.0 {
            let mut scrollbar_paint = Paint::default();
            let alpha = if self.scrollbar_dragging {
                180
            } else if self.scrollbar_hover {
                120
            } else {
                80
            };
            scrollbar_paint.set_color(Color::from_argb(alpha, 200, 200, 200));
            scrollbar_paint.set_anti_alias(true);
            
            canvas.draw_round_rect(
                scrollbar_rect,
                4.0,
                4.0,
                &scrollbar_paint,
            );
        }
    }
    
    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
    
    fn update_hover(&mut self, x: f32, y: f32) {
        if !self.contains(x, y) {
            self.hover_index = None;
            self.scrollbar_hover = false;
            return;
        }
        
        // Check if hovering over scrollbar
        self.scrollbar_hover = self.is_over_scrollbar(x, y);
        
        if self.scrollbar_hover {
            self.hover_index = None;
            return;
        }
        
        let item_height = 28.0;
        let relative_y = y - self.y + self.scroll_offset;
        let index = (relative_y / item_height) as usize;
        
        let visible_count = self.get_visible_items().len();
        if index < visible_count {
            self.hover_index = Some(index);
        } else {
            self.hover_index = None;
        }
    }
    
    fn update_animation(&mut self, _elapsed: f32) {
        // No animation for now
    }
    
    fn on_click(&mut self) {
        // Don't handle clicks if on scrollbar
        if self.scrollbar_hover {
            return;
        }
        
        if let Some(index) = self.hover_index {
            self.toggle_item(index);
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
