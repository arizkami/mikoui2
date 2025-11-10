/// Windows DWM (Desktop Window Manager) integration for native effects
/// Provides rounded corners and drop shadows for borderless windows

#[cfg(target_os = "windows")]
pub mod windows {
    use windows::Win32::Foundation::{HWND, RECT};
    use windows::Win32::Graphics::Dwm::{
        DwmSetWindowAttribute, DWMWA_WINDOW_CORNER_PREFERENCE,
        DWMWINDOWATTRIBUTE,
    };

    /// Window corner preference
    #[repr(i32)]
    #[derive(Debug, Clone, Copy)]
    pub enum CornerPreference {
        /// Let the system decide
        Default = 0,
        /// No rounded corners
        DoNotRound = 1,
        /// Rounded corners
        Round = 2,
        /// Small rounded corners
        RoundSmall = 3,
    }

    /// Apply rounded corners to a window
    pub fn set_window_corner_preference(hwnd: isize, preference: CornerPreference) -> bool {
        unsafe {
            let hwnd = HWND(hwnd as *mut std::ffi::c_void);
            let preference = preference as i32;
            let result = DwmSetWindowAttribute(
                hwnd,
                DWMWA_WINDOW_CORNER_PREFERENCE,
                &preference as *const _ as *const _,
                std::mem::size_of::<i32>() as u32,
            );
            result.is_ok()
        }
    }

    /// Enable drop shadow for a borderless window
    pub fn enable_window_shadow(hwnd: isize) -> bool {
        unsafe {
            let hwnd = HWND(hwnd as *mut std::ffi::c_void);
            // DWMWA_NCRENDERING_POLICY = 2
            let policy: i32 = 2; // DWMNCRP_ENABLED
            let result = DwmSetWindowAttribute(
                hwnd,
                DWMWINDOWATTRIBUTE(2),
                &policy as *const _ as *const _,
                std::mem::size_of::<i32>() as u32,
            );
            result.is_ok()
        }
    }

    /// Enable Windows 11 Snap Layouts for custom titlebar
    /// This uses DWMWA_CAPTION_BUTTON_BOUNDS to tell Windows where the maximize button is
    pub fn enable_snap_layouts(hwnd: isize, max_button_rect: (i32, i32, i32, i32)) -> bool {
        unsafe {
            let hwnd = HWND(hwnd as *mut std::ffi::c_void);
            
            // DWMWA_CAPTION_BUTTON_BOUNDS = 5
            // This tells Windows where our caption buttons are for snap layouts
            let rect = RECT {
                left: max_button_rect.0,
                top: max_button_rect.1,
                right: max_button_rect.0 + max_button_rect.2,
                bottom: max_button_rect.1 + max_button_rect.3,
            };
            
            let result = DwmSetWindowAttribute(
                hwnd,
                DWMWINDOWATTRIBUTE(5), // DWMWA_CAPTION_BUTTON_BOUNDS
                &rect as *const _ as *const _,
                std::mem::size_of::<RECT>() as u32,
            );
            
            result.is_ok()
        }
    }
    
    /// Apply modern window styling (rounded corners + shadow)
    pub fn apply_modern_window_style(hwnd: isize) -> bool {
        let shadow = enable_window_shadow(hwnd);
        let corners = set_window_corner_preference(hwnd, CornerPreference::Round);
        shadow && corners
    }
}

#[cfg(not(target_os = "windows"))]
pub mod windows {
    /// Dummy implementation for non-Windows platforms
    #[derive(Debug, Clone, Copy)]
    pub enum CornerPreference {
        Default = 0,
        DoNotRound = 1,
        Round = 2,
        RoundSmall = 3,
    }

    pub fn set_window_corner_preference(_hwnd: isize, _preference: CornerPreference) -> bool {
        false
    }

    pub fn enable_window_shadow(_hwnd: isize) -> bool {
        false
    }

    pub fn enable_snap_layouts(_hwnd: isize, _max_button_rect: (i32, i32, i32, i32)) -> bool {
        false
    }

    pub fn apply_modern_window_style(_hwnd: isize) -> bool {
        false
    }
}
