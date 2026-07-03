extern crate ironstream;

use ironstream::geom_geodesic::{
    CorrectedFrenet, FrenetFrame, GuideSectionGenerator, sphere_geodesic, sphere_geodesic_path,
};
use std::f64::consts::PI;

#[test]
fn frenet_frame_binormal() {
    let mut f = FrenetFrame::new();
    f.add_frame(0.0, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    // cross([1,0,0], [0,1,0]) = [0,0,1]
    assert!((f.binormals[0][2] - 1.0).abs() < 1e-10);
    assert_eq!(f.nb_frames(), 1);
}

#[test]
fn frenet_frame_interpolate_tangent() {
    let mut f = FrenetFrame::new();
    f.add_frame(0.0, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    f.add_frame(1.0, [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
    let t = f.interpolate_tangent(0.3).unwrap();
    assert_eq!(t, [1.0, 0.0, 0.0]); // nearest frame at t=0
}

#[test]
fn corrected_frenet_add_frame() {
    let mut cf = CorrectedFrenet::new();
    cf.add_frame_with_twist(0.0, [1.0,0.0,0.0], [0.0,1.0,0.0], 0.1);
    assert_eq!(cf.base.nb_frames(), 1);
    assert!((cf.twist_angle - 0.1).abs() < 1e-14);
}

#[test]
fn sphere_geodesic_quarter_circle() {
    let p1 = [1.0, 0.0, 0.0];
    let p2 = [0.0, 1.0, 0.0];
    let d = sphere_geodesic(p1, p2);
    assert!((d - PI / 2.0).abs() < 1e-10);
}

#[test]
fn guide_section_generator_perform() {
    let spine: Vec<[f64; 3]> = (0..6).map(|i| [i as f64, 0.0, 0.0]).collect();
    let guide: Vec<[f64; 3]> = (0..6).map(|i| [i as f64, 1.0, 0.0]).collect();
    let mut gen = GuideSectionGenerator::new(spine, guide);
    gen.perform();
    assert!(gen.is_done());
    assert_eq!(gen.nb_sections, 6);
}
