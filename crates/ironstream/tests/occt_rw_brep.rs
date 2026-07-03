// FILE: rust/ironstream/crates/ironstream/tests/occt_rw_brep.rs
//! Integration tests for `ironstream::rw_brep` — BRep read/write parameters
//! and result types ported from OpenCascade's `BRepTools` package.
//!
//! Tests exercise the public API from an external-crate perspective, verifying
//! that write params, read results, header generation, and header parsing all
//! behave as documented.

extern crate ironstream;

use ironstream::rw_brep::{
    parse_brep_counts, write_brep_header, BrepIO, BrepReadResult, BrepWriteParams,
};

// ---------------------------------------------------------------------------
// BrepWriteParams
// ---------------------------------------------------------------------------

#[test]
fn write_params_new_default_values() {
    let p = BrepWriteParams::new();
    assert!(!p.triangulate(), "triangulate should default to false");
    assert!(!p.relative_deflection(), "relative_deflection should default to false");
    assert_eq!(p.deflection(), 0.001);
    assert_eq!(p.angle(), 0.5);
}

#[test]
fn write_params_default_trait_matches_new() {
    let a = BrepWriteParams::new();
    let b = BrepWriteParams::default();
    assert_eq!(a.deflection(), b.deflection());
    assert_eq!(a.angle(), b.angle());
    assert_eq!(a.triangulate(), b.triangulate());
    assert_eq!(a.relative_deflection(), b.relative_deflection());
}

#[test]
fn write_params_set_triangulate_true() {
    let mut p = BrepWriteParams::new();
    p.set_triangulate(true);
    assert!(p.triangulate());
}

#[test]
fn write_params_set_triangulate_false() {
    let mut p = BrepWriteParams::new();
    p.set_triangulate(true);
    p.set_triangulate(false);
    assert!(!p.triangulate());
}

#[test]
fn write_params_set_deflection() {
    let mut p = BrepWriteParams::new();
    p.set_deflection(0.05);
    assert_eq!(p.deflection(), 0.05);
}

#[test]
fn write_params_set_deflection_zero() {
    let mut p = BrepWriteParams::new();
    p.set_deflection(0.0);
    assert_eq!(p.deflection(), 0.0);
}

#[test]
fn write_params_set_angle() {
    let mut p = BrepWriteParams::new();
    p.set_angle(1.0);
    assert_eq!(p.angle(), 1.0);
}

#[test]
fn write_params_set_relative_deflection_true() {
    let mut p = BrepWriteParams::new();
    p.set_relative_deflection(true);
    assert!(p.relative_deflection());
}

#[test]
fn write_params_set_relative_deflection_false() {
    let mut p = BrepWriteParams::new();
    p.set_relative_deflection(true);
    p.set_relative_deflection(false);
    assert!(!p.relative_deflection());
}

#[test]
fn write_params_clone_is_independent() {
    let mut a = BrepWriteParams::new();
    let b = a.clone();
    a.set_deflection(99.9);
    // b retains the original value
    assert_eq!(b.deflection(), 0.001);
    assert_eq!(a.deflection(), 99.9);
}

#[test]
fn write_params_debug_available() {
    let p = BrepWriteParams::new();
    let _ = format!("{:?}", p);
}

// ---------------------------------------------------------------------------
// BrepReadResult
// ---------------------------------------------------------------------------

#[test]
fn read_result_new_all_zero() {
    let r = BrepReadResult::new();
    assert_eq!(r.nb_vertices(), 0);
    assert_eq!(r.nb_edges(), 0);
    assert_eq!(r.nb_faces(), 0);
    assert_eq!(r.nb_shells(), 0);
    assert_eq!(r.nb_solids(), 0);
}

#[test]
fn read_result_new_not_valid() {
    let r = BrepReadResult::new();
    assert!(!r.is_valid());
}

#[test]
fn read_result_default_equals_new() {
    let a = BrepReadResult::new();
    let b = BrepReadResult::default();
    assert_eq!(a, b);
}

#[test]
fn read_result_set_counts_box() {
    let mut r = BrepReadResult::new();
    // A box has 8 vertices, 12 edges, 6 faces, 1 shell, 1 solid.
    r.set_counts(8, 12, 6, 1, 1);
    assert_eq!(r.nb_vertices(), 8);
    assert_eq!(r.nb_edges(), 12);
    assert_eq!(r.nb_faces(), 6);
    assert_eq!(r.nb_shells(), 1);
    assert_eq!(r.nb_solids(), 1);
}

#[test]
fn read_result_set_counts_zeroes() {
    let mut r = BrepReadResult::new();
    r.set_counts(0, 0, 0, 0, 0);
    assert_eq!(r.nb_vertices(), 0);
    assert_eq!(r.nb_solids(), 0);
}

#[test]
fn read_result_set_valid_true() {
    let mut r = BrepReadResult::new();
    r.set_valid(true);
    assert!(r.is_valid());
}

#[test]
fn read_result_set_valid_false() {
    let mut r = BrepReadResult::new();
    r.set_valid(true);
    r.set_valid(false);
    assert!(!r.is_valid());
}

