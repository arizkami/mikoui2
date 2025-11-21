use skia_safe::{Data, Font, FontMgr, FontStyle, Typeface};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Thai,
    Japanese,
    Korean,
    Chinese,
    Arabic,
    Hebrew,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

pub struct FontManager {
    // Primary system font
    primary_typeface: Option<Typeface>,
    
    // Monospace font for code
    monospace_typeface: Option<Typeface>,
    
    // Language-specific fonts
    thai_typeface: Option<Typeface>,
    cjk_typeface: Option<Typeface>,
    arabic_typeface: Option<Typeface>,
    
    // System font manager
    font_mgr: FontMgr,
    
    // Font cache
    font_cache: HashMap<(Language, i32, i32), Font>,
    mono_font_cache: HashMap<(i32, i32), Font>,
}

impl FontManager {
    pub fn new() -> Self {
        let mut manager = Self {
            primary_typeface: None,
            monospace_typeface: None,
            thai_typeface: None,
            cjk_typeface: None,
            arabic_typeface: None,
            font_mgr: FontMgr::new(),
            font_cache: HashMap::new(),
            mono_font_cache: HashMap::new(),
        };
        
        manager.load_fonts();
        manager
    }
    
    fn load_fonts(&mut self) {
        // Load system default font based on platform
        self.load_system_font();
        
        // Load monospace font for code
        self.load_monospace_font();
        
        // Try to load Thai fonts from system
        self.load_thai_fonts();
        
        // Try to load CJK fonts from system
        self.load_cjk_fonts();
        
        // Try to load Arabic fonts from system
        self.load_arabic_fonts();
    }
    
    fn load_system_font(&mut self) {
        // Platform-specific system fonts
        let system_fonts = if cfg!(target_os = "windows") {
            vec!["Segoe UI", "Segoe UI Variable", "Arial", "Tahoma"]
        } else if cfg!(target_os = "macos") {
            vec!["SF Pro", "Helvetica Neue", "Helvetica", "Arial"]
        } else {
            // Linux
            vec!["Ubuntu", "Noto Sans", "DejaVu Sans", "Liberation Sans", "Arial"]
        };
        
        for font_name in system_fonts {
            if let Some(typeface) = self.font_mgr.match_family_style(font_name, FontStyle::normal()) {
                println!("✓ Loaded system font: {}", font_name);
                self.primary_typeface = Some(typeface);
                return;
            }
        }
        
        println!("⚠ No system font found, using default");
    }
    
    fn load_monospace_font(&mut self) {
        // Platform-specific monospace fonts
        let mono_fonts = if cfg!(target_os = "windows") {
            vec!["Consolas", "Cascadia Code", "Cascadia Mono", "Courier New"]
        } else if cfg!(target_os = "macos") {
            vec!["SF Mono", "Menlo", "Monaco", "Courier New"]
        } else {
            // Linux
            vec!["JetBrains Mono", "Fira Code", "Ubuntu Mono", "DejaVu Sans Mono", "Liberation Mono", "Courier New"]
        };
        
        for font_name in mono_fonts {
            if let Some(typeface) = self.font_mgr.match_family_style(font_name, FontStyle::normal()) {
                println!("✓ Loaded monospace font: {}", font_name);
                self.monospace_typeface = Some(typeface);
                return;
            }
        }
        
        println!("⚠ No monospace font found, using primary font");
    }
    
    /// Set custom primary font (e.g., Inter Variable from app)
    pub fn set_primary_font(&mut self, font_data: &[u8]) -> bool {
        let data = Data::new_copy(font_data);
        
        if let Some(typeface) = self.font_mgr.new_from_data(&data, None) {
            println!("✓ Loaded custom primary font ({} bytes)", font_data.len());
            self.primary_typeface = Some(typeface);
            self.clear_cache(); // Clear cache to use new font
            true
        } else {
            println!("✗ Failed to load custom primary font");
            false
        }
    }
    
