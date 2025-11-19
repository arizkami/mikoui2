use skia_safe::Color;
use mikoui::ThemeColors;

/// Xcode theme - Apple's development environment style
pub struct XcodeTheme;

impl XcodeTheme {
    pub fn dark() -> ThemeColors {
        ThemeColors {
            background: Color::from_argb(255, 41, 42, 45),        // #292A2D
            foreground: Color::from_argb(255, 220, 220, 220),     // #DCDCDC
            card: Color::from_argb(255, 50, 51, 54),              // #323336
            card_foreground: Color::from_argb(255, 220, 220, 220),
            popover: Color::from_argb(255, 50, 51, 54),
            popover_foreground: Color::from_argb(255, 220, 220, 220),
            primary: Color::from_argb(255, 10, 132, 255),         // #0A84FF
            primary_foreground: Color::from_argb(255, 255, 255, 255),
            secondary: Color::from_argb(255, 58, 59, 62),
            secondary_foreground: Color::from_argb(255, 220, 220, 220),
            muted: Color::from_argb(255, 58, 59, 62),
            muted_foreground: Color::from_argb(255, 152, 152, 157),
            accent: Color::from_argb(255, 10, 132, 255),
            accent_foreground: Color::from_argb(255, 255, 255, 255),
            destructive: Color::from_argb(255, 255, 69, 58),
            destructive_foreground: Color::from_argb(255, 255, 255, 255),
            border: Color::from_argb(255, 58, 59, 62),
            input: Color::from_argb(255, 58, 59, 62),
            ring: Color::from_argb(255, 10, 132, 255),
        }
    }

    pub fn light() -> ThemeColors {
        ThemeColors {
            background: Color::from_argb(255, 255, 255, 255),     // #FFFFFF
            foreground: Color::from_argb(255, 0, 0, 0),           // #000000
            card: Color::from_argb(255, 247, 247, 247),           // #F7F7F7
            card_foreground: Color::from_argb(255, 0, 0, 0),
            popover: Color::from_argb(255, 247, 247, 247),
            popover_foreground: Color::from_argb(255, 0, 0, 0),
            primary: Color::from_argb(255, 0, 122, 255),          // #007AFF
            primary_foreground: Color::from_argb(255, 255, 255, 255),
            secondary: Color::from_argb(255, 242, 242, 247),
            secondary_foreground: Color::from_argb(255, 0, 0, 0),
            muted: Color::from_argb(255, 242, 242, 247),
            muted_foreground: Color::from_argb(255, 142, 142, 147),
            accent: Color::from_argb(255, 0, 122, 255),
            accent_foreground: Color::from_argb(255, 255, 255, 255),
            destructive: Color::from_argb(255, 255, 59, 48),
            destructive_foreground: Color::from_argb(255, 255, 255, 255),
            border: Color::from_argb(255, 229, 229, 234),
            input: Color::from_argb(255, 229, 229, 234),
            ring: Color::from_argb(255, 0, 122, 255),
        }
    }
}
