//! Compile a DAG several times in one process and report triangle counts and a
//! mesh fingerprint per run (determinism check).
use std::hash::{Hash, Hasher};
fn main() {
    let path = std::env::args().nth(1).expect("dag json path");
    let n: usize = std::env::args()
        .nth(2)
        .map(|s| s.parse().unwrap())
        .unwrap_or(5);
    let raw = std::fs::read_to_string(&path).unwrap();
    for run in 0..n {
        let t0 = std::time::Instant::now();
        let r = ironstream_dag::compile_json(&raw).unwrap();
        let ms = t0.elapsed().as_millis();
        let mut tris = 0usize;
        let mut h = std::collections::hash_map::DefaultHasher::new();
        for p in &r.parts {
            let m = p.solid.mesh();
            tris += m.tris.len();
            for t in &m.tris {
                for &v in t {
                    let q = m.verts[v];
                    (
                        (q.x * 1e6).round() as i64,
                        (q.y * 1e6).round() as i64,
                        (q.z * 1e6).round() as i64,
                    )
                        .hash(&mut h);
                }
            }
        }
        println!(
            "run={run} ms={ms} parts={} tris={tris} fp={:016x}",
            r.parts.len(),
            h.finish()
        );
    }
}
