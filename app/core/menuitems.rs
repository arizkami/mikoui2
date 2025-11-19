use mikoui::{MenuBarItem, MenuItem};
use std::process::Command;

/// Spawn a new window instance
fn spawn_new_window() {
    // Get the current executable path
    if let Ok(exe_path) = std::env::current_exe() {
        // Spawn a new process with the same executable
        match Command::new(exe_path)
            .spawn()
        {
            Ok(_) => println!("New window spawned successfully"),
            Err(e) => eprintln!("Failed to spawn new window: {}", e),
        }
    } else {
        eprintln!("Failed to get current executable path");
    }
}

/// Create the default editor menu structure
pub fn create_editor_menus() -> Vec<MenuBarItem> {
    vec![
        MenuBarItem::new("File", vec![
            MenuItem::new("New File", 1).with_shortcut("Ctrl+N"),
            MenuItem::new("New Window", 2).with_shortcut("Ctrl+Shift+N"),
            MenuItem::new("Open File...", 3).with_shortcut("Ctrl+O"),
            MenuItem::new("Open Folder...", 4).with_shortcut("Ctrl+K Ctrl+O"),
            MenuItem::new("Open Recent", 5),
            MenuItem::separator(),
            MenuItem::new("Save", 6).with_shortcut("Ctrl+S"),
            MenuItem::new("Save As...", 7).with_shortcut("Ctrl+Shift+S"),
            MenuItem::new("Save All", 8).with_shortcut("Ctrl+K S"),
            MenuItem::separator(),
            MenuItem::new("Auto Save", 9),
            MenuItem::separator(),
            MenuItem::new("Close", 10).with_shortcut("Ctrl+W"),
            MenuItem::new("Close All", 11).with_shortcut("Ctrl+K Ctrl+W"),
            MenuItem::new("Revert File", 12),
            MenuItem::separator(),
            MenuItem::new("Preferences", 13).with_shortcut("Ctrl+,"),
            MenuItem::separator(),
            MenuItem::new("Exit", 14).with_shortcut("Alt+F4"),
        ]),
        MenuBarItem::new("Edit", vec![
            MenuItem::new("Undo", 20).with_shortcut("Ctrl+Z"),
            MenuItem::new("Redo", 21).with_shortcut("Ctrl+Y"),
            MenuItem::separator(),
            MenuItem::new("Cut", 22).with_shortcut("Ctrl+X"),
            MenuItem::new("Copy", 23).with_shortcut("Ctrl+C"),
            MenuItem::new("Paste", 24).with_shortcut("Ctrl+V"),
            MenuItem::new("Delete", 25).with_shortcut("Del"),
            MenuItem::separator(),
            MenuItem::new("Select All", 26).with_shortcut("Ctrl+A"),
            MenuItem::new("Expand Selection", 27).with_shortcut("Shift+Alt+Right"),
            MenuItem::new("Shrink Selection", 28).with_shortcut("Shift+Alt+Left"),
            MenuItem::separator(),
            MenuItem::new("Find", 29).with_shortcut("Ctrl+F"),
            MenuItem::new("Find Next", 30).with_shortcut("F3"),
            MenuItem::new("Find Previous", 31).with_shortcut("Shift+F3"),
            MenuItem::new("Replace", 32).with_shortcut("Ctrl+H"),
            MenuItem::separator(),
            MenuItem::new("Find in Files", 33).with_shortcut("Ctrl+Shift+F"),
            MenuItem::new("Replace in Files", 34).with_shortcut("Ctrl+Shift+H"),
            MenuItem::separator(),
            MenuItem::new("Go To Line...", 35).with_shortcut("Ctrl+G"),
            MenuItem::new("Go To Symbol...", 36).with_shortcut("Ctrl+Shift+O"),
            MenuItem::separator(),
            MenuItem::new("Toggle Line Comment", 37).with_shortcut("Ctrl+/"),
            MenuItem::new("Toggle Block Comment", 38).with_shortcut("Shift+Alt+A"),
            MenuItem::separator(),
            MenuItem::new("Format Document", 39).with_shortcut("Shift+Alt+F"),
            MenuItem::new("Format Selection", 40).with_shortcut("Ctrl+K Ctrl+F"),
            MenuItem::new("Trim Trailing Whitespace", 41),
        ]),
        MenuBarItem::new("Selection", vec![
            MenuItem::new("Select Line", 50).with_shortcut("Ctrl+L"),
            MenuItem::new("Select Word", 51).with_shortcut("Ctrl+D"),
            MenuItem::new("Expand Selection", 52).with_shortcut("Shift+Alt+Right"),
            MenuItem::new("Shrink Selection", 53).with_shortcut("Shift+Alt+Left"),
            MenuItem::separator(),
            MenuItem::new("Select All Occurrences", 54).with_shortcut("Ctrl+Shift+L"),
            MenuItem::new("Add Cursor Above", 55).with_shortcut("Ctrl+Alt+Up"),
            MenuItem::new("Add Cursor Below", 56).with_shortcut("Ctrl+Alt+Down"),
            MenuItem::new("Add Next Occurrence", 57).with_shortcut("Ctrl+D"),
            MenuItem::new("Undo Last Cursor", 58).with_shortcut("Ctrl+U"),
        ]),
        MenuBarItem::new("View", vec![
            MenuItem::new("Command Palette", 60).with_shortcut("Ctrl+Shift+P"),
            MenuItem::new("Open View...", 61).with_shortcut("Ctrl+Q"),
            MenuItem::separator(),
            MenuItem::new("Explorer", 62).with_shortcut("Ctrl+Shift+E"),
            MenuItem::new("Search", 63).with_shortcut("Ctrl+Shift+F"),
            MenuItem::new("Source Control", 64).with_shortcut("Ctrl+Shift+G"),
            MenuItem::new("Run and Debug", 65).with_shortcut("Ctrl+Shift+D"),
            MenuItem::new("Extensions", 66).with_shortcut("Ctrl+Shift+X"),
            MenuItem::separator(),
            MenuItem::new("Problems", 67).with_shortcut("Ctrl+Shift+M"),
            MenuItem::new("Output", 68).with_shortcut("Ctrl+Shift+U"),
            MenuItem::new("Terminal", 69).with_shortcut("Ctrl+`"),
            MenuItem::separator(),
            MenuItem::new("Show Tabs", 70),
            MenuItem::new("Show Status Bar", 71),
            MenuItem::new("Toggle Minimap", 72),
            MenuItem::separator(),
            MenuItem::new("Zoom In", 73).with_shortcut("Ctrl++"),
            MenuItem::new("Zoom Out", 74).with_shortcut("Ctrl+-"),
            MenuItem::new("Reset Zoom", 75).with_shortcut("Ctrl+0"),
            MenuItem::separator(),
            MenuItem::new("Toggle Full Screen", 76).with_shortcut("F11"),
            MenuItem::new("Toggle Zen Mode", 77).with_shortcut("Ctrl+K Z"),
        ]),
        MenuBarItem::new("Go", vec![
            MenuItem::new("Back", 80).with_shortcut("Alt+Left"),
            MenuItem::new("Forward", 81).with_shortcut("Alt+Right"),
            MenuItem::new("Last Edit Location", 82).with_shortcut("Ctrl+K Ctrl+Q"),
            MenuItem::separator(),
            MenuItem::new("Switch Editor", 83).with_shortcut("Ctrl+Tab"),
            MenuItem::new("Go to File...", 84).with_shortcut("Ctrl+P"),
            MenuItem::new("Go to Symbol...", 85).with_shortcut("Ctrl+Shift+O"),
            MenuItem::separator(),
            MenuItem::new("Go to Definition", 86).with_shortcut("F12"),
            MenuItem::new("Go to Declaration", 87),
            MenuItem::new("Go to Type Definition", 88),
            MenuItem::new("Go to Implementation", 89).with_shortcut("Ctrl+F12"),
            MenuItem::new("Go to References", 90).with_shortcut("Shift+F12"),
            MenuItem::separator(),
            MenuItem::new("Go to Line...", 91).with_shortcut("Ctrl+G"),
            MenuItem::new("Go to Bracket", 92).with_shortcut("Ctrl+Shift+\\"),
        ]),
        MenuBarItem::new("Run", vec![
            MenuItem::new("Start Debugging", 100).with_shortcut("F5"),
            MenuItem::new("Run Without Debugging", 101).with_shortcut("Ctrl+F5"),
            MenuItem::new("Stop Debugging", 102).with_shortcut("Shift+F5"),
            MenuItem::new("Restart Debugging", 103).with_shortcut("Ctrl+Shift+F5"),
            MenuItem::separator(),
            MenuItem::new("Step Over", 104).with_shortcut("F10"),
            MenuItem::new("Step Into", 105).with_shortcut("F11"),
            MenuItem::new("Step Out", 106).with_shortcut("Shift+F11"),
            MenuItem::new("Continue", 107).with_shortcut("F5"),
            MenuItem::separator(),
            MenuItem::new("Toggle Breakpoint", 108).with_shortcut("F9"),
            MenuItem::new("New Breakpoint", 109),
            MenuItem::separator(),
            MenuItem::new("Open Configurations", 110),
            MenuItem::new("Add Configuration...", 111),
        ]),
        MenuBarItem::new("Terminal", vec![
            MenuItem::new("New Terminal", 120).with_shortcut("Ctrl+Shift+`"),
            MenuItem::new("Split Terminal", 121).with_shortcut("Ctrl+Shift+5"),
            MenuItem::separator(),
            MenuItem::new("Run Task...", 122).with_shortcut("Ctrl+Shift+B"),
            MenuItem::new("Run Build Task", 123),
            MenuItem::separator(),
            MenuItem::new("Show Running Tasks", 124),
            MenuItem::new("Restart Running Task", 125),
            MenuItem::new("Terminate Task", 126),
            MenuItem::separator(),
            MenuItem::new("Configure Tasks...", 127),
            MenuItem::new("Configure Default Build Task", 128),
        ]),
        MenuBarItem::new("Help", vec![
            MenuItem::new("Welcome", 130),
            MenuItem::new("Show All Commands", 131).with_shortcut("Ctrl+Shift+P"),
            MenuItem::new("Documentation", 132),
            MenuItem::new("Release Notes", 133),
            MenuItem::separator(),
            MenuItem::new("Keyboard Shortcuts Reference", 134).with_shortcut("Ctrl+K Ctrl+R"),
            MenuItem::new("Video Tutorials", 135),
            MenuItem::new("Tips and Tricks", 136),
            MenuItem::separator(),
            MenuItem::new("Join Us on Twitter", 137),
            MenuItem::new("Report Issue", 138),
            MenuItem::separator(),
            MenuItem::new("Check for Updates...", 139),
            MenuItem::separator(),
            MenuItem::new("About", 140),
        ]),
    ]
}

