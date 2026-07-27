use ironstream::graphic3d_attr::*;

#[test]
fn polygon_offset_default_is_none() {
    assert_eq!(PolygonOffset::default(), PolygonOffset::None);
}

#[test]
fn texture_params_wrap_modes() {
    let mut tp = Graphic3dTextureParams::new();
    tp.set_wrap(TextureWrapMode::Mirror, TextureWrapMode::ClampToBorder);
    assert_eq!(tp.wrap_u, TextureWrapMode::Mirror);
    assert_eq!(tp.wrap_v, TextureWrapMode::ClampToBorder);
    tp.set_translation(0.5, 0.25);
    assert!((tp.translation[0] - 0.5).abs() < 1e-10);
}

#[test]
fn texture_params_rotation() {
    let mut tp = Graphic3dTextureParams::new();
    tp.set_rotation(45.0);
    assert!((tp.rotation_angle - 45.0).abs() < 1e-10);
}

#[test]
fn material_aspect_transparency_clamp() {
    let mut mat = Graphic3dMaterialAspect::new("Glass");
    mat.set_transparency(1.5); // should clamp to 1.0
    assert!((mat.transparency() - 1.0).abs() < 1e-6);
    assert!(mat.is_transparent());
}

#[test]
fn material_aspect_specular() {
    let mut mat = Graphic3dMaterialAspect::default();
    mat.set_specular_color(0.9, 0.9, 0.9);
    assert_eq!(mat.specular(), [0.9, 0.9, 0.9]);
}

#[test]
fn aspects_set_material_both_sides() {
    let mut a = Graphic3dAspects::new();
    let mut mat = Graphic3dMaterialAspect::new("Rubber");
    mat.set_diffuse_color(0.1, 0.1, 0.1);
    a.set_material(mat);
    assert_eq!(a.material_front.diffuse_color, [0.1, 0.1, 0.1]);
    assert_eq!(a.material_back.diffuse_color, [0.1, 0.1, 0.1]);
    a.set_line_width(2.5);
    assert!((a.line_width() - 2.5).abs() < 1e-6);
}
