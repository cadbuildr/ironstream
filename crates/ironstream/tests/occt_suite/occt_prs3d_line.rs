use ironstream::prs3d_line::*;

#[test]
fn prs3d_line_aspect_color_type_width() {
    let mut a = Prs3dLineAspect::new([1.0, 0.0, 0.0], TypeOfLine::Dashed, 2.0);
    assert_eq!(a.type_of_line(), TypeOfLine::Dashed);
    assert!((a.width() - 2.0).abs() < 1e-12);
    assert_eq!(a.color(), [1.0, 0.0, 0.0]);

    a.set_color([0.0, 1.0, 0.0]);
    assert_eq!(a.color(), [0.0, 1.0, 0.0]);

    a.set_type_of_line(TypeOfLine::Dotted);
    assert_eq!(a.type_of_line(), TypeOfLine::Dotted);

    a.set_width(3.5);
    assert!((a.width() - 3.5).abs() < 1e-12);

    // default
    let def = Prs3dLineAspect::default();
    assert_eq!(def.type_of_line(), TypeOfLine::Solid);
    assert!((def.width() - 1.0).abs() < 1e-12);
}

#[test]
fn prs3d_point_aspect_marker_and_scale() {
    let mut p = Prs3dPointAspect::new([1.0, 1.0, 0.0], TypeOfMarker::Star, 2.0);
    assert_eq!(p.type_of_marker(), TypeOfMarker::Star);
    assert!((p.scale() - 2.0).abs() < 1e-12);

    p.set_type_of_marker(TypeOfMarker::Plus);
    assert_eq!(p.type_of_marker(), TypeOfMarker::Plus);

    p.set_scale(5.0);
    assert!((p.scale() - 5.0).abs() < 1e-12);

    p.set_color([0.5, 0.5, 0.5]);
    assert_eq!(p.color(), [0.5, 0.5, 0.5]);

    // default
    let def = Prs3dPointAspect::default();
    assert_eq!(def.type_of_marker(), TypeOfMarker::Point);
    assert!((def.scale() - 1.0).abs() < 1e-12);
}

#[test]
fn prs3d_shading_aspect_transparency_clamp() {
    let mut s = Prs3dShadingAspect::new();
    assert!((s.transparency() - 0.0).abs() < 1e-12); // default

    s.set_transparency(0.5);
    assert!((s.transparency() - 0.5).abs() < 1e-12);

    // clamp above 1.0
    s.set_transparency(2.0);
    assert!((s.transparency() - 1.0).abs() < 1e-12);

    // clamp below 0.0
    s.set_transparency(-1.0);
    assert!((s.transparency() - 0.0).abs() < 1e-12);

    s.set_color([0.2, 0.3, 0.4]);
    assert_eq!(s.color(), [0.2, 0.3, 0.4]);

    s.set_shading_model(ShadingModel::Flat);
    assert_eq!(s.shading_model(), ShadingModel::Flat);
}

#[test]
fn prs3d_iso_aspect_nb_iso() {
    let mut iso = Prs3dIsoAspect::default();
    assert_eq!(iso.number(), 10); // default

    iso.set_number(5);
    assert_eq!(iso.number(), 5);

    iso.set_number(0);
    assert_eq!(iso.number(), 0);

    // construction with explicit count
    let iso2 = Prs3dIsoAspect::new([0.8, 0.8, 0.8], TypeOfLine::Solid, 1.0, 20);
    assert_eq!(iso2.number(), 20);
}

#[test]
fn prs3d_plane_aspect_plane_length() {
    let mut p = Prs3dPlaneAspect::new();
    // defaults
    assert!((p.plane_x_length() - 100.0).abs() < 1e-10);
    assert!((p.plane_y_length() - 100.0).abs() < 1e-10);
    assert!(p.display_edges());
    assert!(p.display_iso());

    p.set_plane_length(200.0, 150.0);
    assert!((p.plane_x_length() - 200.0).abs() < 1e-10);
    assert!((p.plane_y_length() - 150.0).abs() < 1e-10);

    p.set_display_edges(false);
    assert!(!p.display_edges());

    p.set_display_iso(false);
    assert!(!p.display_iso());
}
