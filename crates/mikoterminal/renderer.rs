use crate::terminal::{Terminal, Cell};
use skia_safe::{Canvas, Color, Paint, Rect, Font, Typeface, FontStyle, FontMgr};

/// Terminal renderer
pub struct TerminalRenderer {
    font_size: f32,
    cell_width: f32,
    cell_height: f32,
    typeface: Option<Typeface>,
}

impl TerminalRenderer {
    pub fn new(font_size: f32) -> Self {
        // Try to load a monospace font using FontMgr
        let font_mgr = FontMgr::new();
        let typeface = font_mgr.match_family_style("Consolas", FontStyle::normal())
            .or_else(|| font_mgr.match_family_style("Courier New", FontStyle::normal()))
            .or_else(|| font_mgr.match_family_style("monospace", FontStyle::normal()))
            .or_else(|| font_mgr.match_family_style("Courier", FontStyle::normal()));
        
        // Calculate cell dimensions (approximate)
        let cell_width = font_size * 0.6;
        let cell_height = font_size * 1.2;
        
        Self {
            font_size,
            cell_width,
            cell_height,
            typeface,
        }
    }
    
    /// Render terminal to canvas
    pub fn render(&self, terminal: &Terminal, canvas: &Canvas, x: f32, y: f32) {
        let buffer = terminal.buffer();
        let (cursor_row, cursor_col) = terminal.cursor_position();
        
        // Create font
        let font = if let Some(ref typeface) = self.typeface {
            Font::from_typeface(typeface, self.font_size)
        } else {
            Font::default()
        };
        
        // Render each cell
        for (row_idx, row) in buffer.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                let cell_x = x + (col_idx as f32 * self.cell_width);
                let cell_y = y + (row_idx as f32 * self.cell_height);
                
                // Draw background
                let mut bg_paint = Paint::default();
                bg_paint.set_color(Color::from_rgb(
                    cell.bg_color.0,
                    cell.bg_color.1,
                    cell.bg_color.2,
                ));
                bg_paint.set_anti_alias(true);
                
                let cell_rect = Rect::from_xywh(
                    cell_x,
                    cell_y,
                    self.cell_width,
                    self.cell_height,
                );
                canvas.draw_rect(cell_rect, &bg_paint);
                
                // Draw character
                if cell.ch != ' ' {
                    let mut fg_paint = Paint::default();
                    fg_paint.set_color(Color::from_rgb(
                        cell.fg_color.0,
                        cell.fg_color.1,
                        cell.fg_color.2,
                    ));
                    fg_paint.set_anti_alias(true);
                    
                    let text_y = cell_y + self.cell_height - (self.cell_height - self.font_size) / 2.0;
                    canvas.draw_str(
                        &cell.ch.to_string(),
                        (cell_x, text_y),
                        &font,
                        &fg_paint,
                    );
                }
                
                // Draw cursor
                if row_idx == cursor_row && col_idx == cursor_col {
                    let mut cursor_paint = Paint::default();
                    cursor_paint.set_color(Color::from_rgb(255, 255, 255));
                    cursor_paint.set_style(skia_safe::PaintStyle::Stroke);
                    cursor_paint.set_stroke_width(2.0);
                    cursor_paint.set_anti_alias(true);
                    
                    canvas.draw_rect(cell_rect, &cursor_paint);
                }
            }
        }
    }
    
    /// Get cell dimensions
    pub fn cell_size(&self) -> (f32, f32) {
        (self.cell_width, self.cell_height)
    }
}
