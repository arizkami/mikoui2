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

fn main() {
    println!("cargo:rerun-if-changed=src/components/icons/icons");
    println!("cargo:rerun-if-changed=src/components/codicon/src/icons");
    println!("cargo:rerun-if-changed=app/assets/app.rc");
    println!("cargo:rerun-if-changed=app/assets/icon.ico");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Generate Lucide icons
    generate_icons(
        "src/components/icons/icons",
        "lucide_generated.rs",
        "LucideIcons",
        &out_dir
    );
    
    // Generate Codicon icons
    generate_icons(
        "src/components/codicon/src/icons",
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
}
