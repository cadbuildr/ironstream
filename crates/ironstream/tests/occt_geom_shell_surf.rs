extern crate ironstream;
use ironstream::geom_shell_surf::*;

#[test]
fn make_shell_success() {
    let mut s = MakeShell::new();
    s.add_face(ShellFace::new(0, "Plane", [0.0,0.0,1.0], 4.0));
    s.add_face(ShellFace::new(1, "Plane", [0.0,0.0,-1.0], 4.0));
    s.build();
    assert!(s.is_done());
    assert_eq!(s.nb_faces(), 2);
    assert!((s.total_area() - 8.0).abs() < 1e-10);
}

#[test]
fn make_shell_empty_fails() {
    let mut s = MakeShell::new();
    s.build();
    assert!(!s.is_done());
    assert!(s.error.is_some());
}

#[test]
fn shell_analysis_free_edges() {
    let faces = vec![ShellFace::new(0, "Cylinder", [1.0,0.0,0.0], 6.28)];
    let mut a = ShellAnalysis::new(faces);
    assert!(a.check_closed());
    a.add_free_edge(FreeEdge { start: [0.0,0.0,0.0], end: [1.0,0.0,0.0], adjacent_face_count: 1 });
    assert!(!a.check_closed());
    assert_eq!(a.nb_free_edges(), 1);
    assert!(a.has_free_edges());
}

#[test]
fn solid_classifier_all_closed() {
    let shells = vec![
        ShellSummary { is_closed: true, is_oriented_outward: true, nb_faces: 6 },
        ShellSummary { is_closed: true, is_oriented_outward: false, nb_faces: 4 },
    ];
    let c = SolidClassifier::new(shells, 1e-6);
    assert!(c.all_closed());
    assert_eq!(c.nb_shells(), 2);
    assert_eq!(c.classify([0.0;3]), TopAbsState::Out);
}

#[test]
fn shell_boundary_closed() {
    let mut b = ShellBoundary::new();
    b.add_curve(vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[1.0,1.0,0.0]]);
    b.add_curve(vec![[1.0,1.0,0.0],[0.0,0.0,0.0]]);
    b.check_closed();
    assert!(b.is_closed);
    assert_eq!(b.nb_curves(), 2);
}
