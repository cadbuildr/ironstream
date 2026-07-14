// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_int.rs
extern crate ironstream;

use ironstream::geom_int::*;

#[test]
fn intss_new_not_done() {
    let intss = GeomIntIntSS::new(1e-7);
    assert!(!intss.is_done());
}

#[test]
fn intss_perform_sets_done() {
    let mut intss = GeomIntIntSS::new(1e-7);
    intss.perform();
    assert!(intss.is_done());
}

#[test]
fn intss_perform_produces_no_lines() {
    let mut intss = GeomIntIntSS::new(1e-7);
    intss.perform();
    assert_eq!(intss.nb_lines(), 0);
}

#[test]
fn intss_perform_produces_no_points() {
    let mut intss = GeomIntIntSS::new(1e-7);
    intss.perform();
    assert_eq!(intss.nb_points(), 0);
}

#[test]
fn intss_add_line_increases_count() {
    let mut intss = GeomIntIntSS::new(1e-7);
    intss.perform();
    let line = GeomIntIntSSLine::new(GeomIntLineType::Walking);
    intss.add_line(line);
    assert_eq!(intss.nb_lines(), 1);
}

#[test]
fn intss_add_multiple_lines() {
    let mut intss = GeomIntIntSS::new(1e-6);
    intss.add_line(GeomIntIntSSLine::new(GeomIntLineType::Walking));
    intss.add_line(GeomIntIntSSLine::new(GeomIntLineType::Restriction));
    intss.add_line(GeomIntIntSSLine::new(GeomIntLineType::Analytic));
    assert_eq!(intss.nb_lines(), 3);
}

#[test]
fn intss_line_accessor_one_based() {
    let mut intss = GeomIntIntSS::new(1e-7);
    intss.add_line(GeomIntIntSSLine::new(GeomIntLineType::Analytic));
    assert!(intss.line(1).is_some());
    assert!(intss.line(0).is_none());
    assert!(intss.line(2).is_none());
}

#[test]
fn intss_line_type_preserved() {
    let mut intss = GeomIntIntSS::new(1e-7);
    intss.add_line(GeomIntIntSSLine::new(GeomIntLineType::Restriction));
    assert_eq!(intss.line(1).unwrap().line_type, GeomIntLineType::Restriction);
}

#[test]
fn intss_point_accessor_zero_returns_none() {
    let intss = GeomIntIntSS::new(1e-7);
    assert!(intss.point(0).is_none());
}

#[test]
fn intss_line_new_empty() {
    let line = GeomIntIntSSLine::new(GeomIntLineType::Walking);
    assert_eq!(line.nb_points(), 0);
    assert!(!line.has_first_point);
    assert!(!line.has_last_point);
}

#[test]
fn intss_line_add_point_increases_count() {
    let mut line = GeomIntIntSSLine::new(GeomIntLineType::Walking);
    let p = GeomIntIntSSPoint::new(0.1, 0.2, 0.3, 0.4, [1.0, 2.0, 3.0]);
    line.add_point(p);
    assert_eq!(line.nb_points(), 1);
}

#[test]
fn intss_line_point_accessor_one_based() {
    let mut line = GeomIntIntSSLine::new(GeomIntLineType::Walking);
    line.add_point(GeomIntIntSSPoint::new(0.0, 0.0, 0.0, 0.0, [0.0, 0.0, 0.0]));
    line.add_point(GeomIntIntSSPoint::new(1.0, 1.0, 1.0, 1.0, [1.0, 1.0, 1.0]));
    assert!(line.point(1).is_some());
    assert!(line.point(2).is_some());
    assert!(line.point(0).is_none());
    assert!(line.point(3).is_none());
}

#[test]
fn intss_line_point_values_correct() {
    let mut line = GeomIntIntSSLine::new(GeomIntLineType::Analytic);
    let p = GeomIntIntSSPoint::new(0.25, 0.5, 0.75, 1.0, [10.0, 20.0, 30.0]);
    line.add_point(p);
    let retrieved = line.point(1).unwrap();
    assert_eq!(retrieved.u1, 0.25);
    assert_eq!(retrieved.v1, 0.5);
    assert_eq!(retrieved.u2, 0.75);
    assert_eq!(retrieved.v2, 1.0);
    assert_eq!(retrieved.xyz, [10.0, 20.0, 30.0]);
}

#[test]
fn intss_point_new_fields() {
    let p = GeomIntIntSSPoint::new(0.1, 0.2, 0.3, 0.4, [5.0, 6.0, 7.0]);
    assert_eq!(p.u1, 0.1);
    assert_eq!(p.v1, 0.2);
    assert_eq!(p.u2, 0.3);
    assert_eq!(p.v2, 0.4);
    assert_eq!(p.xyz[0], 5.0);
    assert_eq!(p.xyz[1], 6.0);
    assert_eq!(p.xyz[2], 7.0);
}

// --- GeomIntSsResult ---

#[test]
fn ss_result_new_not_done() {
    let r = GeomIntSsResult::new();
    assert!(!r.is_done());
}

#[test]
fn ss_result_new_nb_lines_zero() {
    let r = GeomIntSsResult::new();
    assert_eq!(r.nb_lines(), 0);
}

#[test]
fn ss_result_mark_done_sets_done() {
    let mut r = GeomIntSsResult::new();
    r.mark_done();
    assert!(r.is_done());
}

#[test]
fn ss_result_mark_done_does_not_change_nb_lines() {
    let mut r = GeomIntSsResult::new();
    r.mark_done();
    assert_eq!(r.nb_lines(), 0);
}

#[test]
fn ss_result_default_equals_new() {
    let r: GeomIntSsResult = Default::default();
    assert!(!r.is_done());
    assert_eq!(r.nb_lines(), 0);
}

// --- GeomIntIntSs (stub) ---

#[test]
fn int_ss_stub_new_not_done() {
    let q = GeomIntIntSs::new();
    assert!(!q.is_done());
}

#[test]
fn int_ss_stub_new_nb_lines_zero() {
    let q = GeomIntIntSs::new();
    assert_eq!(q.nb_lines(), 0);
}

#[test]
fn int_ss_stub_perform_sets_done() {
    let mut q = GeomIntIntSs::new();
    q.perform();
    assert!(q.is_done());
}

#[test]
fn int_ss_stub_perform_nb_lines_stays_zero() {
    let mut q = GeomIntIntSs::new();
    q.perform();
    assert_eq!(q.nb_lines(), 0);
}

#[test]
fn int_ss_stub_fields_accessible() {
    let mut q = GeomIntIntSs::new();
    q.perform();
    assert!(q.is_done);
    assert_eq!(q.nb_lines, 0);
}

#[test]
fn int_ss_stub_default_equals_new() {
    let q: GeomIntIntSs = Default::default();
    assert!(!q.is_done());
    assert_eq!(q.nb_lines(), 0);
}

// --- GeomIntLineTool ---

#[test]
fn line_tool_nb_vertex_returns_input() {
    assert_eq!(GeomIntLineTool::nb_vertex(0), 0);
    assert_eq!(GeomIntLineTool::nb_vertex(5), 5);
    assert_eq!(GeomIntLineTool::nb_vertex(100), 100);
}

#[test]
fn line_tool_first_vertex_returns_zero() {
    assert_eq!(GeomIntLineTool::first_vertex(), 0);
}

#[test]
fn line_tool_last_vertex_returns_zero() {
    assert_eq!(GeomIntLineTool::last_vertex(), 0);
}
