extern crate ironstream;
use ironstream::brep_tools_ext::*;

#[test]
fn test_brep_tools_ext_basic() {
    // Quilt
    let mut q = Quilt::new();
    q.add("face_A");
    q.add("face_B");
    assert_eq!(q.surface_count(), 2);
    q.bind("face_A", "face_B");
    assert_eq!(q.bindings.len(), 1);

    // ReShape
    let mut r = ReShape::new();
    r.replace("old_edge", "new_edge");
    assert_eq!(r.apply("old_edge"), "new_edge");
    assert_eq!(r.apply("other"), "other");
    assert!(r.is_recorded("old_edge"));
    assert!(!r.is_recorded("new_edge"));

    // WireExplorer
    let edges = vec!["e1".to_string(), "e2".to_string(), "e3".to_string()];
    let mut ex = WireExplorer::new(edges);
    assert!(ex.more());
    assert_eq!(ex.next().unwrap(), "e1");
    assert_eq!(ex.next().unwrap(), "e2");
    assert_eq!(ex.next().unwrap(), "e3");
    assert!(!ex.more());
}
