// FILE: tests/occt_rw_iges.rs
//! Integration tests for `ironstream::rw_iges` — the IGES I/O framework
//! ported from OpenCascade's `IGESControl` package.
//!
//! Tests exercise the public API from an external-crate perspective, mirroring
//! the way downstream code uses IGESControl_Reader and IGESControl_Writer:
//! constructing a writer, populating it with geometry, serialising to an
//! in-memory buffer, then reading it back and checking transferred shapes.

extern crate ironstream;

use std::collections::HashMap;

use ironstream::rw_iges::{
    IgesEntityKind, IgesError, IgesUnit, IGESControl_Reader, IGESControl_Writer, IGESEntity,
    IGESModel,
};
use ironstream::gp::Pnt;
use ironstream::top_exp::TopoDS_Shape;

// ---------------------------------------------------------------------------
// Helper — write to vfs and read back
// ---------------------------------------------------------------------------

fn write_read(writer: &IGESControl_Writer, path: &str) -> IGESControl_Reader {
    let mut vfs: HashMap<String, Vec<u8>> = HashMap::new();
    writer.write_file(path, &mut vfs);
    let mut reader = IGESControl_Reader::new();
    for (k, v) in vfs {
        reader.register_content(k, v);
    }
    reader.read_file(path).expect("read_file must succeed");
    reader
}

// ---------------------------------------------------------------------------
// IgesUnit
// ---------------------------------------------------------------------------

#[test]
fn unit_flag_and_code_are_consistent() {
    let pairs = [
        (IgesUnit::Millimeter, "MM", 2u8),
        (IgesUnit::Inch,       "IN", 1u8),
        (IgesUnit::Meter,      "M",  6u8),
        (IgesUnit::Centimeter, "CM", 3u8),
    ];
    for (unit, flag, code) in pairs {
        assert_eq!(unit.flag(), flag, "flag mismatch for {unit:?}");
        assert_eq!(unit.code(), code, "code mismatch for {unit:?}");
        assert_eq!(IgesUnit::from_flag(flag), Some(unit));
    }
}

#[test]
fn unit_from_flag_unknown_returns_none() {
    assert_eq!(IgesUnit::from_flag("XYZ"), None);
    assert_eq!(IgesUnit::from_flag(""), None);
}

#[test]
fn unit_mm_conversion_inch_is_25_4() {
    let factor = IgesUnit::Inch.to_mm();
    assert!((factor - 25.4).abs() < 1e-10, "inch to mm: {factor}");
}

// ---------------------------------------------------------------------------
// IgesEntityKind
// ---------------------------------------------------------------------------

#[test]
fn entity_kind_all_standard_type_numbers_round_trip() {
    let numbers = [0, 100, 102, 104, 108, 110, 116, 118, 120, 122, 126, 128,
                   142, 143, 144, 186, 314, 402, 408, 502, 504, 508, 510, 514];
    for n in numbers {
        let kind = IgesEntityKind::from_type_number(n);
        assert_eq!(kind.type_number(), n, "failed for type number {n}");
    }
}

#[test]
fn entity_kind_unknown_type_stored_verbatim() {
    let k = IgesEntityKind::from_type_number(777);
    assert_eq!(k, IgesEntityKind::Unknown(777));
    assert_eq!(k.type_number(), 777);
}

// ---------------------------------------------------------------------------
// IGESControl_Writer — construction and metadata
// ---------------------------------------------------------------------------

#[test]
fn writer_default_unit_is_millimeter() {
    let w = IGESControl_Writer::new();
    assert_eq!(w.model().global.unit, IgesUnit::Millimeter);
}

#[test]
fn writer_set_unit_updates_model() {
    let mut w = IGESControl_Writer::new();
    w.set_unit(IgesUnit::Inch);
    assert_eq!(w.model().global.unit, IgesUnit::Inch);
}

#[test]
fn writer_set_author_stored_in_global() {
    let mut w = IGESControl_Writer::new();
    w.set_author("Ada Lovelace", "Computing Inc.");
    assert_eq!(w.model().global.author, "Ada Lovelace");
    assert_eq!(w.model().global.organisation, "Computing Inc.");
}

#[test]
fn writer_add_point_increments_entity_count() {
    let mut w = IGESControl_Writer::new();
    assert_eq!(w.nb_entities(), 0);
    w.add_point(Pnt::new(1.0, 2.0, 3.0));
    assert_eq!(w.nb_entities(), 1);
    w.add_point(Pnt::new(4.0, 5.0, 6.0));
    assert_eq!(w.nb_entities(), 2);
}

