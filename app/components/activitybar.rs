use mikoui::{CodiconIcons, Icon, IconSize, Widget, FontManager};
use skia_safe::{Canvas, Color, Paint, Rect};

const ACTIVITY_BAR_WIDTH: f32 = 48.0;
const ICON_SIZE: f32 = 24.0;
const ITEM_HEIGHT: f32 = 48.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivityBarItem {
    Explorer,
    Search,
    SourceControl,
    Debug,
    Extensions,
    Settings,
}

impl ActivityBarItem {
    pub fn icon(&self) -> &'static str {
        match self {
            ActivityBarItem::Explorer => CodiconIcons::FILES,
            ActivityBarItem::Search => CodiconIcons::SEARCH,
            ActivityBarItem::SourceControl => CodiconIcons::SOURCE_CONTROL,
            ActivityBarItem::Debug => CodiconIcons::DEBUG_ALT,
            ActivityBarItem::Extensions => CodiconIcons::EXTENSIONS,
            ActivityBarItem::Settings => CodiconIcons::SETTINGS_GEAR,
        }
    }
}

pub struct ActivityBar {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    items: Vec<ActivityBarItem>,
    active_item: Option<usize>,
    hover_item: Option<usize>,
    hover_progress: Vec<f32>,
}

impl ActivityBar {
    pub fn new(x: f32, y: f32, height: f32) -> Self {
        let items = vec![
            ActivityBarItem::Explorer,
            ActivityBarItem::Search,
            ActivityBarItem::SourceControl,
            ActivityBarItem::Debug,
            ActivityBarItem::Extensions,
        ];
        
        let hover_progress = vec![0.0; items.len()];
        
        Self {
            x,
            y,
            width: ACTIVITY_BAR_WIDTH,
            height,
            items,
            active_item: Some(0), // Explorer active by default
            hover_item: None,
            hover_progress,
        }
    }
    
    pub fn width(&self) -> f32 {
        self.width
    }
    
    fn item_rect(&self, index: usize) -> Rect {
        let y = self.y + (index as f32 * ITEM_HEIGHT);
        Rect::from_xywh(self.x, y, self.width, ITEM_HEIGHT)
    }
    
    pub fn get_active_item(&self) -> Option<ActivityBarItem> {
        self.active_item.and_then(|i| self.items.get(i).copied())
    }
}

impl Widget for ActivityBar {
    fn draw(&self, canvas: &Canvas, _font_manager: &mut FontManager) {
        let theme = mikoui::current_theme();
        
        // Background
        let mut bg_paint = Paint::default();
        bg_paint.set_anti_alias(true);
        bg_paint.set_color(theme.card);
        
        let rect = Rect::from_xywh(self.x, self.y, self.width, self.height);
        canvas.draw_rect(rect, &bg_paint);
        
        // Draw items
        for (i, item) in self.items.iter().enumerate() {
            let item_rect = self.item_rect(i);
            let is_active = self.active_item == Some(i);
            let is_hover = self.hover_item == Some(i);
            
            // Active indicator (left border)
            if is_active {
                let mut indicator_paint = Paint::default();
                indicator_paint.set_anti_alias(true);
                indicator_paint.set_color(theme.primary);
                
                let indicator_rect = Rect::from_xywh(
                    self.x,
                    item_rect.top,
                    2.0,
                    ITEM_HEIGHT,
                );
                canvas.draw_rect(indicator_rect, &indicator_paint);
            }
            
            // Hover background
            if is_hover || is_active {
                let alpha = if is_active {
                    (0.1 * 255.0) as u8
                } else {
                    (self.hover_progress[i] * 0.1 * 255.0) as u8
                };
                
                let mut hover_paint = Paint::default();
                hover_paint.set_anti_alias(true);
                let fg = theme.foreground;
                hover_paint.set_color(Color::from_argb(alpha, fg.r(), fg.g(), fg.b()));
                canvas.draw_rect(item_rect, &hover_paint);
            }
            
            // Icon - centered in the activity bar
            let icon_x = self.x + (self.width - ICON_SIZE) / 2.0;
            let icon_y = item_rect.top + (ITEM_HEIGHT - ICON_SIZE) / 2.0;
            
            let icon_color = if is_active {
                theme.foreground
            } else {
                theme.muted_foreground
            };
            
            let icon = Icon::new(
                icon_x,
                icon_y,
                item.icon(),
                IconSize::Medium,
                icon_color,
            );
            icon.draw(canvas, _font_manager);
        }
        
        // Right border
        let mut border_paint = Paint::default();
        border_paint.set_anti_alias(true);
        border_paint.set_color(theme.border);
        border_paint.set_stroke_width(1.0);
        
        canvas.draw_line(
            (self.x + self.width, self.y),
            (self.x + self.width, self.y + self.height),
            &border_paint,
        );
    }
    
    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
    
    fn update_hover(&mut self, x: f32, y: f32) {
        self.hover_item = None;
        
        if !self.contains(x, y) {
            return;
        }
        
        for i in 0..self.items.len() {
            let item_rect = self.item_rect(i);
            if x >= item_rect.left && x <= item_rect.right 
                && y >= item_rect.top && y <= item_rect.bottom {
                self.hover_item = Some(i);
                break;
            }
        }
    }
    
    fn update_animation(&mut self, _elapsed: f32) {
        let animation_speed = 0.2;
        
        for i in 0..self.hover_progress.len() {
            let target = if self.hover_item == Some(i) { 1.0 } else { 0.0 };
            if (self.hover_progress[i] - target).abs() > 0.01 {
                self.hover_progress[i] += (target - self.hover_progress[i]) * animation_speed;
            } else {
                self.hover_progress[i] = target;
            }
        }
    }
    
    fn on_click(&mut self) {
        if let Some(hover) = self.hover_item {
            self.active_item = Some(hover);
            println!("Activity bar item clicked: {:?}", self.items[hover]);
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
