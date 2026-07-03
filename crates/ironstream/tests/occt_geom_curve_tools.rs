extern crate ironstream;
use ironstream::geom_curve_tools::*;

#[test]
fn test_geom_curve_tools_basic() {
    // curve_value stub
    let v = curve_value("my_curve", 0.5);
    assert_eq!(v[0], 0.5);

    // curve_d1 stub
    let (pt, tan) = curve_d1("my_curve", 0.0);
    assert_eq!(pt[0], 0.0);
    assert_eq!(tan[0], 1.0);
    assert_eq!(tan[1], 0.0);
    assert_eq!(tan[2], 0.0);

    // normal_to_surface stub
    let n = normal_to_surface("my_surface", 0.0, 0.0);
    assert_eq!(n, [0.0, 0.0, 1.0]);

    // is_c0_curve
    assert!(is_c0_curve("curve", 1e-7));
    assert!(!is_c0_curve("", 1e-7));
    assert!(!is_c0_curve("curve", -1.0));

    // build_curve_3d
    let tag = build_curve_3d("c2d", "surf", 1e-7);
    assert!(tag.contains("c2d"));
    assert!(tag.contains("surf"));

    // CurveTools struct
    let ct = CurveTools::new(1e-6);
    assert_eq!(ct.tolerance, 1e-6);
    assert!(ct.check_curve("my_curve"));
    assert!(!ct.check_curve(""));

    let samples = ct.sample_curve("my_curve", 5);
    assert_eq!(samples.len(), 5);

    let (lo, hi) = ct.curve_range("my_curve");
    assert_eq!(lo, 0.0);
    assert_eq!(hi, 1.0);
}
