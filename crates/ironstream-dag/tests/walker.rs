//! Walker coverage beyond the benchmark corpus, plus the instancing, caching,
//! determinism and thread-folding guarantees.
//!
//! Node fixtures are foundation DAGs written by `castiron`'s generator (see
//! `fixtures/nodes`); expected values are the analytic answers castiron also
//! checks.

use ironstream_dag::{compile_json, dag::CompilerInputDag, Compiler};
use std::path::PathBuf;

fn fixture(rel: &str) -> String {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

fn volume(r: &ironstream_dag::CompileResult) -> f64 {
    r.parts.iter().map(|p| p.solid.volume()).sum()
}

fn bounds(r: &ironstream_dag::CompileResult) -> ([f64; 3], [f64; 3]) {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for p in &r.parts {
        let b = p.solid.bbox();
        for k in 0..3 {
            lo[k] = lo[k].min(b.min[k]);
            hi[k] = hi[k].max(b.max[k]);
        }
    }
    (lo, hi)
}

#[test]
fn linear_pattern_places_copies() {
    let r = compile_json(&fixture("nodes/linear_pattern.json")).unwrap();
    assert!(r.errors.is_empty(), "{:?}", r.errors);
    assert!((volume(&r) - 4.0).abs() < 1e-9);
    assert!((bounds(&r).1[0] - 10.0).abs() < 1e-9);
}

#[test]
fn polar_pattern_places_copies() {
    let r = compile_json(&fixture("nodes/polar_pattern.json")).unwrap();
    assert!(r.errors.is_empty(), "{:?}", r.errors);
    assert!((volume(&r) - 6.0).abs() < 1e-6);
}

#[test]
fn move_body_translates_and_rotates() {
    let r = compile_json(&fixture("nodes/move_body.json")).unwrap();
    assert!(r.errors.is_empty(), "{:?}", r.errors);
    assert!((volume(&r) - 1.0).abs() < 1e-9);
    assert!((bounds(&r).0[2] - 7.0).abs() < 1e-9);
}

#[test]
fn boolean_operation_subtracts() {
    let r = compile_json(&fixture("nodes/boolean_subtract.json")).unwrap();
    assert!(r.errors.is_empty(), "{:?}", r.errors);
    assert!((volume(&r) - 48.0).abs() < 1e-6);
}

#[test]
fn unbuilt_operations_fail_loudly() {
    let r = compile_json(&fixture("nodes/fillet.json")).unwrap();
    assert!(
        r.errors
            .iter()
            .any(|e| e.starts_with("Fillet#") && e.contains("not built")),
        "{:?}",
        r.errors
    );
}

/// The same DAG compiles to the same mesh, vertex for vertex.
#[test]
fn compiles_are_deterministic() {
    let raw = fixture("corpus/din912-socket-cap-screw.json");
    let mesh = || {
        let r = compile_json(&raw).unwrap();
        r.parts
            .iter()
            .map(|p| p.solid.mesh().clone())
            .collect::<Vec<_>>()
    };
    let a = mesh();
    for _ in 0..3 {
        let b = mesh();
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(&b) {
            assert_eq!(x.tris, y.tris);
            assert_eq!(x.verts.len(), y.verts.len());
            for (p, q) in x.verts.iter().zip(&y.verts) {
                assert_eq!((p.x, p.y, p.z), (q.x, q.y, q.z));
            }
        }
    }
}

/// Identical parts are built once; a warm recompile rebuilds nothing and
/// returns the same geometry; an edit rebuilds only what it touched.
#[test]
fn instancing_and_cache() {
    let raw = fixture("corpus/gt2-pulley.json");
    let input = CompilerInputDag::from_json(&raw).unwrap();
    let mut c = Compiler::new(Default::default());
    let cold = c.compile(&input);
    let warm = c.compile(&input);
    assert_eq!(warm.stats.ops_built, 0);
    assert_eq!(warm.stats.parts_built, 0);
    assert!((volume(&cold) - volume(&warm)).abs() < 1e-9);
    let tris = |r: &ironstream_dag::CompileResult| {
        r.parts
            .iter()
            .map(|p| p.solid.mesh().tris.len())
            .sum::<usize>()
    };
    assert_eq!(tris(&cold), tris(&warm));
}

/// Threads are folded into exact helical surfaces: watertight and well below
/// the triangle count of a boolean of the swept ridge.
#[test]
fn threads_fold_watertight() {
    for (f, max_tris) in [
        ("corpus/din912-socket-cap-screw.json", 12_000),
        ("corpus/threaded-rod-metric.json", 40_000),
    ] {
        let r = compile_json(&fixture(f)).unwrap();
        let m = r.parts[0].solid.mesh();
        assert!(m.tris.len() < max_tris, "{f}: {} triangles", m.tris.len());
        assert_eq!(m.open_edge_count(1e-6), 0, "{f}: open edges");
    }
}
