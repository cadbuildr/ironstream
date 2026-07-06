// FILE: vrml_ascii_text.rs
// occt: Vrml_AsciiText
//
// Faithful port of OCCT Vrml_AsciiText (DataExchange/TKDEVRML/Vrml/
// Vrml_AsciiText.hxx/.cxx): VRML 1.0 AsciiText node.
// Renders text strings in 3D using font specifications, spacing, and justification.

use std::cell::RefCell;
use std::rc::Rc;

/// Simple 3D vector for text positioning.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AsciiTextVec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl AsciiTextVec {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        AsciiTextVec { x, y, z }
    }
}

impl Default for AsciiTextVec {
    fn default() -> Self {
        AsciiTextVec::new(0.0, 0.0, 0.0)
    }
}

/// Justification mode for text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsciiTextJustification {
    Left = 0,
    Center = 1,
    Right = 2,
}

impl AsciiTextJustification {
    pub fn as_str(&self) -> &str {
        match self {
            AsciiTextJustification::Left => "LEFT",
            AsciiTextJustification::Center => "CENTER",
            AsciiTextJustification::Right => "RIGHT",
        }
    }
}

impl Default for AsciiTextJustification {
    fn default() -> Self {
        AsciiTextJustification::Left
    }
}

/// Font specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsciiTextFont {
    pub name: String,  // e.g., "SERIF", "SANS"
}

impl AsciiTextFont {
    pub fn new(name: &str) -> Self {
        AsciiTextFont {
            name: name.to_string(),
        }
    }
}

impl Default for AsciiTextFont {
    fn default() -> Self {
        AsciiTextFont::new("SERIF")
    }
}

/// VRML 1.0 AsciiText node: renders text in 3D space.
/// Supports multiple strings, font specification, character spacing, and justification.
pub struct VrmlAsciiText {
    my_strings: Vec<String>,
    my_font: AsciiTextFont,
    my_height: f64,
    my_spacing: f64,
    my_justification: AsciiTextJustification,
    my_name: String,
}

