use skia_safe::Color;
use mikoui::ThemeColors;

/// VSCode theme - Familiar editor colors
pub struct VSCodeTheme;

impl VSCodeTheme {
    pub fn dark() -> ThemeColors {
        ThemeColors {
            background: Color::from_argb(255, 30, 30, 30),        // #1E1E1E
            foreground: Color::from_argb(255, 212, 212, 212),     // #D4D4D4
            card: Color::from_argb(255, 37, 37, 38),              // #252526
            card_foreground: Color::from_argb(255, 212, 212, 212),
            popover: Color::from_argb(255, 37, 37, 38),
            popover_foreground: Color::from_argb(255, 212, 212, 212),
            primary: Color::from_argb(255, 14, 99, 156),          // #0E639C
            primary_foreground: Color::from_argb(255, 255, 255, 255),
            secondary: Color::from_argb(255, 51, 51, 51),
            secondary_foreground: Color::from_argb(255, 212, 212, 212),
            muted: Color::from_argb(255, 51, 51, 51),
            muted_foreground: Color::from_argb(255, 150, 150, 150),
            accent: Color::from_argb(255, 0, 122, 204),           // #007ACC
            accent_foreground: Color::from_argb(255, 255, 255, 255),
            destructive: Color::from_argb(255, 244, 71, 71),
            destructive_foreground: Color::from_argb(255, 255, 255, 255),
            border: Color::from_argb(255, 51, 51, 51),
            input: Color::from_argb(255, 51, 51, 51),
            ring: Color::from_argb(255, 0, 122, 204),
        }
    }

    pub fn light() -> ThemeColors {
        ThemeColors {
            background: Color::from_argb(255, 255, 255, 255),     // #FFFFFF
            foreground: Color::from_argb(255, 51, 51, 51),        // #333333
            card: Color::from_argb(255, 246, 246, 246),           // #F6F6F6
            card_foreground: Color::from_argb(255, 51, 51, 51),
            popover: Color::from_argb(255, 246, 246, 246),
            popover_foreground: Color::from_argb(255, 51, 51, 51),
            primary: Color::from_argb(255, 0, 122, 204),          // #007ACC
            primary_foreground: Color::from_argb(255, 255, 255, 255),
            secondary: Color::from_argb(255, 240, 240, 240),
            secondary_foreground: Color::from_argb(255, 51, 51, 51),
            muted: Color::from_argb(255, 240, 240, 240),
            muted_foreground: Color::from_argb(255, 108, 108, 108),
            accent: Color::from_argb(255, 0, 122, 204),
            accent_foreground: Color::from_argb(255, 255, 255, 255),
            destructive: Color::from_argb(255, 205, 49, 49),
            destructive_foreground: Color::from_argb(255, 255, 255, 255),
            border: Color::from_argb(255, 229, 229, 229),
            input: Color::from_argb(255, 229, 229, 229),
            ring: Color::from_argb(255, 0, 122, 204),
        }
    }
}
