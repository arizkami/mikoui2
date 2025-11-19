use skia_safe::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
}

/// Shadcn/Radix inspired theme system
/// Based on MikoUI design tokens
pub struct Theme;

#[derive(Clone, Copy)]
pub struct ThemeColors {
    pub background: Color,
    pub foreground: Color,
    pub card: Color,
    pub card_foreground: Color,
    pub popover: Color,
    pub popover_foreground: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub muted: Color,
    pub muted_foreground: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub destructive: Color,
    pub destructive_foreground: Color,
    pub border: Color,
    pub input: Color,
    pub ring: Color,
}

impl ThemeColors {
    pub fn dark() -> Self {
        Self {
            background: Color::from_argb(255, 9, 9, 11), // zinc-950
            foreground: Color::from_argb(255, 250, 250, 250), // zinc-50
            card: Color::from_argb(255, 9, 9, 11), // zinc-950
            card_foreground: Color::from_argb(255, 250, 250, 250), // zinc-50
            popover: Color::from_argb(255, 9, 9, 11), // zinc-950
            popover_foreground: Color::from_argb(255, 250, 250, 250), // zinc-50
            primary: Color::from_argb(255, 250, 250, 250), // zinc-50
            primary_foreground: Color::from_argb(255, 24, 24, 27), // zinc-900
            secondary: Color::from_argb(255, 39, 39, 42), // zinc-800
            secondary_foreground: Color::from_argb(255, 250, 250, 250), // zinc-50
            muted: Color::from_argb(255, 39, 39, 42), // zinc-800
            muted_foreground: Color::from_argb(255, 161, 161, 170), // zinc-400
            accent: Color::from_argb(255, 39, 39, 42), // zinc-800
            accent_foreground: Color::from_argb(255, 250, 250, 250), // zinc-50
            destructive: Color::from_argb(255, 127, 29, 29), // red-900
            destructive_foreground: Color::from_argb(255, 250, 250, 250), // zinc-50
            border: Color::from_argb(255, 39, 39, 42), // zinc-800
            input: Color::from_argb(255, 39, 39, 42), // zinc-800
            ring: Color::from_argb(255, 212, 212, 216), // zinc-300
        }
    }
    
    pub fn light() -> Self {
        Self {
            background: Color::from_argb(255, 255, 255, 255), // white
            foreground: Color::from_argb(255, 9, 9, 11), // zinc-950
            card: Color::from_argb(255, 255, 255, 255), // white
            card_foreground: Color::from_argb(255, 9, 9, 11), // zinc-950
            popover: Color::from_argb(255, 255, 255, 255), // white
            popover_foreground: Color::from_argb(255, 9, 9, 11), // zinc-950
            primary: Color::from_argb(255, 24, 24, 27), // zinc-900
            primary_foreground: Color::from_argb(255, 250, 250, 250), // zinc-50
            secondary: Color::from_argb(255, 244, 244, 245), // zinc-100
            secondary_foreground: Color::from_argb(255, 24, 24, 27), // zinc-900
            muted: Color::from_argb(255, 244, 244, 245), // zinc-100
            muted_foreground: Color::from_argb(255, 113, 113, 122), // zinc-500
            accent: Color::from_argb(255, 244, 244, 245), // zinc-100
            accent_foreground: Color::from_argb(255, 24, 24, 27), // zinc-900
            destructive: Color::from_argb(255, 239, 68, 68), // red-500
            destructive_foreground: Color::from_argb(255, 250, 250, 250), // zinc-50
            border: Color::from_argb(255, 228, 228, 231), // zinc-200
            input: Color::from_argb(255, 228, 228, 231), // zinc-200
            ring: Color::from_argb(255, 24, 24, 27), // zinc-900
        }
    }
}

impl Theme {
    // Static colors for backward compatibility (dark mode)
    pub const BACKGROUND: Color = Color::from_argb(255, 9, 9, 11);
    pub const FOREGROUND: Color = Color::from_argb(255, 250, 250, 250);
    pub const CARD: Color = Color::from_argb(255, 9, 9, 11);
    pub const CARD_FOREGROUND: Color = Color::from_argb(255, 250, 250, 250);
    pub const POPOVER: Color = Color::from_argb(255, 9, 9, 11);
    pub const POPOVER_FOREGROUND: Color = Color::from_argb(255, 250, 250, 250);
    pub const PRIMARY: Color = Color::from_argb(255, 250, 250, 250);
    pub const PRIMARY_FOREGROUND: Color = Color::from_argb(255, 24, 24, 27);
    pub const SECONDARY: Color = Color::from_argb(255, 39, 39, 42);
    pub const SECONDARY_FOREGROUND: Color = Color::from_argb(255, 250, 250, 250);
    pub const MUTED: Color = Color::from_argb(255, 39, 39, 42);
    pub const MUTED_FOREGROUND: Color = Color::from_argb(255, 161, 161, 170);
    pub const ACCENT: Color = Color::from_argb(255, 39, 39, 42);
    pub const ACCENT_FOREGROUND: Color = Color::from_argb(255, 250, 250, 250);
    pub const DESTRUCTIVE: Color = Color::from_argb(255, 127, 29, 29);
    pub const DESTRUCTIVE_FOREGROUND: Color = Color::from_argb(255, 250, 250, 250);
    pub const BORDER: Color = Color::from_argb(255, 39, 39, 42);
    pub const INPUT: Color = Color::from_argb(255, 39, 39, 42);
    pub const RING: Color = Color::from_argb(255, 212, 212, 216);
    
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

/// Button variants matching MikoUI
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Variant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

/// Global theme state using thread-local storage
use std::cell::RefCell;

thread_local! {
    static CURRENT_THEME: RefCell<ThemeColors> = RefCell::new(ThemeColors::dark());
}

/// Set the global theme
pub fn set_theme(theme: ThemeColors) {
    CURRENT_THEME.with(|t| {
        *t.borrow_mut() = theme;
    });
}

/// Get a color from the current theme
pub fn get_theme_color<F, R>(f: F) -> R
where
    F: FnOnce(&ThemeColors) -> R,
{
    CURRENT_THEME.with(|t| f(&t.borrow()))
}

/// Convenience helper to copy the active theme palette
pub fn current_theme() -> ThemeColors {
    CURRENT_THEME.with(|t| *t.borrow())
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