impl VrmlAsciiText {
    /// Constructor: creates default AsciiText node.
    pub fn new(name: Option<&str>) -> Self {
        VrmlAsciiText {
            my_strings: Vec::new(),
            my_font: AsciiTextFont::default(),
            my_height: 1.0,
            my_spacing: 1.0,
            my_justification: AsciiTextJustification::default(),
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Full constructor with parameters.
    pub fn with_fields(
        strings: Vec<String>,
        font: AsciiTextFont,
        height: f64,
        spacing: f64,
        justification: AsciiTextJustification,
        name: Option<&str>,
    ) -> Self {
        VrmlAsciiText {
            my_strings: strings,
            my_font: font,
            my_height: height.max(0.0),
            my_spacing: spacing.max(0.0),
            my_justification: justification,
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Query the name.
    pub fn name(&self) -> &str {
        &self.my_name
    }

    /// Set the name.
    pub fn set_name(&mut self, name: &str) {
        self.my_name = name.to_string();
    }

    /// Add a text string.
    pub fn add_string(&mut self, text: &str) {
        self.my_strings.push(text.to_string());
    }

    /// Get the number of text strings.
    pub fn string_count(&self) -> usize {
        self.my_strings.len()
    }

    /// Get a string by index.
    pub fn get_string(&self, index: usize) -> Option<&str> {
        self.my_strings.get(index).map(|s| s.as_str())
    }

    /// Set all strings.
    pub fn set_strings(&mut self, strings: Vec<String>) {
        self.my_strings = strings;
    }

    /// Get all strings.
    pub fn strings(&self) -> &[String] {
        &self.my_strings
    }

    /// Clear all strings.
    pub fn clear_strings(&mut self) {
        self.my_strings.clear();
    }

    /// Get the font.
    pub fn font(&self) -> &AsciiTextFont {
        &self.my_font
    }

    /// Set the font.
    pub fn set_font(&mut self, font: AsciiTextFont) {
        self.my_font = font;
    }

    /// Get the text height.
    pub fn height(&self) -> f64 {
        self.my_height
    }

    /// Set the text height.
    pub fn set_height(&mut self, height: f64) {
        self.my_height = height.max(0.0);
    }

    /// Get the character spacing factor.
    pub fn spacing(&self) -> f64 {
        self.my_spacing
    }

    /// Set the character spacing factor.
    pub fn set_spacing(&mut self, spacing: f64) {
        self.my_spacing = spacing.max(0.0);
    }

    /// Get the text justification.
    pub fn justification(&self) -> AsciiTextJustification {
        self.my_justification
    }

    /// Set the text justification.
    pub fn set_justification(&mut self, just: AsciiTextJustification) {
        self.my_justification = just;
    }

    /// Check if this node is in default state.
    pub fn is_default(&self) -> bool {
        self.my_strings.is_empty()
            && self.my_font == AsciiTextFont::default()
            && (self.my_height - 1.0).abs() < 1e-10
            && (self.my_spacing - 1.0).abs() < 1e-10
            && self.my_justification == AsciiTextJustification::default()
    }

    /// Get combined text (all strings joined).
    pub fn combined_text(&self) -> String {
        self.my_strings.join("\n")
    }

    /// Get estimated text width in units (rough approximation).
    pub fn estimated_width(&self) -> f64 {
        if self.my_strings.is_empty() {
            return 0.0;
        }
        let longest = self.my_strings.iter().map(|s| s.len()).max().unwrap_or(0) as f64;
        longest * self.my_height * 0.5 * self.my_spacing
    }
}

impl Default for VrmlAsciiText {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Clone for VrmlAsciiText {
    fn clone(&self) -> Self {
        VrmlAsciiText {
            my_strings: self.my_strings.clone(),
            my_font: self.my_font.clone(),
            my_height: self.my_height,
            my_spacing: self.my_spacing,
            my_justification: self.my_justification,
            my_name: self.my_name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_ascii_text() {
        let text = VrmlAsciiText::new(None);
        assert_eq!(text.string_count(), 0);
        assert_eq!(text.height(), 1.0);
        assert_eq!(text.spacing(), 1.0);
        assert_eq!(text.justification(), AsciiTextJustification::Left);
        assert!(text.is_default());
    }

    #[test]
    fn add_string() {
        let mut text = VrmlAsciiText::new(None);
        text.add_string("Hello");
        text.add_string("World");
        assert_eq!(text.string_count(), 2);
        assert_eq!(text.get_string(0), Some("Hello"));
        assert_eq!(text.get_string(1), Some("World"));
    }

    #[test]
    fn set_height() {
        let mut text = VrmlAsciiText::new(None);
        text.set_height(2.5);
        assert_eq!(text.height(), 2.5);
    }

    #[test]
    fn height_non_negative() {
        let mut text = VrmlAsciiText::new(None);
        text.set_height(-5.0);
        assert_eq!(text.height(), 0.0);
    }

    #[test]
    fn set_spacing() {
        let mut text = VrmlAsciiText::new(None);
        text.set_spacing(0.8);
        assert_eq!(text.spacing(), 0.8);
    }

    #[test]
    fn set_font() {
        let mut text = VrmlAsciiText::new(None);
        text.set_font(AsciiTextFont::new("SANS"));
        assert_eq!(text.font().name, "SANS");
    }

    #[test]
    fn set_justification() {
        let mut text = VrmlAsciiText::new(None);
        text.set_justification(AsciiTextJustification::Center);
        assert_eq!(text.justification(), AsciiTextJustification::Center);
    }

    #[test]
    fn combined_text() {
        let mut text = VrmlAsciiText::new(None);
        text.add_string("Line1");
        text.add_string("Line2");
        text.add_string("Line3");
        assert_eq!(text.combined_text(), "Line1\nLine2\nLine3");
    }

    #[test]
    fn estimated_width() {
        let mut text = VrmlAsciiText::new(None);
        text.add_string("ABCDE");
        text.set_height(2.0);
        text.set_spacing(1.0);
        let width = text.estimated_width();
        assert!(width > 0.0);
    }

    #[test]
    fn clone_preserves_data() {
        let mut text = VrmlAsciiText::new(Some("text1"));
        text.add_string("Hello");
        text.set_height(1.5);
        text.set_spacing(0.9);
        text.set_justification(AsciiTextJustification::Right);
        let cloned = text.clone();
        assert_eq!(cloned.name(), "text1");
        assert_eq!(cloned.string_count(), 1);
        assert_eq!(cloned.height(), 1.5);
        assert_eq!(cloned.spacing(), 0.9);
        assert_eq!(cloned.justification(), AsciiTextJustification::Right);
    }

    #[test]
    fn justification_string_repr() {
        assert_eq!(AsciiTextJustification::Left.as_str(), "LEFT");
        assert_eq!(AsciiTextJustification::Center.as_str(), "CENTER");
        assert_eq!(AsciiTextJustification::Right.as_str(), "RIGHT");
    }

    #[test]
    fn font_default() {
        let font = AsciiTextFont::default();
        assert_eq!(font.name, "SERIF");
    }

    #[test]
    fn set_name() {
        let mut text = VrmlAsciiText::new(Some("Old"));
        text.set_name("New");
        assert_eq!(text.name(), "New");
    }
}
