// FILE: xcaf_doc.rs

// occt: XCAFDoc_ColorType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XcafDocColorType {
    Generic,
    Surface,
    Curve,
}

// occt: XCAFDoc_ColorTool color
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XcafDocRGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl XcafDocRGBColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

// occt: XCAFDoc_ShapeTool entry
pub struct XcafDocShapeEntry {
    pub label_path: Vec<u32>,
    pub name: Option<String>,
    pub color: Option<XcafDocRGBColor>,
    pub is_free: bool,
    pub is_assembly: bool,
}

impl XcafDocShapeEntry {
    pub fn new(label_path: Vec<u32>) -> Self {
        Self {
            label_path,
            name: None,
            color: None,
            is_free: true,
            is_assembly: false,
        }
    }
}

// occt: XCAFDoc_ShapeTool
pub struct XcafDocShapeTool {
    entries: Vec<XcafDocShapeEntry>,
}

impl XcafDocShapeTool {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_shape(&mut self, path: Vec<u32>) -> usize {
        let idx = self.entries.len();
        self.entries.push(XcafDocShapeEntry::new(path));
        idx
    }

    pub fn nb_shapes(&self) -> usize {
        self.entries.len()
    }

    pub fn shape(&self, idx: usize) -> Option<&XcafDocShapeEntry> {
        self.entries.get(idx)
    }

    pub fn shape_mut(&mut self, idx: usize) -> Option<&mut XcafDocShapeEntry> {
        self.entries.get_mut(idx)
    }

    pub fn set_name(&mut self, idx: usize, name: &str) {
        if let Some(e) = self.entries.get_mut(idx) {
            e.name = Some(name.to_owned());
        }
    }

    pub fn get_name(&self, idx: usize) -> Option<&str> {
        self.entries.get(idx)?.name.as_deref()
    }

    pub fn set_color(&mut self, idx: usize, color: XcafDocRGBColor) {
        if let Some(e) = self.entries.get_mut(idx) {
            e.color = Some(color);
        }
    }

    pub fn get_color(&self, idx: usize) -> Option<XcafDocRGBColor> {
        self.entries.get(idx)?.color
    }

    pub fn is_free(&self, idx: usize) -> bool {
        self.entries.get(idx).map_or(false, |e| e.is_free)
    }

    pub fn free_shapes(&self) -> Vec<usize> {
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_free)
            .map(|(i, _)| i)
            .collect()
    }
}

impl Default for XcafDocShapeTool {
    fn default() -> Self {
        Self::new()
    }
}

// occt: XCAFDoc document
pub struct XcafDocDocument {
    pub shape_tool: XcafDocShapeTool,
    pub format: String,
}

impl XcafDocDocument {
    pub fn new(format: &str) -> Self {
        Self {
            shape_tool: XcafDocShapeTool::new(),
            format: format.to_owned(),
        }
    }

    pub fn format(&self) -> &str {
        &self.format
    }

    pub fn shape_tool(&self) -> &XcafDocShapeTool {
        &self.shape_tool
    }

    pub fn shape_tool_mut(&mut self) -> &mut XcafDocShapeTool {
        &mut self.shape_tool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_type_variants_are_distinct() {
        assert_ne!(XcafDocColorType::Generic, XcafDocColorType::Surface);
        assert_ne!(XcafDocColorType::Surface, XcafDocColorType::Curve);
    }

    #[test]
    fn rgb_color_new_stores_components() {
        let c = XcafDocRGBColor::new(0.1, 0.5, 0.9);
        assert!((c.r - 0.1).abs() < 1e-6);
        assert!((c.g - 0.5).abs() < 1e-6);
        assert!((c.b - 0.9).abs() < 1e-6);
    }

    #[test]
    fn shape_entry_new_defaults() {
        let e = XcafDocShapeEntry::new(vec![0, 1, 2]);
        assert_eq!(e.label_path, vec![0, 1, 2]);
        assert!(e.name.is_none());
        assert!(e.color.is_none());
        assert!(e.is_free);
        assert!(!e.is_assembly);
    }

    #[test]
    fn shape_tool_add_returns_sequential_indices() {
        let mut st = XcafDocShapeTool::new();
        let i0 = st.add_shape(vec![0]);
        let i1 = st.add_shape(vec![1]);
        assert_eq!(i0, 0);
        assert_eq!(i1, 1);
        assert_eq!(st.nb_shapes(), 2);
    }

    #[test]
    fn shape_tool_set_and_get_name() {
        let mut st = XcafDocShapeTool::new();
        let idx = st.add_shape(vec![0, 1]);
        assert!(st.get_name(idx).is_none());
        st.set_name(idx, "Part-A");
        assert_eq!(st.get_name(idx), Some("Part-A"));
    }

    #[test]
    fn shape_tool_set_and_get_color() {
        let mut st = XcafDocShapeTool::new();
        let idx = st.add_shape(vec![0]);
        assert!(st.get_color(idx).is_none());
        st.set_color(idx, XcafDocRGBColor::new(1.0, 0.0, 0.0));
        let c = st.get_color(idx).unwrap();
        assert!((c.r - 1.0).abs() < 1e-6);
    }

    #[test]
    fn shape_tool_is_free_default_true() {
        let mut st = XcafDocShapeTool::new();
        let idx = st.add_shape(vec![0]);
        assert!(st.is_free(idx));
    }

    #[test]
    fn shape_tool_free_shapes_filters_correctly() {
        let mut st = XcafDocShapeTool::new();
        let i0 = st.add_shape(vec![0]);
        let i1 = st.add_shape(vec![1]);
        let i2 = st.add_shape(vec![2]);
        if let Some(e) = st.shape_mut(i1) {
            e.is_free = false;
        }
        let free = st.free_shapes();
        assert!(free.contains(&i0));
        assert!(!free.contains(&i1));
        assert!(free.contains(&i2));
    }

    #[test]
    fn shape_tool_out_of_bounds_returns_none() {
        let st = XcafDocShapeTool::new();
        assert!(st.shape(99).is_none());
        assert!(st.get_name(99).is_none());
        assert!(st.get_color(99).is_none());
        assert!(!st.is_free(99));
    }

    #[test]
    fn document_new_stores_format() {
        let doc = XcafDocDocument::new("XDE");
        assert_eq!(doc.format(), "XDE");
        assert_eq!(doc.shape_tool().nb_shapes(), 0);
    }

    #[test]
    fn document_shape_tool_mut_add_and_query() {
        let mut doc = XcafDocDocument::new("STEP");
        let idx = doc.shape_tool_mut().add_shape(vec![0, 1]);
        doc.shape_tool_mut().set_name(idx, "Body");
        assert_eq!(doc.shape_tool().get_name(idx), Some("Body"));
    }
}
