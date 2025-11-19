/// File dialog utilities for opening and saving files/folders
/// Cross-platform file dialogs using native Windows APIs

#[cfg(target_os = "windows")]
pub mod windows {
    use windows::Win32::UI::Shell::Common::COMDLG_FILTERSPEC;
    use windows::Win32::UI::Shell::{
        IFileOpenDialog, IFileSaveDialog, FileOpenDialog, FileSaveDialog,
        FOS_PICKFOLDERS, FOS_ALLOWMULTISELECT, FOS_FORCEFILESYSTEM,
    };
    use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED};
    use windows::core::{PWSTR, PCWSTR};
    use std::path::PathBuf;

    /// Open a file dialog to select a single file
    pub fn open_file_dialog(title: &str, filters: &[(&str, &str)]) -> Option<PathBuf> {
        unsafe {
            // Initialize COM
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            // Create file open dialog
            let dialog: IFileOpenDialog = CoCreateInstance(&FileOpenDialog, None, CLSCTX_ALL).ok()?;

            // Set title
            let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
            let _ = dialog.SetTitle(PWSTR(title_wide.as_ptr() as *mut u16));

            // Set file type filters
            if !filters.is_empty() {
                let filter_specs: Vec<COMDLG_FILTERSPEC> = filters
                    .iter()
                    .map(|(name, pattern)| {
                        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
                        let pattern_wide: Vec<u16> = pattern.encode_utf16().chain(std::iter::once(0)).collect();
                        COMDLG_FILTERSPEC {
                            pszName: PCWSTR(name_wide.as_ptr()),
                            pszSpec: PCWSTR(pattern_wide.as_ptr()),
                        }
                    })
                    .collect();
                let _ = dialog.SetFileTypes(&filter_specs);
            }

            // Show dialog
            if dialog.Show(None).is_ok() {
                if let Ok(item) = dialog.GetResult() {
                    if let Ok(path_pwstr) = item.GetDisplayName(windows::Win32::UI::Shell::SIGDN_FILESYSPATH) {
                        let path_str = path_pwstr.to_string().ok()?;
                        return Some(PathBuf::from(path_str));
                    }
                }
            }

            None
        }
    }

    /// Open a file dialog to select multiple files
    pub fn open_files_dialog(title: &str, filters: &[(&str, &str)]) -> Vec<PathBuf> {
        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            let dialog: IFileOpenDialog = match CoCreateInstance(&FileOpenDialog, None, CLSCTX_ALL) {
                Ok(d) => d,
                Err(_) => return Vec::new(),
            };

            let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
            let _ = dialog.SetTitle(PWSTR(title_wide.as_ptr() as *mut u16));

            // Enable multi-select
            if let Ok(options) = dialog.GetOptions() {
                let _ = dialog.SetOptions(options | FOS_ALLOWMULTISELECT | FOS_FORCEFILESYSTEM);
            }

            if !filters.is_empty() {
                let filter_specs: Vec<COMDLG_FILTERSPEC> = filters
                    .iter()
                    .map(|(name, pattern)| {
                        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
                        let pattern_wide: Vec<u16> = pattern.encode_utf16().chain(std::iter::once(0)).collect();
                        COMDLG_FILTERSPEC {
                            pszName: PCWSTR(name_wide.as_ptr()),
                            pszSpec: PCWSTR(pattern_wide.as_ptr()),
                        }
                    })
                    .collect();
                let _ = dialog.SetFileTypes(&filter_specs);
            }

            if dialog.Show(None).is_ok() {
                if let Ok(items) = dialog.GetResults() {
                    if let Ok(count) = items.GetCount() {
                        let mut paths = Vec::new();
                        for i in 0..count {
                            if let Ok(item) = items.GetItemAt(i) {
                                if let Ok(path_pwstr) = item.GetDisplayName(windows::Win32::UI::Shell::SIGDN_FILESYSPATH) {
                                    if let Ok(path_str) = path_pwstr.to_string() {
                                        paths.push(PathBuf::from(path_str));
                                    }
                                }
                            }
                        }
                        return paths;
                    }
                }
            }

            Vec::new()
        }
    }

    /// Open a folder picker dialog
    pub fn open_folder_dialog(title: &str) -> Option<PathBuf> {
        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            let dialog: IFileOpenDialog = CoCreateInstance(&FileOpenDialog, None, CLSCTX_ALL).ok()?;

            let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
            let _ = dialog.SetTitle(PWSTR(title_wide.as_ptr() as *mut u16));

            // Set options for folder picking
            if let Ok(options) = dialog.GetOptions() {
                let _ = dialog.SetOptions(options | FOS_PICKFOLDERS | FOS_FORCEFILESYSTEM);
            }

            if dialog.Show(None).is_ok() {
                if let Ok(item) = dialog.GetResult() {
                    if let Ok(path_pwstr) = item.GetDisplayName(windows::Win32::UI::Shell::SIGDN_FILESYSPATH) {
                        let path_str = path_pwstr.to_string().ok()?;
                        return Some(PathBuf::from(path_str));
                    }
                }
            }

            None
        }
    }

    /// Open a save file dialog
    pub fn save_file_dialog(title: &str, default_name: &str, filters: &[(&str, &str)]) -> Option<PathBuf> {
        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            let dialog: IFileSaveDialog = CoCreateInstance(&FileSaveDialog, None, CLSCTX_ALL).ok()?;

            let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
            let _ = dialog.SetTitle(PWSTR(title_wide.as_ptr() as *mut u16));

            // Set default filename
            if !default_name.is_empty() {
                let name_wide: Vec<u16> = default_name.encode_utf16().chain(std::iter::once(0)).collect();
                let _ = dialog.SetFileName(PWSTR(name_wide.as_ptr() as *mut u16));
            }

            // Set file type filters
            if !filters.is_empty() {
                let filter_specs: Vec<COMDLG_FILTERSPEC> = filters
                    .iter()
                    .map(|(name, pattern)| {
                        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
                        let pattern_wide: Vec<u16> = pattern.encode_utf16().chain(std::iter::once(0)).collect();
                        COMDLG_FILTERSPEC {
                            pszName: PCWSTR(name_wide.as_ptr()),
                            pszSpec: PCWSTR(pattern_wide.as_ptr()),
                        }
                    })
                    .collect();
                let _ = dialog.SetFileTypes(&filter_specs);
            }

            if dialog.Show(None).is_ok() {
                if let Ok(item) = dialog.GetResult() {
                    if let Ok(path_pwstr) = item.GetDisplayName(windows::Win32::UI::Shell::SIGDN_FILESYSPATH) {
                        let path_str = path_pwstr.to_string().ok()?;
                        return Some(PathBuf::from(path_str));
                    }
                }
            }

            None
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub mod windows {
    use std::path::PathBuf;

    pub fn open_file_dialog(_title: &str, _filters: &[(&str, &str)]) -> Option<PathBuf> {
        None
    }

    pub fn open_files_dialog(_title: &str, _filters: &[(&str, &str)]) -> Vec<PathBuf> {
        Vec::new()
    }

    pub fn open_folder_dialog(_title: &str) -> Option<PathBuf> {
        None
    }

    pub fn save_file_dialog(_title: &str, _default_name: &str, _filters: &[(&str, &str)]) -> Option<PathBuf> {
        None
    }
}

