use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/components/icons/icons");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lucide_generated.rs");
    let mut f = fs::File::create(&dest_path).unwrap();

    writeln!(f, "/// Auto-generated Lucide icons from SVG files").unwrap();
    writeln!(f, "pub struct LucideIcons;").unwrap();
    writeln!(f, "").unwrap();
    writeln!(f, "impl LucideIcons {{").unwrap();

    let icons_dir = "src/components/icons/icons";
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
