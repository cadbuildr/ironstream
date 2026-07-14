use ironstream::tcolstd_ext::*;

#[test]
fn array1_real_from_vec_lower_bound() {
    let a = Array1OfReal::from_vec(5, vec![10.0, 20.0, 30.0]);
    assert_eq!(a.lower(), 5);
    assert_eq!(a.upper(), 7);
    assert_eq!(a.get(5), Some(10.0));
    assert_eq!(a.get(7), Some(30.0));
    assert!(a.get(4).is_none());
}

#[test]
fn array1_real_init_and_overwrite() {
    let mut a = Array1OfReal::with_init(1, 4, 3.14);
    for i in 1..=4 {
        assert!((a.get(i).unwrap() - 3.14).abs() < 1e-10);
    }
    a.set(2, 99.0);
    assert!((a.get(2).unwrap() - 99.0).abs() < 1e-10);
}

#[test]
fn array2_real_out_of_bounds() {
    let mut a = Array2OfReal::new(1, 2, 1, 2);
    a.set(1, 1, 5.0);
    assert!(a.get(0, 1).is_none()); // row 0 out of bounds
    assert!(a.get(1, 3).is_none()); // col 3 out of bounds
    assert_eq!(a.get(1, 1), Some(5.0));
}

#[test]
fn data_map_string_integer_overwrite() {
    let mut m = DataMapOfStringInteger::new();
    m.bind("key", 1);
    m.bind("key", 42); // overwrite
    assert_eq!(m.find("key"), Some(42));
    assert_eq!(m.size(), 1);
    assert!(!m.is_empty());
    m.clear();
    assert!(m.is_empty());
}

#[test]
fn list_of_integer_prepend_ordering() {
    let mut l = ListOfInteger::new();
    l.append(1);
    l.append(2);
    l.prepend(0);
    assert_eq!(l.first(), Some(0));
    assert_eq!(l.last(), Some(2));
    assert_eq!(l.size(), 3);
}

#[test]
fn sequence_of_real_set_value() {
    let mut s = SequenceOfReal::new();
    s.append(1.0);
    s.append(2.0);
    s.append(3.0);
    s.set_value(1, 20.0);
    assert_eq!(s.value(1), Some(20.0));
    s.insert_before(0, 0.5);
    assert_eq!(s.first(), Some(0.5));
    assert_eq!(s.length(), 4);
}
