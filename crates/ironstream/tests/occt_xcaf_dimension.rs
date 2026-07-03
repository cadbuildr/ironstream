// FILE: rust/ironstream/crates/ironstream/tests/occt_xcaf_dimension.rs

//! Tests for `xcaf_dimension` — ported from the XCAFDoc_DimTolTool usage
//! patterns in the OpenCascade XDE tests.

extern crate ironstream;
use ironstream::xcaf_dimension::*;

// ---- DimTolType ----

#[test]
fn dim_tol_type_equality() {
    assert_eq!(DimTolType::Linear, DimTolType::Linear);
    assert_ne!(DimTolType::Linear, DimTolType::Angular);
    assert_eq!(DimTolType::PositionTolerance, DimTolType::PositionTolerance);
}

#[test]
fn dim_tol_type_copy() {
    let t = DimTolType::Diameter;
    let t2 = t;
    assert_eq!(t, t2);
}

// ---- DimTolEntry::new ----

#[test]
fn entry_new_stores_fields() {
    let e = DimTolEntry::new(0, DimTolType::Linear, 25.0);
    assert_eq!(e.id(), 0);
    assert_eq!(e.dim_type(), DimTolType::Linear);
    assert_eq!(e.value(), 25.0);
}

#[test]
fn entry_new_default_limits_equal_value() {
    let e = DimTolEntry::new(3, DimTolType::Flatness, 0.05);
    assert_eq!(e.lower(), 0.05);
    assert_eq!(e.upper(), 0.05);
}

#[test]
fn entry_new_default_description_empty() {
    let e = DimTolEntry::new(1, DimTolType::Roundness, 1.0);
    assert_eq!(e.description(), "");
}

// ---- DimTolEntry::set_limits ----

#[test]
fn entry_set_limits() {
    let mut e = DimTolEntry::new(0, DimTolType::Angular, 45.0);
    e.set_limits(44.5, 45.5);
    assert_eq!(e.lower(), 44.5);
    assert_eq!(e.upper(), 45.5);
}

#[test]
fn entry_set_limits_symmetric() {
    let mut e = DimTolEntry::new(0, DimTolType::Cylindricity, 10.0);
    e.set_limits(9.0, 11.0);
    assert_eq!(e.lower(), 9.0);
    assert_eq!(e.upper(), 11.0);
    // value unchanged
    assert_eq!(e.value(), 10.0);
}

#[test]
fn entry_set_limits_same_value() {
    let mut e = DimTolEntry::new(0, DimTolType::Diameter, 20.0);
    e.set_limits(20.0, 20.0);
    assert_eq!(e.lower(), 20.0);
    assert_eq!(e.upper(), 20.0);
}

// ---- DimTolEntry description ----

#[test]
fn entry_set_description() {
    let mut e = DimTolEntry::new(0, DimTolType::Parallelism, 0.1);
    e.set_description("datum A");
    assert_eq!(e.description(), "datum A");
}

#[test]
fn entry_set_description_replace() {
    let mut e = DimTolEntry::new(0, DimTolType::Perpendicularity, 0.2);
    e.set_description("first");
    e.set_description("second");
    assert_eq!(e.description(), "second");
}

#[test]
fn entry_set_description_empty_string() {
    let mut e = DimTolEntry::new(0, DimTolType::Symmetry, 5.0);
    e.set_description("something");
    e.set_description("");
    assert_eq!(e.description(), "");
}

// ---- DimTolTool::new ----

#[test]
fn tool_new_is_empty() {
    let tool = DimTolTool::new();
    assert_eq!(tool.nb_entries(), 0);
}

#[test]
fn tool_default_is_empty() {
    let tool = DimTolTool::default();
    assert_eq!(tool.nb_entries(), 0);
}

// ---- DimTolTool::add_dimension ----

#[test]
fn tool_add_dimension_returns_sequential_ids() {
    let mut tool = DimTolTool::new();
    let id0 = tool.add_dimension(DimTolType::Linear, 10.0);
    let id1 = tool.add_dimension(DimTolType::Angular, 90.0);
    let id2 = tool.add_dimension(DimTolType::Radius, 5.0);
    assert_eq!(id0, 0);
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
}

