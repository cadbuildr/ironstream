//! OCCT-parity + memory-ceiling regressions on real foundation DAGs.
//!
//! Fixtures are cadbuildr-stdlib catalog parts (DAGs serialized by
//! foundation 0.2.20) that the kernel_compare benchmark found broken: the
//! GT2 pulley ran the wasm build out of a 2.5 GB heap, the threaded parts had
//! no geometry ("path has < 2 points", Helix3D was a no-op). Each must now
//! match the OCCT kernel's volume within 1 % and stay under a memory ceiling.
//! The reference volumes are OCCT (kernel-api / replicad) mesh volumes.

use std::path::Path;

/// (fixture, OCCT volume mm^3, max triangles)
const CASES: &[(&str, f64, usize)] = &[
    ("gt2-pulley.json", 992.353, 40_000),
    ("lead-screw-nut-t8.json", 1406.217, 60_000),
    ("din912-socket-cap-screw.json", 2512.474, 250_000),
    ("threaded-rod-metric.json", 5479.605, 600_000),
];

/// Peak resident set of this process (Linux `VmHWM`), in MB.
fn peak_rss_mb() -> Option<f64> {
    let s = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = s.lines().find(|l| l.starts_with("VmHWM:"))?;
    let kb: f64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kb / 1024.0)
}

#[test]
fn stdlib_corpus_matches_occt_under_memory_ceiling() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/corpus");
    for &(name, occt_vol, max_tris) in CASES {
        let raw = std::fs::read_to_string(dir.join(name)).unwrap();
        let r = ironstream_dag::compile_json(&raw).unwrap();
        assert!(r.errors.is_empty(), "{name}: {:?}", r.errors);
        let vol: f64 = r.parts.iter().map(|p| p.solid.volume()).sum();
        let tris: usize = r.parts.iter().map(|p| p.solid.mesh().tris.len()).sum();
        assert!(
            (vol / occt_vol - 1.0).abs() < 0.01,
            "{name}: volume {vol:.3} vs OCCT {occt_vol:.3}"
        );
        assert!(tris <= max_tris, "{name}: {tris} triangles > {max_tris}");
    }
    // One process for all cases: the high-water mark covers the worst one.
    // The wasm build OOM'd above 2.5 GB on the pulley before the BSP rework.
    if let Some(mb) = peak_rss_mb() {
        assert!(mb < 600.0, "peak RSS {mb:.0} MB over the 600 MB ceiling");
    }
}
