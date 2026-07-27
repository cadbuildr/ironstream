// FILE: tests/occt_tdata_std.rs
extern crate ironstream;
use ironstream::tdata_std::*;

#[test]
fn integer_attr() {
    let mut a = TDataStdInteger::new(1, 42);
    assert_eq!(a.get(), 42);
    a.set(-7);
    assert_eq!(a.get(), -7);
    assert!(a.is_attached());
}

#[test]
fn real_attr_dimension() {
    let mut r = TDataStdReal::new(2, 3.14);
    assert!((r.get() - 3.14).abs() < 1e-10);
    r.set_dimension(TDataStdRealDimension::Length);
    assert_eq!(r.dimension(), TDataStdRealDimension::Length);
}

#[test]
fn ascii_string_attr() {
    let mut s = TDataStdAsciiString::new(3, "hello");
    assert_eq!(s.get(), "hello");
    assert_eq!(s.length(), 5);
    s.set("world");
    assert_eq!(s.get(), "world");
    assert!(!s.is_empty());
}

#[test]
fn ext_string_list_ops() {
    let mut l = TDataStdExtStringList::new(4);
    l.append("a");
    l.append("b");
    l.prepend("z");
    assert_eq!(l.size(), 3);
    assert_eq!(l.value(0), Some("z"));
    l.remove_first();
    assert_eq!(l.size(), 2);
}

#[test]
fn bool_list_counts() {
    let mut b = TDataStdBooleanList::new(5);
    b.append(true);
    b.append(false);
    b.append(true);
    b.append(true);
    assert_eq!(b.nb_true(), 3);
    assert_eq!(b.nb_false(), 1);
}

#[test]
fn integer_array() {
    let mut a = TDataStdIntegerArray::new(6, 1, 3);
    a.set_value(1, 10);
    a.set_value(2, 20);
    a.set_value(3, 30);
    assert_eq!(a.value(1), Some(10));
    assert_eq!(a.value(2), Some(20));
    assert_eq!(a.value(3), Some(30));
    // Out of bounds
    assert_eq!(a.value(4), None);
}
