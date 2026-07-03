use ironstream::xcaf_ext::*;

#[test]
fn xcaf_color_clamp() {
    let c = XcafColor::rgb(2.0, -1.0, 0.5);
    assert!((c.r - 1.0).abs() < 1e-10, "r > 1 should clamp to 1");
    assert!((c.g - 0.0).abs() < 1e-10, "g < 0 should clamp to 0");
    assert!((c.b - 0.5).abs() < 1e-10);
    assert!(c.is_opaque(), "RGB constructor should set full opacity");
}

#[test]
fn xcaf_document_add_color_dedup() {
    let mut doc = XcafDocumentTool::new();
    let c = XcafColor::rgb(1.0, 0.0, 0.0);
    let i1 = doc.add_color(c);
    let i2 = doc.add_color(c);
    assert_eq!(i1, i2, "same color added twice should return same index");
    assert_eq!(doc.nb_colors(), 1);
}

#[test]
fn xcaf_set_and_get_color() {
    let mut doc = XcafDocumentTool::new();
    doc.set_color(10, XcafColor::rgb(0.0, 1.0, 0.0));
    let c = doc.get_color(10).unwrap();
    assert!((c.g - 1.0).abs() < 1e-10);
    assert!(doc.get_color(99).is_none(), "unknown shape should return None");
}

#[test]
fn xcaf_layer_and_material() {
    let mut doc = XcafDocumentTool::new();
    doc.add_layer(XcafLayer::new(1, "Layer0"));
    doc.add_layer(XcafLayer::new(1, "Layer0")); // duplicate
    assert_eq!(doc.nb_layers(), 1, "duplicate layer should not be added");

    let mut mat = XcafMaterial::new(1, "Steel");
    mat.set_density(7800.0, "kg/m3");
    doc.add_material(mat);
    assert_eq!(doc.nb_materials(), 1);
}

#[test]
fn xcaf_shape_layer() {
    let mut doc = XcafDocumentTool::new();
    doc.add_layer(XcafLayer::new(5, "Walls"));
    doc.set_layer(100, 5);
    assert_eq!(doc.get_layer(100), Some(5));
    doc.set_layer(100, 6); // override
    assert_eq!(doc.get_layer(100), Some(6));
    assert!(doc.get_layer(999).is_none());
}

#[test]
fn xcaf_rgba_transparency() {
    let c = XcafColor::rgba(0.5, 0.5, 0.5, 0.3);
    assert!(!c.is_opaque(), "alpha 0.3 should not be opaque");
    let arr = c.to_array();
    assert!((arr[3] - 0.3).abs() < 1e-10);
}
