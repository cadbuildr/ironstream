extern crate ironstream;
use ironstream::occt_collect::*;

#[test]
fn array1_size_and_bounds() {
    let a: Array1<i32> = Array1::new(1, 5);
    assert_eq!(a.size(), 5);
    assert_eq!(a.lower(), 1);
    assert_eq!(a.upper(), 5);
}

#[test]
fn array1_get_set() {
    let mut a: Array1<f64> = Array1::new(1, 4);
    a.set(2, 3.14);
    assert!((a.get(2).unwrap() - 3.14).abs() < 1e-10);
    assert_eq!(a.get(1), Some(&0.0));
}

#[test]
fn array2_dimensions_and_set() {
    let mut a: Array2<i32> = Array2::new(1, 3, 1, 4);
    assert_eq!(a.nb_rows(), 3);
    assert_eq!(a.nb_cols(), 4);
    a.set(2, 3, 99);
    assert_eq!(a.get(2, 3), Some(&99));
    assert_eq!(a.get(1, 1), Some(&0));
}

#[test]
fn list_operations() {
    let mut l: List<u32> = List::new();
    l.append(10);
    l.append(20);
    l.prepend(5);
    assert_eq!(l.size(), 3);
    assert_eq!(l.first(), Some(&5));
    assert_eq!(l.last(), Some(&20));
    l.remove_first();
    assert_eq!(l.size(), 2);
}

#[test]
fn sequence_value_and_remove() {
    let mut s: Sequence<&str> = Sequence::new();
    s.append("a");
    s.append("b");
    s.append("c");
    assert_eq!(s.value(2), Some(&"b"));
    s.remove(2);
    assert_eq!(s.size(), 2);
    assert_eq!(s.value(2), Some(&"c"));
}
