extern crate ironstream;
use ironstream::geom_seq_tools::*;

#[test]
fn test_geom_seq_tools_basic() {
    // SeqOfPnt
    let mut seq = SeqOfPnt::new();
    assert!(seq.is_empty());
    seq.append([1.0, 2.0, 3.0]);
    seq.append([4.0, 5.0, 6.0]);
    assert_eq!(seq.length(), 2);
    assert_eq!(seq.first(), Some([1.0, 2.0, 3.0]));
    assert_eq!(seq.last(), Some([4.0, 5.0, 6.0]));
    assert_eq!(seq.value(0), [1.0, 2.0, 3.0]);
    seq.prepend([0.0, 0.0, 0.0]);
    assert_eq!(seq.length(), 3);
    assert_eq!(seq.first(), Some([0.0, 0.0, 0.0]));
    seq.clear();
    assert!(seq.is_empty());

    // SeqOfReal
    let mut rseq = SeqOfReal::new();
    rseq.append(1.0);
    rseq.append(2.0);
    rseq.append(3.0);
    assert_eq!(rseq.length(), 3);
    assert_eq!(rseq.first(), Some(1.0));
    assert_eq!(rseq.last(), Some(3.0));
    assert!((rseq.value(1) - 2.0).abs() < 1e-12);
    rseq.remove(0);
    assert_eq!(rseq.length(), 2);

    // SeqOfInteger
    let mut iseq = SeqOfInteger::new();
    iseq.append(10);
    iseq.append(20);
    assert_eq!(iseq.length(), 2);
    assert_eq!(iseq.value(0), 10);
    assert_eq!(iseq.value(1), 20);
}
