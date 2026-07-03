extern crate ironstream;
use ironstream::bspl_surf::*;

fn make_bilinear_surface() -> GeomBSplineSurface {
    let poles = vec![
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
    ];
    GeomBSplineSurface::new(
        1, 1, 1, poles,
        vec![0.0, 1.0], vec![0.0, 1.0],
        vec![2, 2], vec![2, 2],
    )
}

#[test]
fn test_bspline_surface_dims_and_degrees() {
    let s = make_bilinear_surface();
    assert_eq!(s.nb_u_poles(), 2);
    assert_eq!(s.nb_v_poles(), 2);
    assert_eq!(s.degree_u(), 1);
    assert_eq!(s.degree_v(), 1);
    assert_eq!(s.nb_u_knots(), 2);
    assert_eq!(s.nb_v_knots(), 2);
}

#[test]
fn test_bspline_surface_pole_access() {
    let mut s = make_bilinear_surface();
    // 1-based pole access
    let p11 = s.pole(1, 1).unwrap();
    assert!((p11[0]).abs() < 1e-10);
    assert!((p11[1]).abs() < 1e-10);

    let p12 = s.pole(1, 2).unwrap();
    assert!((p12[0] - 1.0).abs() < 1e-10);

    // 0-based indices return None
    assert_eq!(s.pole(0, 1), None);
    assert_eq!(s.pole(1, 0), None);

    // set_pole
    s.set_pole(2, 2, [5.0, 5.0, 5.0]);
    let p22 = s.pole(2, 2).unwrap();
    assert!((p22[0] - 5.0).abs() < 1e-10);
}

#[test]
fn test_bspline_surface_knots_and_parameters() {
    let s = make_bilinear_surface();
    assert_eq!(s.u_knot(0), None);
    assert_eq!(s.u_knot(1), Some(0.0));
    assert_eq!(s.u_knot(2), Some(1.0));
    assert!((s.first_u_parameter()).abs() < 1e-10);
    assert!((s.last_u_parameter() - 1.0).abs() < 1e-10);
    assert!((s.first_v_parameter()).abs() < 1e-10);
    assert!((s.last_v_parameter() - 1.0).abs() < 1e-10);
}

#[test]
fn test_bspline_surface_value() {
    let s = make_bilinear_surface();
    // Corner (0,0) → pole[0][0] = [0,0,0]
    let v00 = s.value(0.0, 0.0);
    assert!((v00[0]).abs() < 1e-10);
    assert!((v00[1]).abs() < 1e-10);
    // Corner (1,0) → pole[1][0] = [0,1,0] (row=1, col=0)
    let v10 = s.value(1.0, 0.0);
    // row index = 1 (nr=2, tu=1 => i=1), col=0 (tv=0 => j=0)
    assert!((v10[0]).abs() < 1e-10);
    assert!((v10[1] - 1.0).abs() < 1e-10);
}

#[test]
fn test_geom_convert_approx_surface() {
    let a = GeomConvertApproxSurface::new(1, 1e-4, 1, 1, 5, 5, 100);
    assert!(a.is_done());
    assert!(a.surface() > 0);
    assert!(a.max_error() < 1.0);
    assert!(a.average_error() <= a.max_error());

    // null surface → not done
    let a_null = GeomConvertApproxSurface::new(0, 1e-4, 1, 1, 5, 5, 100);
    assert!(!a_null.is_done());
    assert_eq!(a_null.surface(), 0);
}

#[test]
fn test_geom_fill_nsections_and_bspline_curves() {
    // GeomFillNSections requires ≥2 sections
    let mut ns = GeomFillNSections::new();
    ns.add_section(1, 0.0);
    ns.build();
    assert!(!ns.is_done()); // only 1 section
    ns.add_section(2, 1.0);
    ns.build();
    assert!(ns.is_done());
    assert_eq!(ns.nb_sections(), 2);
    assert!(ns.surface() > 0);

    // GeomFillBSplineCurves: c1=0 or c2=0 → not done
    let gc_ok = GeomFillBSplineCurves::new(1, 2, 3, 4, FillingOrder::Curved);
    assert!(gc_ok.is_done());
    assert!(gc_ok.surface() > 0);

    let gc_bad = GeomFillBSplineCurves::new(0, 2, 3, 4, FillingOrder::Stretch);
    assert!(!gc_bad.is_done());
    assert_eq!(gc_bad.surface(), 0);
}
