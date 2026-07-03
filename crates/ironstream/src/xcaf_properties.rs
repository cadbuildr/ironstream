// FILE: rust/ironstream/crates/ironstream/src/xcaf_properties.rs

// occt: XCAFPrs_Style — presentation style applied to an XDE shape (color + transparency)
#[derive(Clone, Debug, PartialEq)]
pub struct PrsStyle {
    pub color: [f64; 3],
    pub transparency: f64,
    pub is_set_color: bool,
}

impl PrsStyle {
    /// Create a default PrsStyle with no color set and full opacity.
    pub fn new() -> Self {
        Self {
            color: [0.0, 0.0, 0.0],
            transparency: 0.0,
            is_set_color: false,
        }
    }

    /// Set the RGB color and mark the color as active.
    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        self.color = [r, g, b];
        self.is_set_color = true;
    }

    /// Set the transparency in the range [0.0, 1.0] where 0 is fully opaque.
    pub fn set_transparency(&mut self, t: f64) {
        self.transparency = t;
    }

    /// Return true if a color has been explicitly assigned.
    pub fn has_color(&self) -> bool {
        self.is_set_color
    }
}

impl Default for PrsStyle {
    fn default() -> Self {
        Self::new()
    }
}

// occt: XCAFDoc_DocumentTool — generic key/value property attached to an XDE label
#[derive(Clone, Debug, PartialEq)]
pub struct XcafProperty {
    pub name: String,
    pub value: String,
}

impl XcafProperty {
    /// Create a new property with the given name and value.
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_owned(),
            value: value.to_owned(),
        }
    }
}

// occt: XCAFDoc_DocumentTool — tool that manages a collection of XcafProperty entries
pub struct PropertyTool {
    pub properties: Vec<XcafProperty>,
}

impl PropertyTool {
    /// Create an empty PropertyTool.
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
        }
    }

    /// Append a property to the collection.
    pub fn add(&mut self, p: XcafProperty) {
        self.properties.push(p);
    }

    /// Find the first property whose name matches, returning a shared reference.
    pub fn find(&self, name: &str) -> Option<&XcafProperty> {
        self.properties.iter().find(|p| p.name == name)
    }

    /// Return the total number of properties stored.
    pub fn count(&self) -> usize {
        self.properties.len()
    }
}

impl Default for PropertyTool {
    fn default() -> Self {
        Self::new()
    }
}

/// Return a PrsStyle representing the OCCT default presentation: white, fully opaque.
// occt: XCAFPrs_Style
pub fn default_style() -> PrsStyle {
    let mut s = PrsStyle::new();
    s.set_color(1.0, 1.0, 1.0);
    s.set_transparency(0.0);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_style_new_has_no_color() {
        let s = PrsStyle::new();
        assert!(!s.has_color());
        assert!((s.transparency - 0.0).abs() < 1e-12);
    }

    #[test]
    fn prs_style_set_color_marks_flag() {
        let mut s = PrsStyle::new();
        s.set_color(0.2, 0.4, 0.8);
        assert!(s.has_color());
        assert!((s.color[0] - 0.2).abs() < 1e-12);
        assert!((s.color[1] - 0.4).abs() < 1e-12);
        assert!((s.color[2] - 0.8).abs() < 1e-12);
    }

    #[test]
    fn prs_style_set_transparency() {
        let mut s = PrsStyle::new();
        s.set_transparency(0.5);
        assert!((s.transparency - 0.5).abs() < 1e-12);
    }

    #[test]
    fn xcaf_property_new_stores_fields() {
        let p = XcafProperty::new("Author", "Alice");
        assert_eq!(p.name, "Author");
        assert_eq!(p.value, "Alice");
    }

    #[test]
    fn property_tool_add_and_count() {
        let mut t = PropertyTool::new();
        assert_eq!(t.count(), 0);
        t.add(XcafProperty::new("k1", "v1"));
        t.add(XcafProperty::new("k2", "v2"));
        assert_eq!(t.count(), 2);
    }

    #[test]
    fn property_tool_find_existing() {
        let mut t = PropertyTool::new();
        t.add(XcafProperty::new("material", "steel"));
        let found = t.find("material");
        assert!(found.is_some());
        assert_eq!(found.unwrap().value, "steel");
    }

    #[test]
    fn property_tool_find_missing_returns_none() {
        let t = PropertyTool::new();
        assert!(t.find("nope").is_none());
    }

    #[test]
    fn property_tool_find_returns_first_match() {
        let mut t = PropertyTool::new();
        t.add(XcafProperty::new("key", "first"));
        t.add(XcafProperty::new("key", "second"));
        assert_eq!(t.find("key").unwrap().value, "first");
    }

    #[test]
    fn default_style_has_white_color() {
        let s = default_style();
        assert!(s.has_color());
        assert!((s.color[0] - 1.0).abs() < 1e-12);
        assert!((s.color[1] - 1.0).abs() < 1e-12);
        assert!((s.color[2] - 1.0).abs() < 1e-12);
        assert!((s.transparency - 0.0).abs() < 1e-12);
    }
}
