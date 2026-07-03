// FILE: rust/ironstream/crates/ironstream/tests/occt_xcaf_material.rs

extern crate ironstream;
use ironstream::xcaf_material::*;

// --- XcafMaterial ---

#[test]
fn test_material_new_stores_label() {
    let m = XcafMaterial::new("lbl1", "Steel");
    assert_eq!(m.label(), "lbl1");
}

#[test]
fn test_material_new_stores_name() {
    let m = XcafMaterial::new("lbl1", "Steel");
    assert_eq!(m.name(), "Steel");
}

#[test]
fn test_material_new_description_empty() {
    let m = XcafMaterial::new("lbl1", "Steel");
    assert_eq!(m.description(), "");
}

#[test]
fn test_material_new_density_zero() {
    let m = XcafMaterial::new("lbl1", "Steel");
    assert_eq!(m.density(), 0.0);
}

#[test]
fn test_material_new_density_unit_empty() {
    let m = XcafMaterial::new("lbl1", "Steel");
    assert_eq!(m.density_unit(), "");
}

#[test]
fn test_material_set_description() {
    let mut m = XcafMaterial::new("lbl1", "Steel");
    m.set_description("High-strength steel alloy");
    assert_eq!(m.description(), "High-strength steel alloy");
}

#[test]
fn test_material_set_description_empty() {
    let mut m = XcafMaterial::new("lbl1", "Steel");
    m.set_description("something");
    m.set_description("");
    assert_eq!(m.description(), "");
}

#[test]
fn test_material_set_density() {
    let mut m = XcafMaterial::new("lbl1", "Steel");
    m.set_density(7850.0);
    assert!((m.density() - 7850.0).abs() < 1e-9);
}

#[test]
fn test_material_set_density_zero() {
    let mut m = XcafMaterial::new("lbl1", "Steel");
    m.set_density(7850.0);
    m.set_density(0.0);
    assert_eq!(m.density(), 0.0);
}

#[test]
fn test_material_set_density_unit() {
    let mut m = XcafMaterial::new("lbl1", "Steel");
    m.set_density_unit("kg/m^3");
    assert_eq!(m.density_unit(), "kg/m^3");
}

#[test]
fn test_material_set_density_unit_overwrite() {
    let mut m = XcafMaterial::new("lbl1", "Steel");
    m.set_density_unit("g/cm^3");
    m.set_density_unit("kg/m^3");
    assert_eq!(m.density_unit(), "kg/m^3");
}

#[test]
fn test_material_clone_is_independent() {
    let mut m = XcafMaterial::new("orig", "Aluminium");
    m.set_density(2700.0);
    m.set_density_unit("kg/m^3");
    m.set_description("Lightweight metal");
    let m2 = m.clone();
    assert_eq!(m2.label(), "orig");
    assert_eq!(m2.name(), "Aluminium");
    assert!((m2.density() - 2700.0).abs() < 1e-9);
    assert_eq!(m2.density_unit(), "kg/m^3");
    assert_eq!(m2.description(), "Lightweight metal");
}

// --- XcafMaterialTool ---

#[test]
fn test_tool_new_is_empty() {
    let tool = XcafMaterialTool::new();
    assert_eq!(tool.nb_materials(), 0);
}

#[test]
fn test_tool_default_is_empty() {
    let tool = XcafMaterialTool::default();
    assert_eq!(tool.nb_materials(), 0);
}

#[test]
fn test_tool_add_material_returns_index() {
    let mut tool = XcafMaterialTool::new();
    let m = XcafMaterial::new("lbl1", "Steel");
    let idx = tool.add_material(m);
    assert_eq!(idx, 0);
}

#[test]
fn test_tool_add_multiple_returns_sequential_indices() {
    let mut tool = XcafMaterialTool::new();
    let i0 = tool.add_material(XcafMaterial::new("l0", "Steel"));
    let i1 = tool.add_material(XcafMaterial::new("l1", "Aluminium"));
    let i2 = tool.add_material(XcafMaterial::new("l2", "Copper"));
    assert_eq!(i0, 0);
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
}

#[test]
fn test_tool_nb_materials_increments() {
    let mut tool = XcafMaterialTool::new();
    assert_eq!(tool.nb_materials(), 0);
    tool.add_material(XcafMaterial::new("l0", "Steel"));
    assert_eq!(tool.nb_materials(), 1);
    tool.add_material(XcafMaterial::new("l1", "Aluminium"));
    assert_eq!(tool.nb_materials(), 2);
}

