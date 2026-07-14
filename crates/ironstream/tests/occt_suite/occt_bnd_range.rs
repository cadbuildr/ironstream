//! Ported OpenCascade unit test — `Bnd_Range`.
//!
//! Faithful Rust port of OCCT's `Bnd_Range_Test.cxx`
//! (Open-Cascade-SAS/OCCT, `src/FoundationClasses/TKMath/GTests/`). Same
//! numeric inputs, expected values and (exact-double) comparisons. OCCT's
//! `EXPECT_DOUBLE_EQ` is an exact-ULP comparison; every expected value here is
//! representable, so we compare exactly. `NCollection_List<Bnd_Range>` maps to
//! `Vec<BndRange>`, `Handle`/value types map to plain values, and OCCT's
//! `Standard_ConstructionError` throw maps to a Rust panic (verified via
//! `catch_unwind`) plus the non-panicking `try_with_bounds` variant.

use ironstream::bnd_range::{BndRange, IntersectStatus};

// ====================== Bnd_Range_Test.cxx ======================

#[test]
fn default_constructor_is_void() {
    let range = BndRange::new();
    assert!(range.is_void());
    assert!(range.delta() < 0.0);
}

#[test]
fn parameterized_constructor() {
    let range = BndRange::with_bounds(3.0, 15.0);
    assert!(!range.is_void());
    assert_eq!(range.delta(), 12.0);
}

#[test]
fn parameterized_constructor_invalid_range() {
    // EXPECT_THROW(Bnd_Range(10.0, 5.0), Standard_ConstructionError)
    let result = std::panic::catch_unwind(|| BndRange::with_bounds(10.0, 5.0));
    assert!(result.is_err(), "expected construction error for max < min");
    // And the non-panicking variant reports the same error.
    assert!(BndRange::try_with_bounds(10.0, 5.0).is_err());
}

#[test]
fn parameterized_constructor_point_range() {
    let range = BndRange::with_bounds(5.0, 5.0);
    assert!(!range.is_void());
    assert_eq!(range.delta(), 0.0);
}

#[test]
fn get_min_get_max_get_bounds() {
    let range = BndRange::with_bounds(2.0, 8.0);
    assert_eq!(range.get_min(), Some(2.0));
    assert_eq!(range.get_max(), Some(8.0));

    let (first, last) = range.get_bounds().expect("not void");
    assert_eq!(first, 2.0);
    assert_eq!(last, 8.0);
}

#[test]
fn get_min_get_max_void() {
    let range = BndRange::new();
    assert_eq!(range.get_min(), None);
    assert_eq!(range.get_max(), None);
    assert_eq!(range.get_bounds(), None);
}

#[test]
fn get_intermediate_point() {
    let range = BndRange::with_bounds(10.0, 20.0);
    assert_eq!(range.get_intermediate_point(0.0), Some(10.0));
    assert_eq!(range.get_intermediate_point(0.5), Some(15.0));
    assert_eq!(range.get_intermediate_point(1.0), Some(20.0));
}

#[test]
fn get_intermediate_point_void() {
    let range = BndRange::new();
    assert_eq!(range.get_intermediate_point(0.5), None);
}

#[test]
fn set_void() {
    let mut range = BndRange::with_bounds(1.0, 5.0);
    assert!(!range.is_void());
    range.set_void();
    assert!(range.is_void());
}

#[test]
fn add_double_to_void() {
    let mut range = BndRange::new();
    range.add(5.0);
    assert!(!range.is_void());
    assert_eq!(range.get_bounds(), Some((5.0, 5.0)));
}

#[test]
fn add_double_extends() {
    let mut range = BndRange::with_bounds(3.0, 7.0);
    range.add(1.0);
    range.add(10.0);
    assert_eq!(range.get_bounds(), Some((1.0, 10.0)));
}

#[test]
fn add_range_to_void() {
    let mut range = BndRange::new();
    let other = BndRange::with_bounds(3.0, 7.0);
    range.add_range(&other);
    assert_eq!(range.get_bounds(), Some((3.0, 7.0)));
}

#[test]
fn add_void_range() {
    let mut range = BndRange::with_bounds(3.0, 7.0);
    let void = BndRange::new();
    range.add_range(&void);
    // Should remain unchanged.
    assert_eq!(range.get_bounds(), Some((3.0, 7.0)));
}

#[test]
fn add_range_void_to_void() {
    let mut range = BndRange::new();
    let void = BndRange::new();
    range.add_range(&void);
    assert!(range.is_void());
}

#[test]
fn add_range_extends() {
    let mut range = BndRange::with_bounds(3.0, 7.0);
    let other = BndRange::with_bounds(1.0, 10.0);
    range.add_range(&other);
    assert_eq!(range.get_bounds(), Some((1.0, 10.0)));
}

#[test]
fn common_overlapping() {
    let mut range = BndRange::with_bounds(1.0, 10.0);
    let other = BndRange::with_bounds(5.0, 15.0);
    range.common(&other);
    assert_eq!(range.get_bounds(), Some((5.0, 10.0)));
}

