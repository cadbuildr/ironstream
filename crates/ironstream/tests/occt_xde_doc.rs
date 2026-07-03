// FILE: tests/occt_xde_doc.rs
extern crate ironstream;

use ironstream::top_exp::{
    TopoDS_Edge, TopoDS_Face, TopoDS_Shape, TopoDS_Shell, TopoDS_Solid, TopoDS_Vertex, TopoDS_Wire,
};
use ironstream::xde_doc::{
    Color, ColorKind, Label, XCAFDoc_ColorTool, XCAFDoc_DocumentTool, XCAFDoc_LayerTool,
    XCAFDoc_ShapeTool,
};

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn vertex(id: u64) -> TopoDS_Shape {
    TopoDS_Shape::Vertex(TopoDS_Vertex {
        id,
        x: id as f64,
        y: 0.0,
        z: 0.0,
    })
}

fn edge(id: u64) -> TopoDS_Shape {
    TopoDS_Shape::Edge(TopoDS_Edge {
        id,
        vertices: vec![],
    })
}

fn wire(id: u64) -> TopoDS_Shape {
    TopoDS_Shape::Wire(TopoDS_Wire {
        id,
        edges: vec![],
    })
}

fn face(id: u64) -> TopoDS_Shape {
    TopoDS_Shape::Face(TopoDS_Face {
        id,
        wires: vec![],
    })
}

fn solid(id: u64) -> TopoDS_Shape {
    TopoDS_Shape::Solid(TopoDS_Solid {
        id,
        shells: vec![TopoDS_Shell {
            id,
            faces: vec![],
        }],
    })
}

// ---------------------------------------------------------------------------
// XCAFDoc_DocumentTool — document-level tests
// ---------------------------------------------------------------------------

/// A freshly created document is empty.
#[test]
fn document_tool_new_is_empty() {
    let doc = XCAFDoc_DocumentTool::new();
    assert_eq!(doc.shape_tool().nb_shapes(), 0);
    assert_eq!(doc.color_tool().nb_colors(), 0);
    assert_eq!(doc.layer_tool().nb_layers(), 0);
}

/// The document title can be set and retrieved.
#[test]
fn document_tool_title_roundtrip() {
    let doc = XCAFDoc_DocumentTool::with_title("MyModel");
    assert_eq!(doc.title(), "MyModel");
}

/// The doc_label is non-null.
#[test]
fn document_tool_doc_label_is_non_null() {
    let doc = XCAFDoc_DocumentTool::new();
    assert!(!doc.doc_label().is_null());
}

/// add_shape convenience wrapper delegates to ShapeTool correctly.
#[test]
fn document_tool_add_shape_delegates() {
    let mut doc = XCAFDoc_DocumentTool::new();
    let lbl = doc.add_shape(vertex(1));
    assert!(!lbl.is_null());
    assert_eq!(doc.shape_tool().nb_shapes(), 1);
}

/// set_color convenience wrapper assigns a color visible via the color tool.
#[test]
fn document_tool_set_color_visible_via_tool() {
    let mut doc = XCAFDoc_DocumentTool::new();
    let lbl = doc.add_shape(face(10));
    doc.set_color(lbl, Color::red(), ColorKind::Surf);
    let color = doc.color_tool().get_color(lbl, ColorKind::Surf).unwrap();
    assert!((color.r - 1.0).abs() < 1e-9);
    assert!((color.g).abs() < 1e-9);
    assert!((color.b).abs() < 1e-9);
}

/// assign_layer convenience wrapper creates a layer and assigns the shape.
#[test]
fn document_tool_assign_layer_creates_and_assigns() {
    let mut doc = XCAFDoc_DocumentTool::new();
    let shape_lbl = doc.add_shape(solid(20));
    let layer_lbl = doc.assign_layer(shape_lbl, "Structural");
    assert!(!layer_lbl.is_null());
    assert_eq!(doc.layer_tool().get_layer_name(layer_lbl), Some("Structural"));
    assert!(doc.layer_tool().is_set(shape_lbl, layer_lbl));
}

/// summary() produces a non-empty string containing shape/color/layer counts.
#[test]
fn document_tool_summary_contains_counts() {
    let mut doc = XCAFDoc_DocumentTool::with_title("Test");
    doc.add_shape(vertex(1));
    doc.add_shape(edge(2));
    let s = doc.summary();
    assert!(s.contains("2 shapes"), "summary was: {}", s);
    assert!(s.contains("Test"), "summary was: {}", s);
}

// ---------------------------------------------------------------------------
// XCAFDoc_ShapeTool — shape hierarchy tests
// ---------------------------------------------------------------------------

/// Adding two distinct shapes gives each a unique label.
#[test]
fn shape_tool_two_distinct_shapes_get_distinct_labels() {
    let mut st = XCAFDoc_ShapeTool::new();
    let l1 = st.add_shape(vertex(1));
    let l2 = st.add_shape(vertex(2));
    assert_ne!(l1, l2);
    assert_eq!(st.nb_shapes(), 2);
}

