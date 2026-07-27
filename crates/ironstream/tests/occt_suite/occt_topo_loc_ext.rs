use ironstream::topo_loc_ext::*;

#[test]
fn identity_location_is_empty() {
    let loc = TopoLocLocation::identity();
    assert!(loc.is_identity());
    assert_eq!(loc.depth(), 0);
    assert_eq!(loc.hash_code(), 0);
}

#[test]
fn multiply_appends_items() {
    let a = TopoLocLocation::from_displacement(1);
    let b = TopoLocLocation::from_displacement(2);
    let c = a.multiply(&b);
    assert_eq!(c.depth(), 2);
    assert!(!c.is_identity());
    assert_eq!(c.items[0].0, 1);
    assert_eq!(c.items[1].0, 2);
}

#[test]
fn inverted_reverses_and_negates() {
    let loc = TopoLocLocation::from_displacement(5);
    let inv = loc.inverted();
    assert_eq!(inv.items.len(), 1);
    assert_eq!(inv.items[0], (5, -1));

    // double inversion is identity-equal
    let orig = inv.inverted();
    assert_eq!(orig.items[0], (5, 1));
}

#[test]
fn power_scales_powers() {
    let loc = TopoLocLocation::from_displacement(3);
    let p2 = loc.power(2);
    assert_eq!(p2.items[0].1, 2);
    let p0 = loc.power(0);
    assert!(p0.is_identity());
    let pn = loc.power(-1);
    assert_eq!(pn.items[0].1, -1);
}

#[test]
fn indexed_map_add_dedup_and_find() {
    let mut m = TopoLocIndexedMapOfLocation::new();
    let l1 = TopoLocLocation::from_displacement(1);
    let l2 = TopoLocLocation::from_displacement(2);
    let i1 = m.add(l1.clone());
    let i2 = m.add(l2.clone());
    let i1b = m.add(l1.clone()); // duplicate
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(i1b, 1); // same index returned
    assert_eq!(m.extent(), 2);
    assert_eq!(m.find_index(&l1), Some(1));
    assert!(m.find_key(1).is_some());
    assert!(m.find_key(99).is_none());
}

#[test]
fn indexed_map_clear_and_hasher() {
    let mut m = TopoLocIndexedMapOfLocation::new();
    let l = TopoLocLocation::from_displacement(7);
    m.add(l.clone());
    assert!(!m.is_empty());
    m.clear();
    assert!(m.is_empty());
    assert_eq!(m.extent(), 0);

    // SHasher static methods
    let a = TopoLocLocation::from_displacement(1);
    let b = TopoLocLocation::from_displacement(1);
    let c = TopoLocLocation::from_displacement(2);
    assert!(TopoLocSHasher::is_equal(&a, &b));
    assert!(!TopoLocSHasher::is_equal(&a, &c));
    let h = TopoLocSHasher::hash_code(&a, 100);
    assert!(h < 100);
}
