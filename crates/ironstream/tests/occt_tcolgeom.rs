use ironstream::tcolgeom::*;

#[test]
fn array_of_dir_normalized() {
    let mut a = TColgpArray1OfDir::new(1, 3);
    a.set_value(1, [3.0, 0.0, 0.0]);
    let v = a.value(1).unwrap();
    assert!((v[0] - 1.0).abs() < 1e-10, "direction should be normalised to unit length");
    assert!(a.value(0).is_none(), "index below lower bound should return None");
    assert!(a.value(4).is_none(), "index above upper bound should return None");
}

#[test]
fn array_of_vec_basic() {
    let mut a = TColgpArray1OfVec::new(1, 3);
    a.set_value(2, [1.0, 2.0, 3.0]);
    assert_eq!(a.value(2), Some([1.0, 2.0, 3.0]));
    assert_eq!(a.length(), 3);
    assert_eq!(a.lower(), 1);
    assert_eq!(a.upper(), 3);
}

#[test]
fn array_of_int_init_and_sum() {
    let mut a = TColStdArray1OfInt::new(1, 5);
    a.init(3);
    assert_eq!(a.sum(), 15);
    a.set_value(3, 10);
    assert_eq!(a.value(3), Some(10));
    assert_eq!(a.sum(), 5 * 3 - 3 + 10); // 22
}

#[test]
fn array_of_real_min_max() {
    let mut a = TColStdArray1OfReal::new(1, 4);
    a.set_value(1, 3.0);
    a.set_value(2, 1.0);
    a.set_value(3, 5.0);
    a.set_value(4, 2.0);
    assert!((a.min().unwrap() - 1.0).abs() < 1e-10);
    assert!((a.max().unwrap() - 5.0).abs() < 1e-10);
}

#[test]
fn seq_of_int_append_value() {
    let mut s = TColStdSeqOfInt::new();
    s.append(10);
    s.append(20);
    s.prepend(5);
    assert_eq!(s.length(), 3);
    assert_eq!(s.first(), Some(5));
    assert_eq!(s.last(), Some(20));
    assert_eq!(s.value(2), Some(10));
}

#[test]
fn seq_of_int_remove_and_clear() {
    let mut s = TColStdSeqOfInt::new();
    s.append(1);
    s.append(2);
    s.append(3);
    s.remove(1);
    assert_eq!(s.length(), 2);
    assert_eq!(s.first(), Some(2));
    s.clear();
    assert!(s.is_empty());
}
