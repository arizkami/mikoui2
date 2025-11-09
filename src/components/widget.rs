use skia_safe::{Canvas, Font};

/// Base trait for all UI widgets
pub trait Widget {
    /// Draw the widget on the canvas
    fn draw(&self, canvas: &Canvas, font_factory: &dyn Fn(f32, i32) -> Font);
    
    /// Check if a point is inside the widget bounds
    fn contains(&self, x: f32, y: f32) -> bool;
    
    /// Update hover state based on mouse position
    fn update_hover(&mut self, x: f32, y: f32);
    
    /// Update animations based on elapsed time
    fn update_animation(&mut self, elapsed: f32);
    
    /// Handle click events
    fn on_click(&mut self);
    
    /// Downcast to Any for type checking
    fn as_any(&self) -> &dyn std::any::Any;
    
    /// Downcast to Any for mutable access
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