/// Handle menu item actions
pub fn handle_menu_action(item_id: i32) {
    use mikoui::file_dialogs;
    
    match item_id {
        // File menu
        1 => {
            // New File
            println!("New File");
        }
        2 => {
            // New Window - spawn a new instance
            spawn_new_window();
        }
        3 => {
            // Open File
            let filters = [
                ("All Files", "*.*"),
                ("Text Files", "*.txt"),
                ("Rust Files", "*.rs"),
                ("Markdown Files", "*.md"),
            ];
            if let Some(path) = file_dialogs::open_file_dialog("Open File", &filters) {
                println!("Opening file: {:?}", path);
            }
        }
        4 => {
            // Open Folder
            if let Some(path) = file_dialogs::open_folder_dialog("Open Folder") {
                println!("Opening folder: {:?}", path);
            }
        }
        6 => {
            // Save
            println!("Save");
        }
        7 => {
            // Save As
            let filters = [
                ("All Files", "*.*"),
                ("Text Files", "*.txt"),
                ("Rust Files", "*.rs"),
            ];
            if let Some(path) = file_dialogs::save_file_dialog("Save As", "untitled.txt", &filters) {
                println!("Saving to: {:?}", path);
            }
        }
        14 => {
            // Exit
            println!("Exit requested");
            std::process::exit(0);
        }
        _ => {
            println!("Menu item {} clicked (no handler)", item_id);
        }
    }
}
