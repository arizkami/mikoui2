pub mod leftpanel;
pub mod rightpanel;
pub mod bottompanel;

pub use leftpanel::LeftPanel;
pub use rightpanel::RightPanel;
pub use bottompanel::BottomPanel;

/// Layout configuration
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    pub left_panel_width: f32,
    pub right_panel_width: f32,
    pub bottom_panel_height: f32,
    pub left_panel_visible: bool,
    pub right_panel_visible: bool,
    pub bottom_panel_visible: bool,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            left_panel_width: 300.0,
            right_panel_width: 300.0,
            bottom_panel_height: 200.0,
            left_panel_visible: true,
            right_panel_visible: false,
            bottom_panel_visible: false,
        }
    }
}
