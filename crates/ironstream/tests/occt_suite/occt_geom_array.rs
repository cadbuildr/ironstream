use ironstream::geom_array::*;

#[test]
fn array1_pnt_basic() {
    let mut a = Array1OfPnt::new(1, 4);
    a.set_value(1, [1.0, 0.0, 0.0]);
    a.set_value(4, [4.0, 0.0, 0.0]);
    assert_eq!(a.value(1), Some([1.0, 0.0, 0.0]));
    assert_eq!(a.value(4), Some([4.0, 0.0, 0.0]));
    assert_eq!(a.value(0), None);
    assert_eq!(a.length(), 4);
}

#[test]
fn array1_pnt_bounding_box() {
    let a = Array1OfPnt::from_vec(1, vec![[0.0,0.0,0.0],[2.0,3.0,4.0],[1.0,-1.0,2.0]]);
    let (mn, mx) = a.bounding_box();
    assert!((mn[0]).abs() < 1e-10 && (mn[1]+1.0).abs() < 1e-10);
    assert!((mx[0]-2.0).abs() < 1e-10 && (mx[1]-3.0).abs() < 1e-10);
}

#[test]
fn array2_pnt_basic() {
    let mut a = Array2OfPnt::new(1, 3, 1, 3);
    a.set_value(2, 2, [5.0, 5.0, 0.0]);
    assert_eq!(a.value(2, 2), Some([5.0, 5.0, 0.0]));
    assert_eq!(a.nb_rows(), 3);
    assert_eq!(a.nb_cols(), 3);
    assert_eq!(a.value(0, 1), None);
}

#[test]
fn sequence_pnt_ops() {
    let mut s = SequenceOfPnt::new();
    s.append([1.0, 0.0, 0.0]);
    s.append([2.0, 0.0, 0.0]);
    s.prepend([0.0, 0.0, 0.0]);
    assert_eq!(s.length(), 3);
    assert_eq!(s.value(1), Some([0.0, 0.0, 0.0]));
    assert_eq!(s.value(0), None);
    s.remove(1);
    assert_eq!(s.first(), Some([1.0, 0.0, 0.0]));
}

#[test]
fn sequence_pnt_centroid() {
    let mut s = SequenceOfPnt::new();
    s.append([0.0, 0.0, 0.0]);
    s.append([4.0, 0.0, 0.0]);
    let c = s.centroid().unwrap();
    assert!((c[0] - 2.0).abs() < 1e-10);
}

#[test]
fn harray1_pnt_wrapper() {
    let mut h = HArray1OfPnt::new(1, 5);
    h.set_value(3, [7.0, 8.0, 9.0]);
    assert_eq!(h.value(3), Some([7.0, 8.0, 9.0]));
    assert_eq!(h.length(), 5);
    assert_eq!(h.lower(), 1);
    assert_eq!(h.upper(), 5);
}
