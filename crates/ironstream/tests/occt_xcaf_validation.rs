extern crate ironstream;
use ironstream::xcaf_validation::*;

#[test]
fn test_xcaf_validation_basic() {
    // VisMaterial - construction
    let mut mat = VisMaterial::new("Gold");
    assert_eq!(mat.name, "Gold");
    assert!(!mat.is_defined());  // metallic starts at -1.0

    // set_pbr defines the material
    mat.set_pbr(0.9, 0.1);
    assert!(mat.is_defined());
    assert!((mat.metallic - 0.9).abs() < 1e-12);
    assert!((mat.roughness - 0.1).abs() < 1e-12);

    // clamping
    mat.set_pbr(2.0, -1.0);
    assert!((mat.metallic - 1.0).abs() < 1e-12);
    assert!((mat.roughness - 0.0).abs() < 1e-12);

    // set_color
    mat.set_color(1.0, 0.84, 0.0, 1.0);
    assert!((mat.base_color[1] - 0.84).abs() < 1e-12);

    // VisMaterialTool
    let mut tool = VisMaterialTool::new();
    assert_eq!(tool.material_count(), 0);
    assert!(tool.find_material("Gold").is_none());

    tool.add_material(mat);
    assert_eq!(tool.material_count(), 1);

    let found = tool.find_material("Gold");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Gold");

    // Default impl
    let _default = VisMaterialTool::default();
}
