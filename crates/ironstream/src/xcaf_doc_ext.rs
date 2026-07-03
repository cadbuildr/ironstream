// FILE: xcaf_doc_ext.rs
// occt: XCAFDoc_DocumentTool, XCAFDoc_ColorTool (extended), XCAFDoc_ShapeTool (extended)

use std::collections::HashMap;

/// Kind of XCAF tool attribute.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XcafToolKind {
    Shape,
    Color,
    Layer,
    DimTol,
    Material,
    ViewTool,
    NotesTool,
    Unknown,
}

impl Default for XcafToolKind {
    fn default() -> Self { Self::Unknown }
}

/// Status from document tool operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocToolStatus {
    Ok,
    DocumentNull,
    ToolNotFound,
    LabelNull,
    AlreadyExists,
}

impl Default for DocToolStatus {
    fn default() -> Self { Self::Ok }
}

impl DocToolStatus {
    pub fn is_ok(&self) -> bool { *self == DocToolStatus::Ok }
}

// occt: XCAFDoc_DocumentTool — creates and retrieves standard XCAF tools on a TDocStd_Document
#[derive(Clone, Debug, Default)]
pub struct XcafDocumentTool {
    pub doc_id: u32,
    pub shape_tool_label: Option<u32>,
    pub color_tool_label: Option<u32>,
    pub layer_tool_label: Option<u32>,
    pub dim_tol_tool_label: Option<u32>,
    pub material_tool_label: Option<u32>,
    pub view_tool_label: Option<u32>,
}

impl XcafDocumentTool {
    pub fn new(doc_id: u32) -> Self {
        Self { doc_id, ..Default::default() }
    }

    pub fn shape_tool(&mut self) -> u32 {
        *self.shape_tool_label.get_or_insert(1001)
    }

    pub fn color_tool(&mut self) -> u32 {
        *self.color_tool_label.get_or_insert(1002)
    }

    pub fn layer_tool(&mut self) -> u32 {
        *self.layer_tool_label.get_or_insert(1003)
    }

    pub fn dim_tol_tool(&mut self) -> u32 {
        *self.dim_tol_tool_label.get_or_insert(1004)
    }

    pub fn material_tool(&mut self) -> u32 {
        *self.material_tool_label.get_or_insert(1005)
    }

    pub fn view_tool(&mut self) -> u32 {
        *self.view_tool_label.get_or_insert(1006)
    }

    pub fn is_xcaf_document(&self) -> bool { self.doc_id > 0 }
    pub fn has_shape_tool(&self) -> bool { self.shape_tool_label.is_some() }
    pub fn has_color_tool(&self) -> bool { self.color_tool_label.is_some() }
}

// occt: XCAFDoc_ColorTool extended — assigns/retrieves colors on shapes
#[derive(Clone, Debug, Default)]
pub struct XcafColorToolExt {
    pub label: u32,
    /// shape_label → (surface_color_label, curve_color_label)
    pub color_map: HashMap<u32, (Option<u32>, Option<u32>)>,
    pub colors: HashMap<u32, [f32; 4]>,   // label → RGBA
    next_color_label: u32,
}

impl XcafColorToolExt {
    pub fn new(label: u32) -> Self {
        Self { label, next_color_label: 2001, ..Default::default() }
    }

    pub fn add_color(&mut self, rgba: [f32; 4]) -> u32 {
        let lbl = self.next_color_label;
        self.next_color_label += 1;
        self.colors.insert(lbl, rgba);
        lbl
    }

    pub fn set_color(&mut self, shape_label: u32, color_label: u32, surface: bool) {
        let entry = self.color_map.entry(shape_label).or_default();
        if surface { entry.0 = Some(color_label); } else { entry.1 = Some(color_label); }
    }

    pub fn get_color(&self, shape_label: u32, surface: bool) -> Option<u32> {
        self.color_map.get(&shape_label).and_then(|e| if surface { e.0 } else { e.1 })
    }

    pub fn get_rgba(&self, color_label: u32) -> Option<[f32; 4]> {
        self.colors.get(&color_label).copied()
    }

    pub fn unset_color(&mut self, shape_label: u32, surface: bool) {
        if let Some(entry) = self.color_map.get_mut(&shape_label) {
            if surface { entry.0 = None; } else { entry.1 = None; }
        }
    }

    pub fn is_set(&self, shape_label: u32) -> bool {
        self.color_map.get(&shape_label).map(|e| e.0.is_some() || e.1.is_some()).unwrap_or(false)
    }

    pub fn nb_colors(&self) -> usize { self.colors.len() }
    pub fn nb_colored_shapes(&self) -> usize {
        self.color_map.values().filter(|e| e.0.is_some() || e.1.is_some()).count()
    }

    pub fn get_colors(&self) -> Vec<u32> { self.colors.keys().copied().collect() }
}