    fn load_thai_fonts(&mut self) {
        // Try multiple Thai fonts in order of preference
        let thai_fonts = if cfg!(target_os = "windows") {
            vec!["Leelawadee UI", "Leelawadee", "Noto Sans Thai", "Tahoma"]
        } else if cfg!(target_os = "macos") {
            vec!["Thonburi", "Sukhumvit Set", "Noto Sans Thai", "Ayuthaya"]
        } else {
            vec!["Noto Sans Thai", "Loma", "Garuda", "Sarabun"]
        };
        
        for font_name in thai_fonts {
            if let Some(typeface) = self.font_mgr.match_family_style(font_name, FontStyle::normal()) {
                println!("✓ Loaded Thai font: {}", font_name);
                self.thai_typeface = Some(typeface);
                return;
            }
        }
        
        println!("⚠ No Thai font found, using primary font as fallback");
    }
    
    fn load_cjk_fonts(&mut self) {
        // Try CJK fonts
        let cjk_fonts = if cfg!(target_os = "windows") {
            vec!["Microsoft YaHei", "MS Gothic", "Malgun Gothic", "Yu Gothic", "Noto Sans CJK JP"]
        } else if cfg!(target_os = "macos") {
            vec!["PingFang SC", "Hiragino Sans", "Apple SD Gothic Neo", "Noto Sans CJK JP"]
        } else {
            vec!["Noto Sans CJK JP", "Noto Sans JP", "Noto Sans KR", "Noto Sans SC", "WenQuanYi Micro Hei"]
        };
        
        for font_name in cjk_fonts {
            if let Some(typeface) = self.font_mgr.match_family_style(font_name, FontStyle::normal()) {
                println!("✓ Loaded CJK font: {}", font_name);
                self.cjk_typeface = Some(typeface);
                return;
            }
        }
        
        println!("⚠ No CJK font found, using primary font as fallback");
    }
    
    fn load_arabic_fonts(&mut self) {
        // Try Arabic fonts
        let arabic_fonts = if cfg!(target_os = "windows") {
            vec!["Segoe UI", "Noto Sans Arabic", "Tahoma", "Arial"]
        } else if cfg!(target_os = "macos") {
            vec!["Geeza Pro", "Noto Sans Arabic", "Baghdad", "Damascus"]
        } else {
            vec!["Noto Sans Arabic", "DejaVu Sans", "FreeSans"]
        };
        
        for font_name in arabic_fonts {
            if let Some(typeface) = self.font_mgr.match_family_style(font_name, FontStyle::normal()) {
                println!("✓ Loaded Arabic font: {}", font_name);
                self.arabic_typeface = Some(typeface);
                return;
            }
        }
        
        println!("⚠ No Arabic font found, using primary font as fallback");
    }
    
    /// Detect language from text content
    pub fn detect_language(text: &str) -> Language {
        for ch in text.chars() {
            match ch {
                // Thai Unicode range
                '\u{0E00}'..='\u{0E7F}' => return Language::Thai,
                // Japanese Hiragana/Katakana
                '\u{3040}'..='\u{309F}' | '\u{30A0}'..='\u{30FF}' => return Language::Japanese,
                // Korean Hangul
                '\u{AC00}'..='\u{D7AF}' | '\u{1100}'..='\u{11FF}' => return Language::Korean,
                // Chinese (CJK Unified Ideographs)
                '\u{4E00}'..='\u{9FFF}' => return Language::Chinese,
                // Arabic
                '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' => return Language::Arabic,
                // Hebrew
                '\u{0590}'..='\u{05FF}' => return Language::Hebrew,
                _ => continue,
            }
        }
        Language::English
    }
    
    /// Get appropriate typeface for language with fallback chain
    fn get_typeface_for_language(&self, language: Language) -> &Typeface {
        match language {
            Language::Thai => {
                // Try Thai font first, then primary
                self.thai_typeface.as_ref()
                    .or(self.primary_typeface.as_ref())
                    .expect("No typeface available")
            }
            Language::Japanese | Language::Korean | Language::Chinese => {
                // Try CJK font first, then primary
                self.cjk_typeface.as_ref()
                    .or(self.primary_typeface.as_ref())
                    .expect("No typeface available")
            }
            Language::Arabic | Language::Hebrew => {
                // Try Arabic font first, then primary
                self.arabic_typeface.as_ref()
                    .or(self.primary_typeface.as_ref())
                    .expect("No typeface available")
            }
            _ => {
                self.primary_typeface.as_ref()
                    .expect("No primary typeface available")
            }
        }
    }
    
