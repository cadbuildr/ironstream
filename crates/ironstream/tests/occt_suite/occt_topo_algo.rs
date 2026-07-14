extern crate ironstream;
use ironstream::topo_algo::*;

#[test]
fn line_inter_walking() {
    let mut l = LineInter::new(LineInterKind::Walking, false);
    l.add_point(InterPoint { point: [0.0;3], parameter: 0.5, is_on_first: true, is_on_second: true });
    assert_eq!(l.nb_points(), 1);
    assert!(l.is_walking());
    assert!(!l.is_restriction());
    assert!(!l.is_arc);
}

#[test]
fn faces_filler_collects_lines() {
    let mut ff = FacesFiller::new();
    assert!(!ff.is_done());
    ff.insert(LineInter::new(LineInterKind::GLine, false));
    ff.insert(LineInter::new(LineInterKind::Restriction, true));
    ff.perform();
    assert!(ff.is_done());
    assert_eq!(ff.nb_lines(), 2);
}

#[test]
fn topo_builder_pass_through() {
    let mut b = TopoBuilder::new();
    assert_eq!(b.state, BuilderState::Initial);
    b.add_shape(TopoShape { id: 0, kind: TopoKind::Solid, children: Vec::new() });
    b.add_shape(TopoShape { id: 1, kind: TopoKind::Shell, children: vec![0] });
    b.build();
    assert!(b.is_done());
    assert_eq!(b.nb_results(), 2);
}

#[test]
fn classify_2d_inside() {
    let square = vec![[0.0,0.0],[1.0,0.0],[1.0,1.0],[0.0,1.0]];
    assert_eq!(classify_point_2d([0.5, 0.5], &square), ClassifyResult2d::In);
}

#[test]
fn classify_2d_outside() {
    let square = vec![[0.0,0.0],[1.0,0.0],[1.0,1.0],[0.0,1.0]];
    assert_eq!(classify_point_2d([2.0, 0.5], &square), ClassifyResult2d::Out);
    // Too few boundary points returns Unknown
    assert_eq!(classify_point_2d([0.5, 0.5], &[[0.0,0.0],[1.0,1.0]]), ClassifyResult2d::Unknown);
}