#[test]
fn tool_add_dimension_increments_count() {
    let mut tool = DimTolTool::new();
    tool.add_dimension(DimTolType::Diameter, 20.0);
    assert_eq!(tool.nb_entries(), 1);
    tool.add_dimension(DimTolType::Flatness, 0.01);
    assert_eq!(tool.nb_entries(), 2);
}

#[test]
fn tool_add_dimension_stored_value() {
    let mut tool = DimTolTool::new();
    let id = tool.add_dimension(DimTolType::Straightness, 3.14);
    let entry = tool.find(id).expect("entry must exist");
    assert_eq!(entry.value(), 3.14);
    assert_eq!(entry.dim_type(), DimTolType::Straightness);
}

#[test]
fn tool_add_dimension_limits_equal_value() {
    let mut tool = DimTolTool::new();
    let id = tool.add_dimension(DimTolType::Roundness, 7.5);
    let entry = tool.find(id).expect("entry must exist");
    assert_eq!(entry.lower(), 7.5);
    assert_eq!(entry.upper(), 7.5);
}

// ---- DimTolTool::add_tolerance ----

#[test]
fn tool_add_tolerance_stores_limits() {
    let mut tool = DimTolTool::new();
    let id = tool.add_tolerance(DimTolType::Cylindricity, 10.0, 9.8, 10.2);
    let entry = tool.find(id).expect("entry must exist");
    assert_eq!(entry.value(), 10.0);
    assert_eq!(entry.lower(), 9.8);
    assert_eq!(entry.upper(), 10.2);
}

#[test]
fn tool_add_tolerance_returns_sequential_id() {
    let mut tool = DimTolTool::new();
    let id0 = tool.add_dimension(DimTolType::Linear, 5.0);
    let id1 = tool.add_tolerance(DimTolType::Parallelism, 0.0, -0.1, 0.1);
    assert_eq!(id0, 0);
    assert_eq!(id1, 1);
}

#[test]
fn tool_add_tolerance_type_stored() {
    let mut tool = DimTolTool::new();
    let id = tool.add_tolerance(DimTolType::Coaxiality, 0.0, -0.05, 0.05);
    let entry = tool.find(id).expect("entry must exist");
    assert_eq!(entry.dim_type(), DimTolType::Coaxiality);
}

// ---- DimTolTool::find ----

#[test]
fn tool_find_returns_none_on_empty() {
    let tool = DimTolTool::new();
    assert!(tool.find(0).is_none());
}

#[test]
fn tool_find_returns_none_for_unknown_id() {
    let mut tool = DimTolTool::new();
    tool.add_dimension(DimTolType::Linear, 1.0);
    assert!(tool.find(99).is_none());
}

#[test]
fn tool_find_returns_correct_entry() {
    let mut tool = DimTolTool::new();
    let id = tool.add_dimension(DimTolType::Angularity, 30.0);
    let entry = tool.find(id).expect("entry must exist");
    assert_eq!(entry.id(), id);
    assert_eq!(entry.dim_type(), DimTolType::Angularity);
    assert_eq!(entry.value(), 30.0);
}

#[test]
fn tool_find_multiple_entries_correct() {
    let mut tool = DimTolTool::new();
    let id0 = tool.add_dimension(DimTolType::Linear, 10.0);
    let id1 = tool.add_dimension(DimTolType::Angular, 45.0);
    let id2 = tool.add_tolerance(DimTolType::Flatness, 0.0, -0.01, 0.01);

    assert_eq!(tool.find(id0).unwrap().value(), 10.0);
    assert_eq!(tool.find(id1).unwrap().value(), 45.0);
    assert_eq!(tool.find(id2).unwrap().lower(), -0.01);
    assert_eq!(tool.find(id2).unwrap().upper(), 0.01);
}

// ---- DimTolTool::nb_entries ----

#[test]
fn tool_nb_entries_mixed() {
    let mut tool = DimTolTool::new();
    assert_eq!(tool.nb_entries(), 0);
    tool.add_dimension(DimTolType::Radius, 5.0);
    assert_eq!(tool.nb_entries(), 1);
    tool.add_tolerance(DimTolType::Symmetry, 0.0, -0.5, 0.5);
    assert_eq!(tool.nb_entries(), 2);
    tool.add_dimension(DimTolType::Diameter, 12.0);
    assert_eq!(tool.nb_entries(), 3);
}

