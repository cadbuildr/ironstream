// FILE: rust/ironstream/crates/ironstream/src/xcaf_color.rs

// occt-ref: XCAFDoc_ColorType // — distinguishes how a color is applied to a shape
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XcafColorType {
    /// General color, applied to the whole shape
    Gen,
    /// Surface color only
    Surf,
    /// Curve (edge/wire) color only
    Curve,
}

// occt-ref: XCAFDoc_Color // — stores a named color entry with RGBA and color type
#[derive(Clone, Debug)]
pub struct XcafColorEntry {
    label: String,
    rgba: [f32; 4],
    color_type: XcafColorType,
}

impl XcafColorEntry {
    /// Create a new color entry.
    pub fn new(label: impl Into<String>, r: f32, g: f32, b: f32, a: f32, color_type: XcafColorType) -> Self {
        Self {
            label: label.into(),
            rgba: [r, g, b, a],
            color_type,
        }
    }

    /// Return the label string for this entry.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Return the RGBA values as a 4-element array.
    pub fn rgba(&self) -> [f32; 4] {
        self.rgba
    }

    /// Replace the RGBA values.
    pub fn set_rgba(&mut self, rgba: [f32; 4]) {
        self.rgba = rgba;
    }

    /// Return the color type (Gen, Surf, or Curve).
    pub fn color_type(&self) -> XcafColorType {
        self.color_type
    }
}

// occt-ref: XCAFDoc_ColorTool // — manages a collection of color entries indexed by position
pub struct XcafColorTool {
    entries: Vec<XcafColorEntry>,
}

impl XcafColorTool {
    /// Create an empty color tool.
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// Add a color entry and return its index.
    pub fn add_color(&mut self, entry: XcafColorEntry) -> usize {
        let idx = self.entries.len();
        self.entries.push(entry);
        idx
    }

    /// Retrieve a color entry by index.
    pub fn get_color(&self, idx: usize) -> Option<&XcafColorEntry> {
        self.entries.get(idx)
    }

    /// Return the total number of color entries.
    pub fn nb_colors(&self) -> usize {
        self.entries.len()
    }

    /// Find the index of the first entry whose RGBA values all match within 1e-4.
    pub fn find_color(&self, rgba: [f32; 4]) -> Option<usize> {
        const TOL: f32 = 1e-4;
        self.entries.iter().position(|e| {
            (e.rgba[0] - rgba[0]).abs() < TOL
                && (e.rgba[1] - rgba[1]).abs() < TOL
                && (e.rgba[2] - rgba[2]).abs() < TOL
                && (e.rgba[3] - rgba[3]).abs() < TOL
        })
    }

    /// Remove the entry at the given index. Returns true if the index was valid.
    pub fn remove_color(&mut self, idx: usize) -> bool {
        if idx < self.entries.len() {
            self.entries.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Default for XcafColorTool {
    fn default() -> Self {
        Self::new()
    }
}