#[test]
fn test_tool_get_material_valid_index() {
    let mut tool = XcafMaterialTool::new();
    let mut m = XcafMaterial::new("lbl1", "Titanium");
    m.set_density(4500.0);
    tool.add_material(m);
    let got = tool.get_material(0).unwrap();
    assert_eq!(got.name(), "Titanium");
    assert!((got.density() - 4500.0).abs() < 1e-9);
}

#[test]
fn test_tool_get_material_out_of_bounds_returns_none() {
    let tool = XcafMaterialTool::new();
    assert!(tool.get_material(0).is_none());
}

#[test]
fn test_tool_find_material_matches_by_name() {
    let mut tool = XcafMaterialTool::new();
    tool.add_material(XcafMaterial::new("l0", "Steel"));
    tool.add_material(XcafMaterial::new("l1", "Aluminium"));
    assert_eq!(tool.find_material("Aluminium"), Some(1));
}

#[test]
fn test_tool_find_material_missing_returns_none() {
    let mut tool = XcafMaterialTool::new();
    tool.add_material(XcafMaterial::new("l0", "Steel"));
    assert!(tool.find_material("Gold").is_none());
}

#[test]
fn test_tool_find_material_empty_tool_returns_none() {
    let tool = XcafMaterialTool::new();
    assert!(tool.find_material("Steel").is_none());
}

#[test]
fn test_tool_find_material_returns_first_match() {
    let mut tool = XcafMaterialTool::new();
    tool.add_material(XcafMaterial::new("l0", "Steel"));
    tool.add_material(XcafMaterial::new("l1", "Steel"));
    assert_eq!(tool.find_material("Steel"), Some(0));
}

#[test]
fn test_tool_set_material_for_shape_and_retrieve() {
    let mut tool = XcafMaterialTool::new();
    let idx = tool.add_material(XcafMaterial::new("l0", "Steel"));
    tool.set_material_for_shape("shape_A", idx);
    assert_eq!(tool.material_for_shape("shape_A"), Some(0));
}

#[test]
fn test_tool_material_for_shape_missing_returns_none() {
    let tool = XcafMaterialTool::new();
    assert!(tool.material_for_shape("shape_X").is_none());
}

#[test]
fn test_tool_set_material_for_shape_overwrites_existing() {
    let mut tool = XcafMaterialTool::new();
    let i0 = tool.add_material(XcafMaterial::new("l0", "Steel"));
    let i1 = tool.add_material(XcafMaterial::new("l1", "Aluminium"));
    tool.set_material_for_shape("shape_A", i0);
    tool.set_material_for_shape("shape_A", i1);
    assert_eq!(tool.material_for_shape("shape_A"), Some(i1));
}

#[test]
fn test_tool_set_material_for_multiple_shapes() {
    let mut tool = XcafMaterialTool::new();
    let i0 = tool.add_material(XcafMaterial::new("l0", "Steel"));
    let i1 = tool.add_material(XcafMaterial::new("l1", "Aluminium"));
    tool.set_material_for_shape("shape_A", i0);
    tool.set_material_for_shape("shape_B", i1);
    assert_eq!(tool.material_for_shape("shape_A"), Some(i0));
    assert_eq!(tool.material_for_shape("shape_B"), Some(i1));
}

#[test]
fn test_tool_round_trip_add_find_assign_get() {
    let mut tool = XcafMaterialTool::new();
    let mut m = XcafMaterial::new("mat_lbl", "Brass");
    m.set_density(8500.0);
    m.set_density_unit("kg/m^3");
    m.set_description("Copper-zinc alloy");
    let idx = tool.add_material(m);
    let found = tool.find_material("Brass").unwrap();
    assert_eq!(found, idx);
    tool.set_material_for_shape("body_1", found);
    let shape_idx = tool.material_for_shape("body_1").unwrap();
    let got = tool.get_material(shape_idx).unwrap();
    assert_eq!(got.name(), "Brass");
    assert!((got.density() - 8500.0).abs() < 1e-9);
    assert_eq!(got.density_unit(), "kg/m^3");
    assert_eq!(got.description(), "Copper-zinc alloy");
}
