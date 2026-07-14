// FILE: tests/occt_loc_ops.rs
extern crate ironstream;
use ironstream::loc_ops::*;

#[test]
fn test_mat4d_identity() {
    let m = Mat4d::identity();
    assert!(m.is_identity());
    let p = m.transform_point([1.0, 2.0, 3.0]);
    assert!((p[0] - 1.0).abs() < 1e-9);
    assert!((p[1] - 2.0).abs() < 1e-9);
    assert!((p[2] - 3.0).abs() < 1e-9);
    // Default should also be identity
    let md = Mat4d::default();
    assert!(md.is_identity());
}

#[test]
fn test_mat4d_translation() {
    let m = Mat4d::translation(3.0, -1.0, 5.0);
    assert!(!m.is_identity());

    let p = m.transform_point([0.0, 0.0, 0.0]);
    assert!((p[0] - 3.0).abs() < 1e-9);
    assert!((p[1] - (-1.0)).abs() < 1e-9);
    assert!((p[2] - 5.0).abs() < 1e-9);

    let p2 = m.transform_point([1.0, 1.0, 1.0]);
    assert!((p2[0] - 4.0).abs() < 1e-9);
    assert!((p2[1] - 0.0).abs() < 1e-9);
    assert!((p2[2] - 6.0).abs() < 1e-9);
}

#[test]
fn test_mat4d_multiply_translations() {
    let t1 = Mat4d::translation(1.0, 0.0, 0.0);
    let t2 = Mat4d::translation(0.0, 2.0, 0.0);
    let t3 = t1.multiply(&t2);

    let p = t3.transform_point([0.0, 0.0, 0.0]);
    assert!((p[0] - 1.0).abs() < 1e-9);
    assert!((p[1] - 2.0).abs() < 1e-9);
    assert!((p[2] - 0.0).abs() < 1e-9);

    // identity * M = M
    let id = Mat4d::identity();
    let t4 = id.multiply(&t2);
    let p4 = t4.transform_point([0.0; 3]);
    assert!((p4[1] - 2.0).abs() < 1e-9);
}

#[test]
fn test_toploc_datum3d() {
    let d_id = TopLocDatum3d::identity(7);
    assert!(d_id.is_identity());
    assert_eq!(d_id.id, 7);

    let d = TopLocDatum3d::new(1, Mat4d::translation(5.0, 0.0, 0.0));
    assert!(!d.is_identity());
    let p = d.matrix().transform_point([0.0; 3]);
    assert!((p[0] - 5.0).abs() < 1e-9);
}

#[test]
fn test_toploc_location_identity() {
    let loc = TopLocLocation::identity();
    assert!(loc.is_identity());
    assert_eq!(loc.nb_datums(), 0);

    // transform_point on identity location leaves point unchanged
    let p = loc.transform_point([3.0, 4.0, 5.0]);
    assert!((p[0] - 3.0).abs() < 1e-9);
    assert!((p[1] - 4.0).abs() < 1e-9);
    assert!((p[2] - 5.0).abs() < 1e-9);
}

#[test]
fn test_toploc_location_multiply_and_invert() {
    let d1 = TopLocDatum3d::new(1, Mat4d::translation(2.0, 0.0, 0.0));
    let d2 = TopLocDatum3d::new(2, Mat4d::translation(0.0, 3.0, 0.0));
    let l1 = TopLocLocation::from_datum(d1);
    let l2 = TopLocLocation::from_datum(d2);

    assert!(!l1.is_identity());
    assert_eq!(l1.nb_datums(), 1);

    let l3 = l1.multiply(&l2);
    assert_eq!(l3.nb_datums(), 2);

    // transform_point applies both datums sequentially
    let p = l3.transform_point([0.0; 3]);
    assert!((p[0] - 2.0).abs() < 1e-9);
    assert!((p[1] - 3.0).abs() < 1e-9);

    // inverted reverses datum order and negates powers
    let inv = l3.inverted();
    assert_eq!(inv.nb_datums(), 2);
    // First datum in inv was d2 (power -1), second was d1 (power -1)
    assert_eq!(inv.datums[0].1, -1);
    assert_eq!(inv.datums[1].1, -1);
}
