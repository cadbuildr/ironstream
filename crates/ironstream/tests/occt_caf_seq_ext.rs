use ironstream::caf_seq_ext::*;

#[test]
fn named_data_set_get_integer_real_string() {
    let mut d = TDataStdNamedData::new();
    d.set_integer("count", 42);
    d.set_real("scale", 2.718);
    d.set_string("label", "part_A");
    assert_eq!(d.get_integer("count"), Some(42));
    assert!((d.get_real("scale").unwrap() - 2.718).abs() < 1e-10);
    assert_eq!(d.get_string("label"), Some("part_A"));
}

#[test]
fn named_data_override_integer() {
    let mut d = TDataStdNamedData::new();
    d.set_integer("x", 1);
    d.set_integer("x", 99);
    assert_eq!(d.get_integer("x"), Some(99));
    assert_eq!(d.nb_integers(), 1);
}

#[test]
fn named_data_remove_and_all_integer_keys() {
    let mut d = TDataStdNamedData::new();
    d.set_integer("a", 1);
    d.set_integer("b", 2);
    let keys = d.all_integer_keys();
    assert_eq!(keys.len(), 2);
    d.remove("a");
    assert!(!d.has_integer("a"));
    assert!(d.has_integer("b"));
}

#[test]
fn int_packed_map_add_remove_contains() {
    let mut m = TDataStdIntPackedMap::new();
    assert!(m.add(5));
    assert!(m.add(10));
    assert!(!m.add(5)); // duplicate
    assert_eq!(m.nb_values(), 2);
    assert!(m.contains(5));
    assert!(m.remove(5));
    assert!(!m.contains(5));
    assert!(!m.remove(99)); // not present
}

#[test]
fn int_packed_map_intersect_and_union() {
    let mut a = TDataStdIntPackedMap::new();
    a.add(1); a.add(2); a.add(3);
    let mut b = TDataStdIntPackedMap::new();
    b.add(2); b.add(3); b.add(4);
    let inter = a.intersect(&b);
    assert_eq!(inter.nb_values(), 2);
    assert!(inter.contains(2));
    assert!(inter.contains(3));
    let un = a.union(&b);
    assert_eq!(un.nb_values(), 4);
}

#[test]
fn reference_array_set_get_nb_set_out_of_bounds() {
    let mut arr = TDataStdReferenceArray::new(0, 4);
    assert_eq!(arr.nb_references(), 5);
    arr.set(0, 100);
    arr.set(2, 200);
    arr.set(4, 400);
    assert_eq!(arr.get(0), Some(100));
    assert_eq!(arr.get(1), None);
    assert_eq!(arr.get(4), Some(400));
    assert_eq!(arr.nb_set(), 3);
    // out-of-bounds
    assert_eq!(arr.get(5), None);
    assert!(!arr.set(5, 999));
}
