use skia_safe::Color;

pub struct ZedTheme;

impl ZedTheme {
    // Background colors (GitHub Dark inspired)
    pub const BG: Color = Color::from_argb(255, 17, 19, 25); // #111319
    pub const SURFACE: Color = Color::from_argb(255, 25, 29, 35); // #191d23
    pub const ELEVATED: Color = Color::from_argb(255, 32, 37, 44); // #20252c

    // Primary colors (Zed blue)
    pub const PRIMARY: Color = Color::from_argb(255, 60, 120, 249); // #3c78f9
    pub const PRIMARY_HOVER: Color = Color::from_argb(255, 75, 135, 255); // #4b87ff
    pub const PRIMARY_ACTIVE: Color = Color::from_argb(255, 45, 105, 234); // #2d69ea

    // Text colors
    pub const TEXT: Color = Color::from_argb(255, 235, 239, 242); // #ebeff2
    pub const TEXT_DIM: Color = Color::from_argb(255, 165, 175, 187); // #a5afbb
    pub const TEXT_MUTED: Color = Color::from_argb(255, 120, 133, 150); // #788596

    // Border colors
    pub const BORDER: Color = Color::from_argb(255, 50, 58, 68); // #323a44
    pub const BORDER_FOCUS: Color = Color::from_argb(153, 60, 120, 249); // #3c78f9 with alpha

    // Input colors
    pub const INPUT_BG: Color = Color::from_argb(255, 32, 37, 44); // #20252c
    pub const INPUT_HOVER: Color = Color::from_argb(255, 40, 46, 54); // #282e36
    pub const INPUT_FOCUS: Color = Color::from_argb(255, 37, 43, 51); // #252b33

    // Accent colors (for future use)
    #[allow(dead_code)]
    pub const SUCCESS: Color = Color::from_argb(255, 60, 187, 120); // #3cbb78
    #[allow(dead_code)]
    pub const WARNING: Color = Color::from_argb(255, 249, 187, 60); // #f9bb3c
    #[allow(dead_code)]
    pub const ERROR: Color = Color::from_argb(255, 249, 60, 60); // #f93c3c
}

pub fn lerp_color(color1: Color, color2: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::from_argb(
        (color1.a() as f32 + (color2.a() as f32 - color1.a() as f32) * t) as u8,
        (color1.r() as f32 + (color2.r() as f32 - color1.r() as f32) * t) as u8,
        (color1.g() as f32 + (color2.g() as f32 - color1.g() as f32) * t) as u8,
        (color1.b() as f32 + (color2.b() as f32 - color1.b() as f32) * t) as u8,
    )
}