    /// Match a typeface that can render the given character
    pub fn match_char_typeface(&self, ch: char) -> Option<Typeface> {
        // Detect language from character
        let language = Self::detect_language(&ch.to_string());
        
        // Get the appropriate typeface for this language
        let typeface = self.get_typeface_for_language(language);
        
        // Check if the typeface has the glyph for this character
        let glyph_id = typeface.unichar_to_glyph(ch as i32);
        if glyph_id != 0 {
            return Some(typeface.clone());
        }
        
        // Fallback: try all available typefaces
        if let Some(ref thai_tf) = self.thai_typeface {
            if thai_tf.unichar_to_glyph(ch as i32) != 0 {
                return Some(thai_tf.clone());
            }
        }
        
        if let Some(ref cjk_tf) = self.cjk_typeface {
            if cjk_tf.unichar_to_glyph(ch as i32) != 0 {
                return Some(cjk_tf.clone());
            }
        }
        
        if let Some(ref arabic_tf) = self.arabic_typeface {
            if arabic_tf.unichar_to_glyph(ch as i32) != 0 {
                return Some(arabic_tf.clone());
            }
        }
        
        // Last resort: use primary typeface
        self.primary_typeface.clone()
    }
    
    /// Create font with Variable Font support and language detection
    pub fn create_font(&mut self, text: &str, size: f32, weight: i32) -> Font {
        let language = Self::detect_language(text);
        self.create_font_for_language(language, size, weight)
    }
    
    /// Create font for specific language
    pub fn create_font_for_language(&mut self, language: Language, size: f32, weight: i32) -> Font {
        // Check cache first
        let cache_key = (language, size as i32, weight);
        if let Some(font) = self.font_cache.get(&cache_key) {
            return font.clone();
        }
        
        let typeface = self.get_typeface_for_language(language);
        let font = self.create_variable_font(typeface, size, weight);
        
        // Cache the font
        self.font_cache.insert(cache_key, font.clone());
        font
    }
    
    /// Create Variable Font with proper axes configuration
    fn create_variable_font(&self, typeface: &Typeface, size: f32, weight: i32) -> Font {
        // Only apply VF axes to Inter Variable (primary font)
        let is_variable_font = self.primary_typeface.as_ref()
            .map(|tf| std::ptr::eq(tf as *const _, typeface as *const _))
            .unwrap_or(false);
        
        let varied_typeface = if is_variable_font {
            // Apply Variable Font axes for Inter
            let weight_value = weight.clamp(100, 900) as f32;
            let opsz_value = size.clamp(14.0, 32.0);
            
            let wght_tag = skia_safe::FourByteTag::from_chars('w', 'g', 'h', 't');
            let opsz_tag = skia_safe::FourByteTag::from_chars('o', 'p', 's', 'z');
            
            use skia_safe::font_arguments::variation_position::Coordinate;
            let coordinates = [
                Coordinate {
                    axis: wght_tag,
                    value: weight_value,
                },
                Coordinate {
                    axis: opsz_tag,
                    value: opsz_value,
                },
            ];
            
            let font_args = skia_safe::FontArguments::new().set_variation_design_position(
                skia_safe::font_arguments::VariationPosition {
                    coordinates: &coordinates,
                },
            );
            
            typeface.clone_with_arguments(&font_args).unwrap_or_else(|| typeface.clone())
        } else {
            typeface.clone()
        };
        
        let mut font = Font::from_typeface(&varied_typeface, size);
        
        // Enhanced sub-pixel rendering
        font.set_edging(skia_safe::font::Edging::SubpixelAntiAlias);
        font.set_subpixel(true);
        font.set_linear_metrics(true);
        font.set_hinting(skia_safe::FontHinting::None);
        font.set_force_auto_hinting(false);
        font.set_embedded_bitmaps(false);
        font.set_baseline_snap(false);
        
        // Optical size compensation for Variable Font
        if is_variable_font {
            let spacing_adjustment = if size <= 12.0 {
                1.01 // More spacing for small text
            } else if size >= 20.0 {
                1.0 // Normal spacing for large text
            } else {
                1.0
            };
            font.set_scale_x(spacing_adjustment);
        }
        
        font
    }
    
