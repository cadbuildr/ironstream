extern crate ironstream;
use ironstream::brep_algo_ext::*;

#[test]
fn test_brep_algo_ext_basic() {
    // BuilderAlgo
    let mut ba = BuilderAlgo::new();
    assert!(!ba.is_done());

    ba.set_arguments(vec!["shape_a".to_string(), "shape_b".to_string()]);
    ba.set_tools(vec!["tool_x".to_string()]);
    ba.build();
    assert!(ba.is_done());
    let result = ba.shape();
    assert!(!result.is_empty());

    // passthrough when no tools
    let mut ba2 = BuilderAlgo::new();
    ba2.set_arguments(vec!["s1".to_string(), "s2".to_string()]);
    ba2.build();
    assert!(ba2.is_done());
    let r2 = ba2.shape();
    assert_eq!(r2.len(), 2);

    // FaceRestrictor
    let mut fr = FaceRestrictor::new();
    fr.init("my_face", true);
    fr.add_wire("wire_1");
    fr.add_wire("wire_2");
    fr.perform();
    assert!(fr.is_done());
    let first = fr.current();
    assert!(!first.is_empty());
    fr.next();

    // split_face free function
    let wires = vec!["w1".to_string(), "w2".to_string()];
    let pieces = split_face("big_face", &wires);
    assert!(!pieces.is_empty());
}
