use skia_safe::Color;
use mikoui::ThemeColors;

/// Kiro theme - Modern, clean design with subtle accents
pub struct KiroTheme;

impl KiroTheme {
    pub fn dark() -> ThemeColors {
        ThemeColors {
            background: Color::from_argb(255, 18, 18, 18),        // #121212
            foreground: Color::from_argb(255, 230, 230, 230),     // #E6E6E6
            card: Color::from_argb(255, 24, 24, 24),              // #181818
            card_foreground: Color::from_argb(255, 230, 230, 230),
            popover: Color::from_argb(255, 28, 28, 28),
            popover_foreground: Color::from_argb(255, 230, 230, 230),
            primary: Color::from_argb(255, 99, 102, 241),         // Indigo-500
            primary_foreground: Color::from_argb(255, 255, 255, 255),
            secondary: Color::from_argb(255, 38, 38, 38),
            secondary_foreground: Color::from_argb(255, 230, 230, 230),
            muted: Color::from_argb(255, 38, 38, 38),
            muted_foreground: Color::from_argb(255, 163, 163, 163),
            accent: Color::from_argb(255, 99, 102, 241),
            accent_foreground: Color::from_argb(255, 255, 255, 255),
            destructive: Color::from_argb(255, 220, 38, 38),
            destructive_foreground: Color::from_argb(255, 255, 255, 255),
            border: Color::from_argb(255, 38, 38, 38),
            input: Color::from_argb(255, 38, 38, 38),
            ring: Color::from_argb(255, 99, 102, 241),
        }
    }

    pub fn light() -> ThemeColors {
        ThemeColors {
            background: Color::from_argb(255, 250, 250, 250),     // #FAFAFA
            foreground: Color::from_argb(255, 24, 24, 24),        // #181818
            card: Color::from_argb(255, 255, 255, 255),
            card_foreground: Color::from_argb(255, 24, 24, 24),
            popover: Color::from_argb(255, 255, 255, 255),
            popover_foreground: Color::from_argb(255, 24, 24, 24),
            primary: Color::from_argb(255, 79, 70, 229),          // Indigo-600
            primary_foreground: Color::from_argb(255, 255, 255, 255),
            secondary: Color::from_argb(255, 245, 245, 245),
            secondary_foreground: Color::from_argb(255, 24, 24, 24),
            muted: Color::from_argb(255, 245, 245, 245),
            muted_foreground: Color::from_argb(255, 115, 115, 115),
            accent: Color::from_argb(255, 79, 70, 229),
            accent_foreground: Color::from_argb(255, 255, 255, 255),
            destructive: Color::from_argb(255, 220, 38, 38),
            destructive_foreground: Color::from_argb(255, 255, 255, 255),
            border: Color::from_argb(255, 229, 229, 229),
            input: Color::from_argb(255, 229, 229, 229),
            ring: Color::from_argb(255, 79, 70, 229),
        }
    }
}