    /// Create monospace font for code/terminal
    pub fn create_mono_font(&mut self, size: f32, weight: i32) -> Font {
        // Check cache first
        let cache_key = (size as i32, weight);
        if let Some(font) = self.mono_font_cache.get(&cache_key) {
            return font.clone();
        }
        
        let typeface = self.monospace_typeface.as_ref()
            .or(self.primary_typeface.as_ref())
            .expect("No typeface available");
        
        let mut font = Font::from_typeface(typeface, size);
        
        // Optimized settings for monospace fonts
        font.set_edging(skia_safe::font::Edging::SubpixelAntiAlias);
        font.set_subpixel(true);
        font.set_linear_metrics(false); // Disable for monospace to maintain grid alignment
        font.set_hinting(skia_safe::FontHinting::Slight); // Slight hinting for better readability
        font.set_force_auto_hinting(false);
        font.set_embedded_bitmaps(false);
        font.set_baseline_snap(true); // Enable for monospace to maintain alignment
        
        // Cache the font
        self.mono_font_cache.insert(cache_key, font.clone());
        font
    }
    
    /// Create monospace font with multi-language support
    /// Uses sample text to detect language and select appropriate fallback fonts
    pub fn create_monospace_font(&mut self, text: &str, size: f32, weight: i32) -> Font {
        // Detect if text contains non-ASCII characters
        let has_special_chars = text.chars().any(|c| c as u32 > 127);
        
        if !has_special_chars {
            // Use regular monospace font for ASCII-only text
            return self.create_mono_font(size, weight);
        }
        
        // For text with special characters, use language-aware font selection
        // but still prefer monospace characteristics
        let language = Self::detect_language(text);
        
        // Try to use monospace font first, but fall back to language-specific fonts
        // if the monospace font doesn't support the characters
        let typeface = match language {
            Language::English | Language::Other => {
                self.monospace_typeface.as_ref()
                    .or(self.primary_typeface.as_ref())
                    .expect("No typeface available")
            }
            _ => {
                // For non-English, prefer language-specific fonts over monospace
                // since most monospace fonts don't support CJK/Thai/Arabic well
                self.get_typeface_for_language(language)
            }
        };
        
        let mut font = Font::from_typeface(typeface, size);
        
        // Apply monospace-optimized settings
        font.set_edging(skia_safe::font::Edging::SubpixelAntiAlias);
        font.set_subpixel(true);
        font.set_linear_metrics(false);
        font.set_hinting(skia_safe::FontHinting::Slight);
        font.set_force_auto_hinting(false);
        font.set_embedded_bitmaps(false);
        font.set_baseline_snap(true);
        
        font
    }
    
    /// Set custom monospace font (e.g., JetBrains Mono, Fira Code)
    pub fn set_monospace_font(&mut self, font_data: &[u8]) -> bool {
        let data = Data::new_copy(font_data);
        
        if let Some(typeface) = self.font_mgr.new_from_data(&data, None) {
            println!("✓ Loaded custom monospace font ({} bytes)", font_data.len());
            self.monospace_typeface = Some(typeface);
            self.mono_font_cache.clear(); // Clear cache to use new font
            true
        } else {
            println!("✗ Failed to load custom monospace font");
            false
        }
    }
    
    /// Clear font cache
    pub fn clear_cache(&mut self) {
        self.font_cache.clear();
        self.mono_font_cache.clear();
    }
    
    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.font_cache.len() + self.mono_font_cache.len()
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_language_detection() {
        assert_eq!(FontManager::detect_language("Hello"), Language::English);
        assert_eq!(FontManager::detect_language("สวัสดี"), Language::Thai);
        assert_eq!(FontManager::detect_language("こんにちは"), Language::Japanese);
        assert_eq!(FontManager::detect_language("안녕하세요"), Language::Korean);
        assert_eq!(FontManager::detect_language("你好"), Language::Chinese);
        assert_eq!(FontManager::detect_language("مرحبا"), Language::Arabic);
    }
    
    #[test]
    fn test_mixed_language() {
        // Should detect first non-English language
        assert_eq!(FontManager::detect_language("Hello สวัสดี"), Language::Thai);
        assert_eq!(FontManager::detect_language("Test 你好"), Language::Chinese);
    }
}
