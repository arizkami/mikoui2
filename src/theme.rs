use skia_safe::Color;

/// Shadcn/Radix inspired theme system
/// Based on shadcn/ui design tokens with dark mode as default
pub struct Theme;

impl Theme {
    // Background colors - Radix Gray scale
    pub const BACKGROUND: Color = Color::from_argb(255, 9, 9, 11); // zinc-950
    pub const FOREGROUND: Color = Color::from_argb(255, 250, 250, 250); // zinc-50
    
    // Card/Surface colors
    pub const CARD: Color = Color::from_argb(255, 9, 9, 11); // zinc-950
    pub const CARD_FOREGROUND: Color = Color::from_argb(255, 250, 250, 250); // zinc-50
    
    // Popover colors
    pub const POPOVER: Color = Color::from_argb(255, 9, 9, 11); // zinc-950
    pub const POPOVER_FOREGROUND: Color = Color::from_argb(255, 250, 250, 250); // zinc-50
    
    // Primary colors - Radix style
    pub const PRIMARY: Color = Color::from_argb(255, 250, 250, 250); // zinc-50
    pub const PRIMARY_FOREGROUND: Color = Color::from_argb(255, 24, 24, 27); // zinc-900
    
    // Secondary colors
    pub const SECONDARY: Color = Color::from_argb(255, 39, 39, 42); // zinc-800
    pub const SECONDARY_FOREGROUND: Color = Color::from_argb(255, 250, 250, 250); // zinc-50
    
    // Muted colors
    pub const MUTED: Color = Color::from_argb(255, 39, 39, 42); // zinc-800
    pub const MUTED_FOREGROUND: Color = Color::from_argb(255, 161, 161, 170); // zinc-400
    
    // Accent colors
    pub const ACCENT: Color = Color::from_argb(255, 39, 39, 42); // zinc-800
    pub const ACCENT_FOREGROUND: Color = Color::from_argb(255, 250, 250, 250); // zinc-50
    
    // Destructive colors
    pub const DESTRUCTIVE: Color = Color::from_argb(255, 127, 29, 29); // red-900
    pub const DESTRUCTIVE_FOREGROUND: Color = Color::from_argb(255, 250, 250, 250); // zinc-50
    
    // Border colors
    pub const BORDER: Color = Color::from_argb(255, 39, 39, 42); // zinc-800
    pub const INPUT: Color = Color::from_argb(255, 39, 39, 42); // zinc-800
    
    // Ring color (focus ring)
    pub const RING: Color = Color::from_argb(255, 212, 212, 216); // zinc-300
    
    // Semantic colors
    pub const SUCCESS: Color = Color::from_argb(255, 34, 197, 94); // green-500
    pub const WARNING: Color = Color::from_argb(255, 234, 179, 8); // yellow-500
    pub const ERROR: Color = Color::from_argb(255, 239, 68, 68); // red-500
    pub const INFO: Color = Color::from_argb(255, 59, 130, 246); // blue-500
    
    // Radius values (in pixels)
    pub const RADIUS_SM: f32 = 4.0;
    pub const RADIUS_MD: f32 = 6.0;
    pub const RADIUS_LG: f32 = 8.0;
    pub const RADIUS_XL: f32 = 12.0;
    
    // Spacing scale
    pub const SPACE_1: f32 = 4.0;
    pub const SPACE_2: f32 = 8.0;
    pub const SPACE_3: f32 = 12.0;
    pub const SPACE_4: f32 = 16.0;
    pub const SPACE_5: f32 = 20.0;
    pub const SPACE_6: f32 = 24.0;
    pub const SPACE_8: f32 = 32.0;
    
    // Typography scale
    pub const TEXT_XS: f32 = 12.0;
    pub const TEXT_SM: f32 = 14.0;
    pub const TEXT_BASE: f32 = 16.0;
    pub const TEXT_LG: f32 = 18.0;
    pub const TEXT_XL: f32 = 20.0;
    pub const TEXT_2XL: f32 = 24.0;
}

/// Component size variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Sm,
    Md,
    Lg,
}

impl Size {
    pub fn height(&self) -> f32 {
        match self {
            Size::Sm => 32.0,
            Size::Md => 40.0,
            Size::Lg => 48.0,
        }
    }
    
    pub fn padding_x(&self) -> f32 {
        match self {
            Size::Sm => Theme::SPACE_3,
            Size::Md => Theme::SPACE_4,
            Size::Lg => Theme::SPACE_5,
        }
    }
    
    pub fn font_size(&self) -> f32 {
        match self {
            Size::Sm => Theme::TEXT_SM,
            Size::Md => Theme::TEXT_BASE,
            Size::Lg => Theme::TEXT_LG,
        }
    }
}

/// Button variants matching shadcn/ui
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Variant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

/// Color interpolation utility
pub fn lerp_color(color1: Color, color2: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::from_argb(
        (color1.a() as f32 + (color2.a() as f32 - color1.a() as f32) * t) as u8,
        (color1.r() as f32 + (color2.r() as f32 - color1.r() as f32) * t) as u8,
        (color1.g() as f32 + (color2.g() as f32 - color1.g() as f32) * t) as u8,
        (color1.b() as f32 + (color2.b() as f32 - color1.b() as f32) * t) as u8,
    )
}

/// Create a color with adjusted alpha
pub fn with_alpha(color: Color, alpha: u8) -> Color {
    Color::from_argb(alpha, color.r(), color.g(), color.b())
}
