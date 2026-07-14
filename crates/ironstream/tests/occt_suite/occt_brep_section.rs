extern crate ironstream;

use ironstream::brep_section::{SectionCurve, SectionOperation};

#[test]
fn section_no_plane_no_curves() {
    let pts = vec![[0.0,0.0,5.0],[1.0,0.0,5.0]];
    let mut s = SectionOperation::new(pts);
    s.build();
    assert!(s.is_done());
    assert_eq!(s.nb_edges(), 0);
}

#[test]
fn section_xy_plane_projects_to_z_zero() {
    let pts = vec![[0.0,0.0,3.0],[1.0,0.0,3.0],[2.0,0.0,3.0]];
    let mut s = SectionOperation::new(pts);
    s.set_plane([0.0,0.0,1.0], 0.0);
    s.build();
    assert!(s.is_done());
    assert_eq!(s.nb_edges(), 1);
    for p in &s.result_curves[0].points {
        assert!(p[2].abs() < 1e-10);
    }
}

#[test]
fn section_approximation_flag() {
    let pts = vec![[0.0,0.0,0.0]];
    let mut s = SectionOperation::new(pts);
    assert!(s.approximation);
    s.set_approximation(false);
    assert!(!s.approximation);
}

#[test]
fn section_curve_closed_detection() {
    let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[1.0,1.0,0.0],[0.0,0.0,0.0]];
    let c = SectionCurve::new(pts);
    assert!(c.is_closed);
    assert_eq!(c.nb_points(), 4);
}

#[test]
fn section_curve_open() {
    let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0]];
    let c = SectionCurve::new(pts);
    assert!(!c.is_closed);
}
