use ironstream::prs3d_shading::*;

#[test]
fn shading_model_default_is_phong() {
    assert_eq!(ShadingModel::default(), ShadingModel::Phong);
}

#[test]
fn face_side_default_is_both() {
    assert_eq!(FaceSide::default(), FaceSide::Aspect_TOFM_BOTH_SIDE);
}

#[test]
fn shading_material_matte_no_specular() {
    let m = ShadingMaterial::matte();
    assert_eq!(m.specular, [0.0, 0.0, 0.0]);
    assert!((m.shininess - 1.0).abs() < 1e-6);
    assert!(!m.is_transparent());
}

#[test]
fn prs3d_shading_aspect_model() {
    let mut a = Prs3dShadingAspect::new();
    a.set_shading_model(ShadingModel::Pbr);
    assert_eq!(a.shading_model, ShadingModel::Pbr);
    a.set_face_side(FaceSide::Aspect_TOFM_FRONT_SIDE);
    assert_eq!(a.face_side, FaceSide::Aspect_TOFM_FRONT_SIDE);
}

#[test]
fn prs3d_shading_aspect_front_back_materials() {
    let mut a = Prs3dShadingAspect::new();
    a.set_material_front(ShadingMaterial::metallic());
    a.set_material_back(ShadingMaterial::matte());
    assert!((a.material_front.metallic - 1.0).abs() < 1e-6);
    assert_eq!(a.material_back.specular, [0.0, 0.0, 0.0]);
}

#[test]
fn drawer_ext_iso_counts() {
    let mut d = DrawerExt::new();
    d.set_nb_isos(5, 10);
    assert_eq!(d.u_iso_aspect.number(), 5);
    assert_eq!(d.v_iso_aspect.number(), 10);
    d.set_shading_color(0.3, 0.3, 0.3);
    assert_eq!(d.shading_aspect.material_front.diffuse, [0.3, 0.3, 0.3]);
}
