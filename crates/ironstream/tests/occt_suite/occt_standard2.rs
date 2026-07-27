use ironstream::standard2::*;

#[test]
fn guid_format_and_nonzero() {
    let g = StandardGuid::new(0x12345678, 0xabcd, 0xef01, [1, 2, 3, 4, 5, 6, 7, 8]);
    let s = g.to_hex_string();
    assert!(s.starts_with("12345678-abcd-ef01"), "GUID string should start correctly");
    assert!(!g.is_zero());
}

#[test]
fn guid_zero_default() {
    let g = StandardGuid::default();
    assert!(g.is_zero());
    assert_eq!(g, StandardGuid::ZERO);
}

#[test]
fn standard_type_kind_of() {
    let t = StandardType::new("Geom_Curve", 64).with_parent("Standard_Transient");
    assert!(t.is_kind_of("Geom_Curve"));
    assert!(t.is_kind_of("Standard_Transient"), "should match parent type");
    assert!(!t.is_kind_of("Geom_Surface"));
    assert_eq!(t.name(), "Geom_Curve");
    assert_eq!(t.size_of(), 64);
}

#[test]
fn transient_ref_count() {
    let mut t = StandardTransient::new("AIS_Shape");
    assert_eq!(t.ref_count(), 0);
    t.increment_ref();
    t.increment_ref();
    assert_eq!(t.ref_count(), 2);
    assert!(!t.decrement_ref(), "still has 1 ref remaining");
    assert!(t.decrement_ref(), "last decrement should return true");
}

#[test]
fn handle_null_and_non_null() {
    let h: Handle<i32> = Handle::null();
    assert!(h.is_null());
    assert!(h.as_ref().is_none());

    let h2 = Handle::new(42i32);
    assert!(!h2.is_null());
    assert_eq!(h2.as_ref(), Some(&42));
}

#[test]
fn type_registry_dedup() {
    let mut reg = StandardTypeRegistry::new();
    reg.register(StandardType::new("Geom_Curve", 64));
    reg.register(StandardType::new("Geom_Surface", 80));
    reg.register(StandardType::new("Geom_Curve", 64)); // duplicate
    assert_eq!(reg.nb_types(), 2, "duplicate type should not be stored twice");
    assert!(reg.find("Geom_Curve").is_some());
    assert!(reg.find("Unknown").is_none());
}
