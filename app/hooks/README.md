# Rabital Config Hooks

This module provides automatic configuration loading for Rabital IDE.

## Features

- **Auto-detection**: Automatically detects `.rabital` folder in workspace
- **YAML parsing**: Parses `settings.yml`, `tasks.yml`, and `debug.yml`
- **Global fallback**: Falls back to `shared/config/setting.yml` for global settings
- **Theme management**: Loads themes from `shared/themes/` directory

## Directory Structure

```
workspace/
├── .rabital/              # Project-specific configs (auto-detected)
│   ├── settings.yml       # Editor settings
│   ├── tasks.yml          # Build/run tasks
│   └── debug.yml          # Debug configurations
│
{appdir}/
└── shared/                # Global configs
    ├── themes/            # Theme files
    │   ├── default.yml
    │   ├── sun.yml
    │   └── greyscale.yml
    └── config/
        └── setting.yml    # Global settings
```

## Usage

```rust
use hooks::ConfigLoader;

// Create config loader
let mut loader = ConfigLoader::new();

// Set workspace (auto-loads .rabital configs)
loader.set_workspace(PathBuf::from("/path/to/workspace"));

// Access loaded settings
if let Some(settings) = loader.get_settings() {
    println!("Theme: {}", settings.editor.theme);
    println!("Font: {}", settings.editor.font_family);
}

// List available themes
let themes = loader.list_themes();
println!("Available themes: {:?}", themes);

// Load a specific theme
if let Some(theme_content) = loader.load_theme("default") {
    println!("Theme loaded: {}", theme_content);
}
```

## Configuration Priority

1. **Project config**: `.rabital/settings.yml` in workspace (highest priority)
2. **Global config**: `shared/config/setting.yml` in app directory
3. **Default config**: Hardcoded defaults in code

## Config Files

### settings.yml
Editor behavior, appearance, language-specific settings, explorer, terminal, git, and search configurations.

### tasks.yml
Build, test, run, and custom tasks with command definitions.

### debug.yml
Debug configurations for LLDB/Visual Studio debugger.

## Global Paths

The config loader provides helper methods to access global directories:

- `get_shared_dir()` - Returns `{appdir}/shared`
- `get_themes_dir()` - Returns `{appdir}/shared/themes`
- `get_config_dir()` - Returns `{appdir}/shared/config`