/// Named shapes expose their name via get_name.
#[test]
fn shape_tool_named_shape_roundtrip() {
    let mut st = XCAFDoc_ShapeTool::new();
    let lbl = st.add_named_shape(face(99), "Flange");
    assert_eq!(st.get_name(lbl), Some("Flange"));
}

/// Components with a location store and retrieve the location offset.
#[test]
fn shape_tool_component_location_roundtrip() {
    use ironstream::gp::Pnt;
    let mut st = XCAFDoc_ShapeTool::new();
    let asm_lbl = st.add_shape(solid(1));
    let comp_lbl = st.add_shape(vertex(2));
    let offset = Pnt::new(10.0, 20.0, 30.0);
    st.add_component(asm_lbl, comp_lbl, Some(offset));
    let loc = st.get_location(comp_lbl);
    assert!((loc.x - 10.0).abs() < 1e-9);
    assert!((loc.y - 20.0).abs() < 1e-9);
    assert!((loc.z - 30.0).abs() < 1e-9);
}

// ---------------------------------------------------------------------------
// XCAFDoc_ColorTool — color management tests
// ---------------------------------------------------------------------------

/// All three color kinds can be independently assigned to the same label.
#[test]
fn color_tool_three_kinds_independent() {
    let mut ct = XCAFDoc_ColorTool::new();
    let shape_lbl = Label(1);
    ct.set_color(shape_lbl, Color::red(), ColorKind::Gen);
    ct.set_color(shape_lbl, Color::green(), ColorKind::Surf);
    ct.set_color(shape_lbl, Color::blue(), ColorKind::Curv);

    let gen = ct.get_color(shape_lbl, ColorKind::Gen).unwrap();
    let surf = ct.get_color(shape_lbl, ColorKind::Surf).unwrap();
    let curv = ct.get_color(shape_lbl, ColorKind::Curv).unwrap();

    assert!((gen.r - 1.0).abs() < 1e-9);
    assert!((surf.g - 1.0).abs() < 1e-9);
    assert!((curv.b - 1.0).abs() < 1e-9);
}

/// get_colors returns one label per distinct color, sorted by label id.
#[test]
fn color_tool_get_colors_sorted() {
    let mut ct = XCAFDoc_ColorTool::new();
    ct.add_color(Color::red());
    ct.add_color(Color::green());
    ct.add_color(Color::blue());
    let labels = ct.get_colors();
    assert_eq!(labels.len(), 3);
    // Must be sorted ascending by id.
    assert!(labels[0].0 < labels[1].0 && labels[1].0 < labels[2].0);
}

/// all_assignments returns every (shape, kind, color) triple.
#[test]
fn color_tool_all_assignments_roundtrip() {
    let mut ct = XCAFDoc_ColorTool::new();
    let s1 = Label(10);
    let s2 = Label(20);
    ct.set_color(s1, Color::red(), ColorKind::Surf);
    ct.set_color(s2, Color::blue(), ColorKind::Curv);
    let all = ct.all_assignments();
    assert_eq!(all.len(), 2);
}

// ---------------------------------------------------------------------------
// XCAFDoc_LayerTool — layer management tests
// ---------------------------------------------------------------------------

/// Removing a layer also removes all shape assignments to that layer.
#[test]
fn layer_tool_remove_clears_assignments() {
    let mut lt = XCAFDoc_LayerTool::new();
    let lyr = lt.add_layer("Temp");
    let shape_lbl = Label(5);
    lt.set_layer(shape_lbl, lyr);
    assert!(lt.is_set(shape_lbl, lyr));
    lt.remove_layer(lyr);
    assert!(!lt.is_set(shape_lbl, lyr));
    assert_eq!(lt.nb_layers(), 0);
}

/// shapes_on_layer returns all shapes assigned to a given layer.
#[test]
fn layer_tool_shapes_on_layer() {
    let mut lt = XCAFDoc_LayerTool::new();
    let lyr = lt.add_layer("Visible");
    let s1 = Label(1);
    let s2 = Label(2);
    let s3 = Label(3);
    lt.set_layer(s1, lyr);
    lt.set_layer(s2, lyr);
    let on_layer = lt.shapes_on_layer(lyr);
    assert!(on_layer.contains(&s1));
    assert!(on_layer.contains(&s2));
    assert!(!on_layer.contains(&s3));
}

/// A shape can belong to multiple layers simultaneously.
#[test]
fn layer_tool_shape_on_multiple_layers() {
    let mut lt = XCAFDoc_LayerTool::new();
    let lyr1 = lt.add_layer("A");
    let lyr2 = lt.add_layer("B");
    let shape_lbl = Label(7);
    lt.set_layer(shape_lbl, lyr1);
    lt.set_layer(shape_lbl, lyr2);
    let layers = lt.get_shape_layers(shape_lbl);
    assert_eq!(layers.len(), 2);
    assert!(layers.contains(&lyr1));
    assert!(layers.contains(&lyr2));
}
