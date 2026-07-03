extern crate ironstream;
use ironstream::brep_face_tool::*;

#[test]
fn find_surface_plane_xy() {
    let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[1.0,1.0,0.0],[0.0,1.0,0.0]];
    let f = FindSurface::new(pts, 1e-6);
    assert!(f.found());
    let n = f.plane_normal.unwrap();
    assert!((n[2].abs() - 1.0).abs() < 1e-6);
}

#[test]
fn find_surface_too_few_points() {
    let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
    let f = FindSurface::new(pts, 1e-6);
    assert!(!f.found());
}

#[test]
fn face_analysis_uv_bounds() {
    let boundary = vec![[0.0,0.0],[2.0,0.0],[2.0,3.0],[0.0,3.0]];
    let fa = FaceAnalysis::new(1, 0, boundary);
    let (min, max) = fa.uv_bounds();
    assert!((min[0]).abs() < 1e-10);
    assert!((max[0] - 2.0).abs() < 1e-10);
    assert!((max[1] - 3.0).abs() < 1e-10);
    assert!(!fa.is_degenerated(1e-6));
}

#[test]
fn face_set_operations() {
    let mut set = FaceSet::new();
    set.add(FaceSummary { face_id: 0, area: 4.0, normal: [0.0,0.0,1.0] });
    set.add(FaceSummary { face_id: 1, area: 6.0, normal: [1.0,0.0,0.0] });
    assert_eq!(set.extent(), 2);
    assert!((set.total_area() - 10.0).abs() < 1e-10);
    assert!(set.find_by_id(0).is_some());
    assert!(set.find_by_id(99).is_none());
    set.clear();
    assert_eq!(set.extent(), 0);
}

#[test]
fn face_explorer_iteration() {
    let mut set = FaceSet::new();
    set.add(FaceSummary { face_id: 0, area: 1.0, normal: [0.0,0.0,1.0] });
    set.add(FaceSummary { face_id: 1, area: 2.0, normal: [0.0,1.0,0.0] });
    let mut exp = FaceExplorer::new(&set);
    assert!(exp.more());
    assert_eq!(exp.current().unwrap().face_id, 0);
    exp.next();
    assert!(exp.more());
    assert_eq!(exp.current().unwrap().face_id, 1);
    exp.next();
    assert!(!exp.more());
}
