extern crate ironstream;
use ironstream::brep_edge_maker::*;

#[test]
fn test_brep_edge_maker_basic() {
    let p1 = [0.0_f64, 0.0, 0.0];
    let p2 = [1.0_f64, 0.0, 0.0];
    let em = EdgeMaker::from_line(p1, p2);
    assert!(em.is_done());
    assert!(!em.edge().is_empty());

    let em2 = EdgeMaker::from_circle([0.0, 0.0, 0.0], 1.0, 0.0, std::f64::consts::PI);
    assert!(em2.is_done());
    assert!(!em2.edge().is_empty());

    let em3 = EdgeMaker::from_curve("my_curve", 0.0, 1.0);
    assert!(em3.is_done());

    let vm = VertexMaker::new([1.0, 2.0, 3.0]);
    assert!(vm.is_done());
    let v = vm.vertex();
    assert!((v[0] - 1.0).abs() < 1e-10);
    assert!((v[1] - 2.0).abs() < 1e-10);
    assert!((v[2] - 3.0).abs() < 1e-10);

    let edge_label = make_edge_line(p1, p2);
    assert!(!edge_label.is_empty());

    let arc_label = make_edge_arc([0.0, 0.0, 0.0], 2.0, 0.0, 1.0);
    assert!(!arc_label.is_empty());
}