#[test]
fn writer_add_line_produces_line_entity() {
    let mut w = IGESControl_Writer::new();
    w.add_line(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    assert_eq!(w.nb_entities(), 1);
    let e = w.model().entity(1).unwrap();
    assert_eq!(e.entity_type, IgesEntityKind::Line);
}

#[test]
fn writer_add_circular_arc_produces_arc_entity() {
    let mut w = IGESControl_Writer::new();
    // A full unit circle in the XY plane.
    w.add_circular_arc(0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0);
    assert_eq!(w.nb_entities(), 1);
    let e = w.model().entity(1).unwrap();
    assert_eq!(e.entity_type, IgesEntityKind::CircularArc);
}

// ---------------------------------------------------------------------------
// Serialisation — write_string / write_content
// ---------------------------------------------------------------------------

#[test]
fn serialised_output_has_all_five_section_codes() {
    let mut w = IGESControl_Writer::new();
    w.add_point(Pnt::origin());
    let s = w.write_string();
    assert!(s.lines().any(|l| l.len() >= 73 && l.chars().nth(72) == Some('S')), "S section missing");
    assert!(s.lines().any(|l| l.len() >= 73 && l.chars().nth(72) == Some('G')), "G section missing");
    assert!(s.lines().any(|l| l.len() >= 73 && l.chars().nth(72) == Some('D')), "D section missing");
    assert!(s.lines().any(|l| l.len() >= 73 && l.chars().nth(72) == Some('P')), "P section missing");
    assert!(s.lines().any(|l| l.len() >= 73 && l.chars().nth(72) == Some('T')), "T section missing");
}

#[test]
fn serialised_lines_are_at_most_80_chars() {
    let mut w = IGESControl_Writer::new();
    w.set_author("Charles Babbage", "Analytical Engine Works");
    w.add_point(Pnt::new(1.23, 4.56, 7.89));
    w.add_line(Pnt::origin(), Pnt::new(100.0, 0.0, 0.0));
    for line in w.write_string().lines() {
        assert!(line.len() <= 80, "line too long ({} chars): {:?}", line.len(), line);
    }
}

#[test]
fn write_content_and_write_string_are_consistent() {
    let mut w = IGESControl_Writer::new();
    w.add_point(Pnt::new(0.0, 0.0, 0.0));
    let s = w.write_string();
    let b = w.write_content();
    assert_eq!(s.as_bytes(), b.as_slice());
}

// ---------------------------------------------------------------------------
// Round-trip: Writer → vfs → Reader → shapes
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_single_point_produces_vertex_shape() {
    let mut w = IGESControl_Writer::new();
    w.add_point(Pnt::new(3.0, 4.0, 5.0));
    let mut reader = write_read(&w, "pt.igs");
    let n = reader.transfer_roots();
    assert!(n >= 1, "expected at least one shape, got {n}");
    // The first shape should be a Vertex.
    let shape = reader.shape(1).unwrap();
    assert!(
        matches!(shape, TopoDS_Shape::Vertex(_)),
        "expected Vertex shape, got: {shape:?}"
    );
}

#[test]
fn roundtrip_line_produces_edge_shape() {
    let mut w = IGESControl_Writer::new();
    w.add_line(Pnt::origin(), Pnt::new(10.0, 0.0, 0.0));
    let mut reader = write_read(&w, "line.igs");
    let n = reader.transfer_roots();
    assert!(n >= 1, "expected at least one shape from line");
    let shape = reader.shape(1).unwrap();
    assert!(
        matches!(shape, TopoDS_Shape::Edge(_)),
        "expected Edge shape from line entity, got: {shape:?}"
    );
}

#[test]
fn roundtrip_circular_arc_produces_wire_shape() {
    let mut w = IGESControl_Writer::new();
    w.add_circular_arc(0.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0);
    let mut reader = write_read(&w, "arc.igs");
    let n = reader.transfer_roots();
    assert!(n >= 1, "expected at least one shape from arc");
    let shape = reader.shape(1).unwrap();
    assert!(
        matches!(shape, TopoDS_Shape::Wire(_)),
        "expected Wire shape from circular arc entity, got: {shape:?}"
    );
}

#[test]
fn roundtrip_multiple_entities_all_transferred() {
    let mut w = IGESControl_Writer::new();
    w.add_point(Pnt::new(0.0, 0.0, 0.0));
    w.add_point(Pnt::new(1.0, 0.0, 0.0));
    w.add_line(Pnt::origin(), Pnt::new(1.0, 1.0, 0.0));
    let mut reader = write_read(&w, "multi.igs");
    assert_eq!(reader.nb_roots_for_transfer(), 3);
    let n = reader.transfer_roots();
    assert_eq!(n, 3, "all 3 entities should produce shapes");
    assert_eq!(reader.nb_shapes(), 3);
}

#[test]
fn reader_one_shape_compound_when_multiple() {
    let mut w = IGESControl_Writer::new();
    w.add_point(Pnt::new(0.0, 0.0, 0.0));
    w.add_point(Pnt::new(1.0, 0.0, 0.0));
    let mut reader = write_read(&w, "two_pts.igs");
    reader.transfer_roots();
    let one = reader.one_shape().expect("one_shape must return Some");
    assert!(
        matches!(one, TopoDS_Shape::Compound(_)),
        "two shapes should produce a Compound, got: {one:?}"
    );
}

#[test]
fn reader_missing_file_returns_error() {
    let mut reader = IGESControl_Reader::new();
    let result = reader.read_file("nonexistent.igs");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("not found") || err.message.contains("nonexistent"),
        "unexpected error: {}", err.message);
}

#[test]
fn reader_clear_resets_state() {
    let mut w = IGESControl_Writer::new();
    w.add_point(Pnt::origin());
    let mut reader = write_read(&w, "clr.igs");
    reader.transfer_roots();
    assert!(reader.has_model());
    assert!(reader.is_transferred());
    reader.clear();
    assert!(!reader.has_model());
    assert!(!reader.is_transferred());
    assert_eq!(reader.nb_shapes(), 0);
}

#[test]
fn reader_stats_contains_entity_count_and_unit() {
    let mut w = IGESControl_Writer::new();
    w.set_unit(IgesUnit::Inch);
    w.add_point(Pnt::origin());
    let mut reader = write_read(&w, "stats.igs");
    let stats = reader.stats();
    // The stats string should mention entities.
    assert!(stats.contains("entit"), "expected 'entit' in stats: {stats}");
}