// occt: XCAFDoc_ShapeTool extended — BOM traversal, free shapes, components
#[derive(Clone, Debug, Default)]
pub struct XcafShapeToolExt {
    pub label: u32,
    pub shapes: HashMap<u32, ShapeEntry>,
    next_label: u32,
}

#[derive(Clone, Debug, Default)]
pub struct ShapeEntry {
    pub label: u32,
    pub name: String,
    pub is_free: bool,
    pub is_simple: bool,
    pub is_assembly: bool,
    pub component_labels: Vec<u32>,
    pub referred_label: Option<u32>,
    pub shape_type: u8,   // 0=compound,1=solid,2=shell,3=face,4=edge,5=vertex
}

impl XcafShapeToolExt {
    pub fn new(label: u32) -> Self {
        Self { label, next_label: 3001, ..Default::default() }
    }

    pub fn add_shape(&mut self, name: &str, is_free: bool, shape_type: u8) -> u32 {
        let lbl = self.next_label;
        self.next_label += 1;
        self.shapes.insert(lbl, ShapeEntry {
            label: lbl,
            name: name.to_owned(),
            is_free,
            is_simple: true,
            is_assembly: false,
            component_labels: Vec::new(),
            referred_label: None,
            shape_type,
        });
        lbl
    }

    pub fn make_assembly(&mut self, label: u32) {
        if let Some(e) = self.shapes.get_mut(&label) {
            e.is_assembly = true;
            e.is_simple = false;
        }
    }

    pub fn add_component(&mut self, assembly_label: u32, component_label: u32) -> bool {
        if let Some(e) = self.shapes.get_mut(&assembly_label) {
            e.component_labels.push(component_label);
            true
        } else {
            false
        }
    }

    pub fn shape(&self, label: u32) -> Option<&ShapeEntry> { self.shapes.get(&label) }

    pub fn free_shapes(&self) -> Vec<u32> {
        self.shapes.values().filter(|e| e.is_free).map(|e| e.label).collect()
    }

    pub fn nb_shapes(&self) -> usize { self.shapes.len() }
    pub fn nb_free_shapes(&self) -> usize { self.shapes.values().filter(|e| e.is_free).count() }
    pub fn is_assembly(&self, label: u32) -> bool { self.shapes.get(&label).map(|e| e.is_assembly).unwrap_or(false) }
    pub fn is_free(&self, label: u32) -> bool { self.shapes.get(&label).map(|e| e.is_free).unwrap_or(false) }

    pub fn get_components(&self, assembly_label: u32) -> Vec<u32> {
        self.shapes.get(&assembly_label).map(|e| e.component_labels.clone()).unwrap_or_default()
    }

    pub fn find_by_name(&self, name: &str) -> Option<u32> {
        self.shapes.values().find(|e| e.name == name).map(|e| e.label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_tool_creates_sub_tools() {
        let mut dt = XcafDocumentTool::new(42);
        assert!(dt.is_xcaf_document());
        let st = dt.shape_tool();
        let ct = dt.color_tool();
        assert_ne!(st, ct);
        assert!(dt.has_shape_tool());
        assert!(dt.has_color_tool());
    }

    #[test]
    fn color_tool_add_and_retrieve() {
        let mut ct = XcafColorToolExt::new(1002);
        let lbl = ct.add_color([1.0, 0.0, 0.0, 1.0]);
        ct.set_color(10, lbl, true);
        assert_eq!(ct.get_color(10, true), Some(lbl));
        assert!(ct.is_set(10));
        let rgba = ct.get_rgba(lbl).unwrap();
        assert!((rgba[0] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn color_tool_unset() {
        let mut ct = XcafColorToolExt::new(1002);
        let lbl = ct.add_color([0.0, 1.0, 0.0, 1.0]);
        ct.set_color(20, lbl, false);
        assert!(ct.is_set(20));
        ct.unset_color(20, false);
        assert!(!ct.is_set(20));
    }

    #[test]
    fn shape_tool_assembly() {
        let mut st = XcafShapeToolExt::new(1001);
        let asm = st.add_shape("Assembly1", true, 0);
        let part1 = st.add_shape("Part1", false, 1);
        let part2 = st.add_shape("Part2", false, 1);
        st.make_assembly(asm);
        st.add_component(asm, part1);
        st.add_component(asm, part2);
        assert!(st.is_assembly(asm));
        assert_eq!(st.get_components(asm).len(), 2);
        assert_eq!(st.free_shapes(), vec![asm]);
    }

    #[test]
    fn shape_tool_find_by_name() {
        let mut st = XcafShapeToolExt::new(1001);
        st.add_shape("TestPart", true, 1);
        assert!(st.find_by_name("TestPart").is_some());
        assert!(st.find_by_name("Missing").is_none());
    }

    #[test]
    fn doc_tool_status() {
        let s = DocToolStatus::Ok;
        assert!(s.is_ok());
        let e = DocToolStatus::ToolNotFound;
        assert!(!e.is_ok());
    }
}
