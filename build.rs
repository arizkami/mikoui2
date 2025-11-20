use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn generate_icons(icons_dir: &str, output_file: &str, struct_name: &str, out_dir: &str) {
    let dest_path = Path::new(out_dir).join(output_file);
    let mut f = fs::File::create(&dest_path).unwrap();

    writeln!(f, "/// Auto-generated {} icons from SVG files", struct_name).unwrap();
    writeln!(f, "pub struct {};", struct_name).unwrap();
    writeln!(f, "").unwrap();
    writeln!(f, "impl {} {{", struct_name).unwrap();

    if let Ok(entries) = fs::read_dir(icons_dir) {
        let mut icons = Vec::new();
        
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("svg") {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        if let Ok(content) = fs::read_to_string(&path) {
                            // Normalize line endings to Unix style (LF only) to avoid "bare CR" errors
                            let normalized = content.replace("\r\n", "\n").replace("\r", "\n");
                            icons.push((name.to_string(), normalized));
                        }
                    }
                }
            }
        }

        // Sort icons by name
        icons.sort_by(|a, b| a.0.cmp(&b.0));

        // Generate constants with full SVG content
        for (name, svg_content) in icons {
            let const_name = name.to_uppercase().replace("-", "_");
            writeln!(f, "    pub const {}: &'static str = r#\"{}\"#;", const_name, svg_content).unwrap();
        }
    }

    writeln!(f, "}}").unwrap();
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);
        
        if path.is_dir() {
            copy_dir_recursive(&path, &dst_path)?;
        } else {
            fs::copy(&path, &dst_path)?;
        }
    }
    
    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=cretes/mikoui/components/icons/icons");
    println!("cargo:rerun-if-changed=cretes/mikoui/components/codicon/cretes/mikoui/icons");
    println!("cargo:rerun-if-changed=app/assets/app.rc");
    println!("cargo:rerun-if-changed=app/assets/icon.ico");
    println!("cargo:rerun-if-changed=shared");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Generate Lucide icons
    generate_icons(
        "cretes/mikoui/components/icons/icons",
        "lucide_generated.rs",
        "LucideIcons",
        &out_dir
    );
    
    // Generate Codicon icons
    generate_icons(
        "cretes/mikoui/components/codicon/cretes/mikoui/icons",
        "codicon_generated.rs",
        "CodiconIcons",
        &out_dir
    );
    
    // Compile Windows resources (icon) for the app binary
    #[cfg(target_os = "windows")]
    {
        // Always compile the resource file when building
        if Path::new("app/assets/app.rc").exists() {
            println!("cargo:warning=Compiling Windows resource file app/assets/app.rc");
            embed_resource::compile("app/assets/app.rc", embed_resource::NONE);
        } else {
            println!("cargo:warning=Resource file app/assets/app.rc not found");
        }
    }
    
    // Copy shared folder to build directory (CMake style)
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let build_dir = Path::new("build").join(&profile);
    let shared_src = Path::new("shared");
    let shared_dst = build_dir.join("bin/shared");
    
    // Create build directory structure
    if let Err(e) = fs::create_dir_all(&build_dir) {
        println!("cargo:warning=Failed to create build directory: {}", e);
    }
    
    if shared_src.exists() {
        if let Err(e) = copy_dir_recursive(shared_src, &shared_dst) {
            println!("cargo:warning=Failed to copy shared folder: {}", e);
        } else {
            println!("cargo:warning=Copied shared folder to {}", shared_dst.display());
        }
    } else {
        println!("cargo:warning=Shared folder not found at {}", shared_src.display());
    }
}