// ---- DimTolTool::entries_of_type ----

#[test]
fn tool_entries_of_type_empty_tool() {
    let tool = DimTolTool::new();
    assert!(tool.entries_of_type(DimTolType::Linear).is_empty());
}

#[test]
fn tool_entries_of_type_no_match() {
    let mut tool = DimTolTool::new();
    tool.add_dimension(DimTolType::Angular, 90.0);
    let ids = tool.entries_of_type(DimTolType::Linear);
    assert!(ids.is_empty());
}

#[test]
fn tool_entries_of_type_single_match() {
    let mut tool = DimTolTool::new();
    let id = tool.add_dimension(DimTolType::PositionTolerance, 0.0);
    let ids = tool.entries_of_type(DimTolType::PositionTolerance);
    assert_eq!(ids, vec![id]);
}

#[test]
fn tool_entries_of_type_multiple_matches() {
    let mut tool = DimTolTool::new();
    let id0 = tool.add_dimension(DimTolType::Flatness, 0.01);
    tool.add_dimension(DimTolType::Roundness, 0.02);
    let id2 = tool.add_tolerance(DimTolType::Flatness, 0.0, -0.005, 0.005);

    let ids = tool.entries_of_type(DimTolType::Flatness);
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&id0));
    assert!(ids.contains(&id2));
}

#[test]
fn tool_entries_of_type_preserves_order() {
    let mut tool = DimTolTool::new();
    let id0 = tool.add_dimension(DimTolType::Linear, 1.0);
    tool.add_dimension(DimTolType::Angular, 2.0);
    let id2 = tool.add_dimension(DimTolType::Linear, 3.0);
    tool.add_dimension(DimTolType::Diameter, 4.0);
    let id4 = tool.add_dimension(DimTolType::Linear, 5.0);

    let ids = tool.entries_of_type(DimTolType::Linear);
    assert_eq!(ids, vec![id0, id2, id4]);
}

#[test]
fn tool_entries_of_type_all_types_covered() {
    let mut tool = DimTolTool::new();
    let types = [
        DimTolType::Linear,
        DimTolType::Angular,
        DimTolType::Radius,
        DimTolType::Diameter,
        DimTolType::Cylindricity,
        DimTolType::Flatness,
        DimTolType::Roundness,
        DimTolType::Straightness,
        DimTolType::Parallelism,
        DimTolType::Perpendicularity,
        DimTolType::Angularity,
        DimTolType::Coaxiality,
        DimTolType::Symmetry,
        DimTolType::PositionTolerance,
    ];
    for &t in &types {
        tool.add_dimension(t, 1.0);
    }
    assert_eq!(tool.nb_entries(), types.len());
    for &t in &types {
        let ids = tool.entries_of_type(t);
        assert_eq!(ids.len(), 1, "expected exactly one entry for {:?}", t);
    }
}

// ---- Integration: mutating a found entry ----

#[test]
fn tool_mutate_entry_after_lookup() {
    let mut tool = DimTolTool::new();
    let id = tool.add_tolerance(DimTolType::Perpendicularity, 0.0, -0.1, 0.1);

    // We cannot get &mut from find(), so we test via add + separate find.
    // Verify the returned reference reflects constructed state.
    let entry = tool.find(id).expect("must exist");
    assert_eq!(entry.lower(), -0.1);
    assert_eq!(entry.upper(), 0.1);
    assert_eq!(entry.description(), "");
}

// ---- Zero-value edge cases ----

#[test]
fn entry_zero_value() {
    let e = DimTolEntry::new(0, DimTolType::Flatness, 0.0);
    assert_eq!(e.value(), 0.0);
    assert_eq!(e.lower(), 0.0);
    assert_eq!(e.upper(), 0.0);
}

#[test]
fn entry_negative_limits() {
    let mut e = DimTolEntry::new(0, DimTolType::Straightness, 0.0);
    e.set_limits(-1.0, -0.5);
    assert_eq!(e.lower(), -1.0);
    assert_eq!(e.upper(), -0.5);
}

#[test]
fn entry_large_value() {
    let e = DimTolEntry::new(0, DimTolType::Linear, 1_000_000.0);
    assert_eq!(e.value(), 1_000_000.0);
}