#[test]
fn read_result_clone_is_independent() {
    let mut a = BrepReadResult::new();
    a.set_counts(1, 2, 3, 4, 5);
    let b = a.clone();
    a.set_counts(0, 0, 0, 0, 0);
    assert_eq!(b.nb_vertices(), 1);
    assert_eq!(a.nb_vertices(), 0);
}

#[test]
fn read_result_equality() {
    let mut a = BrepReadResult::new();
    let mut b = BrepReadResult::new();
    a.set_counts(2, 3, 4, 1, 1);
    b.set_counts(2, 3, 4, 1, 1);
    assert_eq!(a, b);
}

#[test]
fn read_result_debug_available() {
    let r = BrepReadResult::new();
    let _ = format!("{:?}", r);
}

// ---------------------------------------------------------------------------
// write_brep_header
// ---------------------------------------------------------------------------

#[test]
fn write_brep_header_magic_line() {
    let p = BrepWriteParams::new();
    let h = write_brep_header(&p);
    assert!(
        h.contains("CASCADE Topology V1"),
        "expected magic string in: {h}"
    );
}

#[test]
fn write_brep_header_default_deflection() {
    let p = BrepWriteParams::new();
    let h = write_brep_header(&p);
    assert!(h.contains("deflection=0.001"), "header: {h}");
}

#[test]
fn write_brep_header_default_angle() {
    let p = BrepWriteParams::new();
    let h = write_brep_header(&p);
    assert!(h.contains("angle=0.5"), "header: {h}");
}

#[test]
fn write_brep_header_custom_deflection() {
    let mut p = BrepWriteParams::new();
    p.set_deflection(0.123);
    let h = write_brep_header(&p);
    assert!(h.contains("deflection=0.123"), "header: {h}");
}

#[test]
fn write_brep_header_custom_angle() {
    let mut p = BrepWriteParams::new();
    p.set_angle(1.2);
    let h = write_brep_header(&p);
    assert!(h.contains("angle=1.2"), "header: {h}");
}

#[test]
fn write_brep_header_triangulate_on() {
    let mut p = BrepWriteParams::new();
    p.set_triangulate(true);
    let h = write_brep_header(&p);
    assert!(h.contains("triangulate=1"), "header: {h}");
}

#[test]
fn write_brep_header_triangulate_off() {
    let p = BrepWriteParams::new();
    let h = write_brep_header(&p);
    assert!(h.contains("triangulate=0"), "header: {h}");
}

#[test]
fn write_brep_header_relative_on() {
    let mut p = BrepWriteParams::new();
    p.set_relative_deflection(true);
    let h = write_brep_header(&p);
    assert!(h.contains("relative=1"), "header: {h}");
}

#[test]
fn write_brep_header_relative_off() {
    let p = BrepWriteParams::new();
    let h = write_brep_header(&p);
    assert!(h.contains("relative=0"), "header: {h}");
}

#[test]
fn write_brep_header_count_tokens_present() {
    let p = BrepWriteParams::new();
    let h = write_brep_header(&p);
    assert!(h.contains("V="), "header: {h}");
    assert!(h.contains("E="), "header: {h}");
    assert!(h.contains("F="), "header: {h}");
    assert!(h.contains("SH="), "header: {h}");
    assert!(h.contains("SO="), "header: {h}");
}

// ---------------------------------------------------------------------------
// parse_brep_counts
// ---------------------------------------------------------------------------

#[test]
fn parse_brep_counts_simple_box() {
    let header = "CASCADE Topology V1, (c) Matra-Datavision\ndeflection=0.001 angle=0.5 triangulate=0 relative=0\nV=8 E=12 F=6 SH=1 SO=1";
    let r = parse_brep_counts(header).expect("should parse box counts");
    assert_eq!(r.nb_vertices(), 8);
    assert_eq!(r.nb_edges(), 12);
    assert_eq!(r.nb_faces(), 6);
    assert_eq!(r.nb_shells(), 1);
    assert_eq!(r.nb_solids(), 1);
}

#[test]
fn parse_brep_counts_valid_flag_set_when_nonempty() {
    let header = "V=8 E=12 F=6 SH=1 SO=1";
    let r = parse_brep_counts(header).expect("should parse");
    assert!(r.is_valid());
}

#[test]
fn parse_brep_counts_not_valid_when_all_zero() {
    let header = "V=0 E=0 F=0 SH=0 SO=0";
    let r = parse_brep_counts(header).expect("should parse zero counts");
    assert!(!r.is_valid());
}

#[test]
fn parse_brep_counts_valid_when_only_vertices() {
    let header = "V=4 E=0 F=0 SH=0 SO=0";
    let r = parse_brep_counts(header).expect("should parse");
    assert!(r.is_valid());
}

#[test]
fn parse_brep_counts_valid_when_only_edges() {
    let header = "V=0 E=5 F=0 SH=0 SO=0";
    let r = parse_brep_counts(header).expect("should parse");
    assert!(r.is_valid());
}

#[test]
fn parse_brep_counts_valid_when_only_faces() {
    let header = "V=0 E=0 F=2 SH=0 SO=0";
    let r = parse_brep_counts(header).expect("should parse");
    assert!(r.is_valid());
}