#[test]
fn common_no_overlap() {
    let mut range = BndRange::with_bounds(1.0, 5.0);
    let other = BndRange::with_bounds(7.0, 10.0);
    range.common(&other);
    assert!(range.is_void());
}

#[test]
fn common_with_void() {
    let mut range = BndRange::with_bounds(1.0, 5.0);
    let void = BndRange::new();
    range.common(&void);
    assert!(range.is_void());
}

#[test]
fn union_overlapping() {
    let mut range = BndRange::with_bounds(1.0, 7.0);
    let other = BndRange::with_bounds(5.0, 15.0);
    assert!(range.union(&other));
    assert_eq!(range.get_bounds(), Some((1.0, 15.0)));
}

#[test]
fn union_separated() {
    let mut range = BndRange::with_bounds(1.0, 5.0);
    let other = BndRange::with_bounds(7.0, 10.0);
    assert!(!range.union(&other));
    // range should remain unchanged.
    assert_eq!(range.get_bounds(), Some((1.0, 5.0)));
}

#[test]
fn union_with_void() {
    let mut range = BndRange::with_bounds(1.0, 5.0);
    let void = BndRange::new();
    assert!(!range.union(&void));
}

#[test]
fn is_intersected_void() {
    let range = BndRange::new();
    assert_eq!(range.is_intersected_val(5.0), IntersectStatus::Out);
}

#[test]
fn is_intersected_contains() {
    let range = BndRange::with_bounds(3.0, 15.0);
    assert_eq!(range.is_intersected_val(5.0), IntersectStatus::In);
}

#[test]
fn is_intersected_outside() {
    let range = BndRange::with_bounds(3.0, 15.0);
    assert_eq!(range.is_intersected_val(20.0), IntersectStatus::Out);
    assert_eq!(range.is_intersected_val(1.0), IntersectStatus::Out);
}

#[test]
fn is_intersected_on_boundary() {
    let range = BndRange::with_bounds(3.0, 15.0);
    assert_eq!(range.is_intersected_val(3.0), IntersectStatus::Boundary);
    assert_eq!(range.is_intersected_val(15.0), IntersectStatus::Boundary);
}

#[test]
fn is_intersected_point_range() {
    // When first == last, only Out or Boundary possible.
    let range = BndRange::with_bounds(5.0, 5.0);
    assert_eq!(range.is_intersected_val(5.0), IntersectStatus::Boundary);
    assert_eq!(range.is_intersected_val(3.0), IntersectStatus::Out);
}

#[test]
fn is_intersected_periodic_contains() {
    // Range [3, 15], period 4, val 1; k=1 gives 5, inside [3,15] -> In.
    let range = BndRange::with_bounds(3.0, 15.0);
    assert_eq!(range.is_intersected(1.0, 4.0), IntersectStatus::In);
}

#[test]
fn is_intersected_periodic_on_boundary() {
    // Range [3, 15], period 4, val 3 -> val + 0*4 = 3 exactly.
    let range = BndRange::with_bounds(3.0, 15.0);
    assert_eq!(range.is_intersected(3.0, 4.0), IntersectStatus::Boundary);
}

#[test]
fn is_intersected_periodic_outside() {
    // Range [3.5, 3.9], period 4, val 0; k=1 gives 4.0, not in range -> Out.
    let range = BndRange::with_bounds(3.5, 3.9);
    assert_eq!(range.is_intersected(0.0, 4.0), IntersectStatus::Out);
}

#[test]
fn split_no_intersection() {
    let range = BndRange::with_bounds(3.0, 15.0);
    let mut list: Vec<BndRange> = Vec::new();
    range.split(20.0, &mut list, 0.0);
    assert_eq!(list.len(), 1);
    assert_eq!(list.first().unwrap().get_bounds(), Some((3.0, 15.0)));
}

#[test]
fn split_at_interior_point() {
    let range = BndRange::with_bounds(3.0, 15.0);
    let mut list: Vec<BndRange> = Vec::new();
    range.split(5.0, &mut list, 0.0);
    assert_eq!(list.len(), 2);
    assert_eq!(list.first().unwrap().get_bounds(), Some((3.0, 5.0)));
    assert_eq!(list.last().unwrap().get_bounds(), Some((5.0, 15.0)));
}

#[test]
fn split_periodic() {
    // Range [3, 15], split at val=5, period=4.
    // Split points: 5, 9, 13 -> sub-ranges: [3,5], [5,9], [9,13], [13,15].
    let range = BndRange::with_bounds(3.0, 15.0);
    let mut list: Vec<BndRange> = Vec::new();
    range.split(5.0, &mut list, 4.0);
    assert_eq!(list.len(), 4);
}

#[test]
fn enlarge() {
    let mut range = BndRange::with_bounds(5.0, 10.0);
    range.enlarge(2.0);
    assert_eq!(range.get_bounds(), Some((3.0, 12.0)));
}

#[test]
fn enlarge_void() {
    let mut range = BndRange::new();
    range.enlarge(2.0);
    assert!(range.is_void());
}

