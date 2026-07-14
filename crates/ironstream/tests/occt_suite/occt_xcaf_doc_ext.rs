extern crate ironstream;
use ironstream::xcaf_doc_ext::*;

#[test]
fn xcaf_document_tool_layer_tool_returns_non_zero() {
    let mut dt = XcafDocumentTool::new(1);
    let lt = dt.layer_tool();
    assert!(lt > 0);
    // Subsequent calls return the same label
    assert_eq!(dt.layer_tool(), lt);
    // Layer tool label is different from shape and color tool labels
    let st = dt.shape_tool();
    let ct = dt.color_tool();
    assert_ne!(lt, st);
    assert_ne!(lt, ct);
}

#[test]
fn xcaf_color_tool_ext_get_colors_lists_all_added_colors() {
    let mut ct = XcafColorToolExt::new(1002);
    let lbl1 = ct.add_color([1.0, 0.0, 0.0, 1.0]);
    let lbl2 = ct.add_color([0.0, 1.0, 0.0, 1.0]);
    let lbl3 = ct.add_color([0.0, 0.0, 1.0, 1.0]);
    let colors = ct.get_colors();
    assert_eq!(colors.len(), 3);
    assert!(colors.contains(&lbl1));
    assert!(colors.contains(&lbl2));
    assert!(colors.contains(&lbl3));
    assert_eq!(ct.nb_colors(), 3);
}

#[test]
fn xcaf_color_tool_ext_nb_colored_shapes_counts() {
    let mut ct = XcafColorToolExt::new(1002);
    let red = ct.add_color([1.0, 0.0, 0.0, 1.0]);
    let blue = ct.add_color([0.0, 0.0, 1.0, 1.0]);
    assert_eq!(ct.nb_colored_shapes(), 0);
    ct.set_color(10, red, true);
    assert_eq!(ct.nb_colored_shapes(), 1);
    ct.set_color(20, blue, false);
    assert_eq!(ct.nb_colored_shapes(), 2);
    // Setting another color on an already-colored shape should not double-count
    ct.set_color(10, blue, false);
    assert_eq!(ct.nb_colored_shapes(), 2);
}

#[test]
fn xcaf_shape_tool_ext_nb_shapes_and_nb_free_shapes() {
    let mut st = XcafShapeToolExt::new(1001);
    assert_eq!(st.nb_shapes(), 0);
    assert_eq!(st.nb_free_shapes(), 0);
    st.add_shape("FreeShape1", true, 1);
    st.add_shape("FreeShape2", true, 1);
    st.add_shape("Component", false, 1);
    assert_eq!(st.nb_shapes(), 3);
    assert_eq!(st.nb_free_shapes(), 2);
}

#[test]
fn xcaf_shape_tool_ext_is_free_returns_correct_result() {
    let mut st = XcafShapeToolExt::new(1001);
    let free_lbl = st.add_shape("TopLevel", true, 0);
    let comp_lbl = st.add_shape("Part", false, 1);
    assert!(st.is_free(free_lbl));
    assert!(!st.is_free(comp_lbl));
    // Non-existent label
    assert!(!st.is_free(9999));
}

#[test]
fn doc_tool_status_document_null_is_not_ok() {
    let bad = DocToolStatus::DocumentNull;
    assert!(!bad.is_ok());
    assert_ne!(bad, DocToolStatus::Ok);
    // All error variants are not ok
    assert!(!DocToolStatus::ToolNotFound.is_ok());
    assert!(!DocToolStatus::LabelNull.is_ok());
    assert!(!DocToolStatus::AlreadyExists.is_ok());
    // Only Ok is ok
    assert!(DocToolStatus::Ok.is_ok());
}
