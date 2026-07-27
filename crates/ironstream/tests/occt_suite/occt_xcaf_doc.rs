// FILE: tests/occt_xcaf_doc.rs
extern crate ironstream;

use ironstream::xcaf_doc::{
    XcafDocColorType, XcafDocDocument, XcafDocRGBColor, XcafDocShapeEntry, XcafDocShapeTool,
};

#[test]
fn color_type_copy_and_eq() {
    let t = XcafDocColorType::Surface;
    let t2 = t;
    assert_eq!(t, t2);
    assert_ne!(t, XcafDocColorType::Curve);
}

#[test]
fn rgb_color_field_access() {
    let c = XcafDocRGBColor::new(0.2, 0.4, 0.6);
    assert!((c.r - 0.2).abs() < 1e-6);
    assert!((c.g - 0.4).abs() < 1e-6);
    assert!((c.b - 0.6).abs() < 1e-6);
}

#[test]
fn rgb_color_is_copy() {
    let c = XcafDocRGBColor::new(0.0, 1.0, 0.0);
    let c2 = c;
    assert_eq!(c, c2);
}

#[test]
fn shape_entry_label_path_stored() {
    let e = XcafDocShapeEntry::new(vec![1, 2, 3]);
    assert_eq!(e.label_path, vec![1, 2, 3]);
}

#[test]
fn shape_tool_empty_on_construction() {
    let st = XcafDocShapeTool::new();
    assert_eq!(st.nb_shapes(), 0);
    assert!(st.free_shapes().is_empty());
}

#[test]
fn shape_tool_add_multiple_shapes() {
    let mut st = XcafDocShapeTool::new();
    st.add_shape(vec![0, 1]);
    st.add_shape(vec![0, 2]);
    st.add_shape(vec![0, 3]);
    assert_eq!(st.nb_shapes(), 3);
}

#[test]
fn shape_tool_name_round_trip() {
    let mut st = XcafDocShapeTool::new();
    let idx = st.add_shape(vec![0]);
    st.set_name(idx, "Chassis");
    assert_eq!(st.get_name(idx), Some("Chassis"));
}

#[test]
fn shape_tool_color_round_trip() {
    let mut st = XcafDocShapeTool::new();
    let idx = st.add_shape(vec![0]);
    st.set_color(idx, XcafDocRGBColor::new(0.0, 0.0, 1.0));
    let c = st.get_color(idx).expect("color should be set");
    assert!((c.b - 1.0).abs() < 1e-6);
}

#[test]
fn shape_tool_free_shapes_all_free_by_default() {
    let mut st = XcafDocShapeTool::new();
    st.add_shape(vec![0]);
    st.add_shape(vec![1]);
    let free = st.free_shapes();
    assert_eq!(free.len(), 2);
}

#[test]
fn shape_tool_free_shapes_after_marking_non_free() {
    let mut st = XcafDocShapeTool::new();
    let i0 = st.add_shape(vec![0]);
    let i1 = st.add_shape(vec![1]);
    let i2 = st.add_shape(vec![2]);
    if let Some(e) = st.shape_mut(i0) {
        e.is_free = false;
    }
    let free = st.free_shapes();
    assert!(!free.contains(&i0));
    assert!(free.contains(&i1));
    assert!(free.contains(&i2));
}

#[test]
fn shape_tool_assembly_flag() {
    let mut st = XcafDocShapeTool::new();
    let idx = st.add_shape(vec![0]);
    assert!(!st.shape(idx).unwrap().is_assembly);
    if let Some(e) = st.shape_mut(idx) {
        e.is_assembly = true;
    }
    assert!(st.shape(idx).unwrap().is_assembly);
}

#[test]
fn document_format_accessor() {
    let doc = XcafDocDocument::new("BinXCAF");
    assert_eq!(doc.format(), "BinXCAF");
}

#[test]
fn document_shape_tool_integration() {
    let mut doc = XcafDocDocument::new("STEP");
    let idx = doc.shape_tool_mut().add_shape(vec![0, 1]);
    doc.shape_tool_mut().set_name(idx, "Wing");
    doc.shape_tool_mut().set_color(idx, XcafDocRGBColor::new(0.5, 0.5, 0.5));
    assert_eq!(doc.shape_tool().get_name(idx), Some("Wing"));
    let c = doc.shape_tool().get_color(idx).unwrap();
    assert!((c.r - 0.5).abs() < 1e-6);
    assert_eq!(doc.shape_tool().nb_shapes(), 1);
}

#[test]
fn document_free_shapes_reflects_shape_tool() {
    let mut doc = XcafDocDocument::new("IGES");
    doc.shape_tool_mut().add_shape(vec![0]);
    doc.shape_tool_mut().add_shape(vec![1]);
    let i2 = doc.shape_tool_mut().add_shape(vec![2]);
    if let Some(e) = doc.shape_tool_mut().shape_mut(i2) {
        e.is_free = false;
    }
    let free = doc.shape_tool().free_shapes();
    assert_eq!(free.len(), 2);
    assert!(!free.contains(&i2));
}