#[test]
fn shifted() {
    let range = BndRange::with_bounds(3.0, 7.0);
    let shifted = range.shifted(10.0);
    assert_eq!(shifted.get_bounds(), Some((13.0, 17.0)));
    // Original unchanged.
    assert_eq!(range.get_bounds(), Some((3.0, 7.0)));
}

#[test]
fn shifted_void() {
    let range = BndRange::new();
    let shifted = range.shifted(10.0);
    assert!(shifted.is_void());
}

#[test]
fn shift() {
    let mut range = BndRange::with_bounds(3.0, 7.0);
    range.shift(10.0);
    assert_eq!(range.get_bounds(), Some((13.0, 17.0)));
}

#[test]
fn trim_from() {
    let mut range = BndRange::with_bounds(3.0, 10.0);
    range.trim_from(5.0);
    assert_eq!(range.get_bounds(), Some((5.0, 10.0)));
}

#[test]
fn trim_from_makes_void() {
    let mut range = BndRange::with_bounds(3.0, 10.0);
    range.trim_from(15.0);
    assert!(range.is_void());
}

#[test]
fn trim_to() {
    let mut range = BndRange::with_bounds(3.0, 10.0);
    range.trim_to(7.0);
    assert_eq!(range.get_bounds(), Some((3.0, 7.0)));
}

#[test]
fn trim_to_makes_void() {
    let mut range = BndRange::with_bounds(3.0, 10.0);
    range.trim_to(1.0);
    assert!(range.is_void());
}

#[test]
fn is_out_value() {
    let range = BndRange::with_bounds(3.0, 10.0);
    assert!(!range.is_out(5.0));
    assert!(!range.is_out(3.0));
    assert!(!range.is_out(10.0));
    assert!(range.is_out(1.0));
    assert!(range.is_out(15.0));
}

#[test]
fn is_out_void() {
    let range = BndRange::new();
    assert!(range.is_out(5.0));
}

#[test]
fn is_out_range() {
    let range = BndRange::with_bounds(3.0, 10.0);
    assert!(!range.is_out_range(&BndRange::with_bounds(5.0, 8.0)));
    assert!(!range.is_out_range(&BndRange::with_bounds(1.0, 5.0)));
    assert!(range.is_out_range(&BndRange::with_bounds(11.0, 15.0)));
}

#[test]
fn is_out_range_void() {
    let range = BndRange::with_bounds(3.0, 10.0);
    let void = BndRange::new();
    assert!(range.is_out_range(&void));
    assert!(void.is_out_range(&range));
}

#[test]
fn equality() {
    let range1 = BndRange::with_bounds(3.0, 10.0);
    let range2 = BndRange::with_bounds(3.0, 10.0);
    let range3 = BndRange::with_bounds(3.0, 11.0);
    assert!(range1 == range2);
    assert!(range1 != range3);
}

#[test]
fn contains_value() {
    let range = BndRange::with_bounds(3.0, 10.0);
    assert!(range.contains(5.0));
    assert!(range.contains(3.0));
    assert!(range.contains(10.0));
    assert!(!range.contains(2.0));
    assert!(!range.contains(11.0));

    let void = BndRange::new();
    assert!(!void.contains(5.0));
}

#[test]
fn intersects_range() {
    let range1 = BndRange::with_bounds(3.0, 10.0);
    let range2 = BndRange::with_bounds(5.0, 15.0);
    let range3 = BndRange::with_bounds(11.0, 20.0);
    assert!(range1.intersects(&range2));
    assert!(!range1.intersects(&range3));

    let void = BndRange::new();
    assert!(!range1.intersects(&void));
}

#[test]
fn min_max_direct_access() {
    let range = BndRange::with_bounds(3.0, 10.0);
    assert!(range.min().is_some());
    assert!(range.max().is_some());
    assert_eq!(range.min().unwrap(), 3.0);
    assert_eq!(range.max().unwrap(), 10.0);
}

#[test]
fn min_max_nullopt_on_void() {
    let void = BndRange::new();
    assert!(void.min().is_none());
    assert!(void.max().is_none());
}

#[test]
fn get_structured_bindings() {
    let range = BndRange::with_bounds(3.0, 10.0);
    let bounds = range.get();
    assert!(bounds.is_some());
    let bounds = bounds.unwrap();
    assert_eq!(bounds.min, 3.0);
    assert_eq!(bounds.max, 10.0);
}

#[test]
fn get_nullopt_on_void() {
    let void = BndRange::new();
    assert!(void.get().is_none());
}

#[test]
fn center() {
    let range = BndRange::with_bounds(2.0, 8.0);
    assert!(range.center().is_some());
    assert_eq!(range.center().unwrap(), 5.0);

    let point = BndRange::with_bounds(3.0, 3.0);
    assert!(point.center().is_some());
    assert_eq!(point.center().unwrap(), 3.0);
}

#[test]
fn center_nullopt_on_void() {
    let void = BndRange::new();
    assert!(void.center().is_none());
}
