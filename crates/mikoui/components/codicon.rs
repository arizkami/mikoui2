/// Codicon icons - Auto-generated from SVG files at compile time
/// Source: https://github.com/microsoft/vscode-codicons
/// 
/// All icons are 16x16 viewBox
/// 
/// Usage:
/// ```rust
/// use components::{Icon, IconSize, CodiconIcons};
/// 
/// let icon = Icon::new(x, y, CodiconIcons::FILE, IconSize::Small, color);
/// ```

// Include the generated icon constants from build.rs
include!(concat!(env!("OUT_DIR"), "/codicon_generated.rs"));