#[test]
fn parse_brep_counts_none_when_empty_string() {
    assert!(parse_brep_counts("").is_none());
}

#[test]
fn parse_brep_counts_none_when_no_count_line() {
    assert!(parse_brep_counts("CASCADE Topology V1\nno counts here").is_none());
}

#[test]
fn parse_brep_counts_none_when_missing_so() {
    // Missing SO= token.
    assert!(parse_brep_counts("V=4 E=6 F=4 SH=1").is_none());
}

#[test]
fn parse_brep_counts_none_when_missing_sh() {
    assert!(parse_brep_counts("V=4 E=6 F=4 SO=1").is_none());
}

#[test]
fn parse_brep_counts_large_values() {
    let header = "V=1000 E=2000 F=3000 SH=500 SO=100";
    let r = parse_brep_counts(header).expect("should parse large counts");
    assert_eq!(r.nb_vertices(), 1000);
    assert_eq!(r.nb_edges(), 2000);
    assert_eq!(r.nb_faces(), 3000);
    assert_eq!(r.nb_shells(), 500);
    assert_eq!(r.nb_solids(), 100);
}

#[test]
fn parse_brep_counts_multiline_finds_correct_line() {
    let header = concat!(
        "CASCADE Topology V1, (c) Matra-Datavision\n",
        "deflection=0.001 angle=0.5 triangulate=0 relative=0\n",
        "some other metadata\n",
        "V=2 E=3 F=1 SH=1 SO=0\n",
        "trailing line"
    );
    let r = parse_brep_counts(header).expect("should parse");
    assert_eq!(r.nb_vertices(), 2);
    assert_eq!(r.nb_edges(), 3);
    assert_eq!(r.nb_faces(), 1);
}

// ---------------------------------------------------------------------------
// BrepIO
// ---------------------------------------------------------------------------

#[test]
fn brep_io_new_write_params_defaults() {
    let io = BrepIO::new();
    assert_eq!(io.write_params.deflection(), 0.001);
    assert_eq!(io.write_params.angle(), 0.5);
    assert!(!io.write_params.triangulate());
    assert!(!io.write_params.relative_deflection());
}

#[test]
fn brep_io_default_equals_new() {
    let a = BrepIO::new();
    let b = BrepIO::default();
    assert_eq!(
        a.write_params.deflection(),
        b.write_params.deflection()
    );
    assert_eq!(a.write_params.angle(), b.write_params.angle());
}

#[test]
fn brep_io_write_header_magic() {
    let io = BrepIO::new();
    let h = io.write_header();
    assert!(h.contains("CASCADE Topology V1"), "header: {h}");
}

#[test]
fn brep_io_write_header_reflects_custom_params() {
    let mut io = BrepIO::new();
    io.write_params.set_deflection(0.007);
    io.write_params.set_angle(0.9);
    io.write_params.set_triangulate(true);
    let h = io.write_header();
    assert!(h.contains("deflection=0.007"), "header: {h}");
    assert!(h.contains("angle=0.9"), "header: {h}");
    assert!(h.contains("triangulate=1"), "header: {h}");
}

#[test]
fn brep_io_read_header_parses_box() {
    let io = BrepIO::new();
    let s = "V=8 E=12 F=6 SH=1 SO=1";
    let r = io.read_header(s).expect("should parse");
    assert_eq!(r.nb_vertices(), 8);
    assert_eq!(r.nb_edges(), 12);
    assert_eq!(r.nb_faces(), 6);
    assert_eq!(r.nb_shells(), 1);
    assert_eq!(r.nb_solids(), 1);
    assert!(r.is_valid());
}

#[test]
fn brep_io_read_header_none_on_empty() {
    let io = BrepIO::new();
    assert!(io.read_header("").is_none());
}

#[test]
fn brep_io_read_header_none_on_garbage() {
    let io = BrepIO::new();
    assert!(io.read_header("not a brep header at all").is_none());
}

#[test]
fn brep_io_write_then_read_roundtrip() {
    let mut io = BrepIO::new();
    io.write_params.set_deflection(0.002);
    io.write_params.set_angle(0.4);
    let header = io.write_header();
    // write_header always emits V=0..SO=0; reading back should succeed.
    let r = io.read_header(&header).expect("roundtrip should parse");
    assert_eq!(r.nb_vertices(), 0);
    assert_eq!(r.nb_solids(), 0);
    assert!(!r.is_valid());
}

#[test]
fn brep_io_debug_available() {
    let io = BrepIO::new();
    let _ = format!("{:?}", io);
}

#[test]
fn brep_io_clone_is_independent() {
    let mut io = BrepIO::new();
    let _clone = io.clone();
    io.write_params.set_deflection(42.0);
    assert_eq!(_clone.write_params.deflection(), 0.001);
}

#[test]
fn brep_io_write_params_public_field_access() {
    let mut io = BrepIO::new();
    io.write_params.set_triangulate(true);
    io.write_params.set_relative_deflection(true);
    assert!(io.write_params.triangulate());
    assert!(io.write_params.relative_deflection());
}
