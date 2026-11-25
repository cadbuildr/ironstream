//! Per-part mesh health: triangles, edges not shared by exactly two
//! triangles, volume. usage: check_parts <dag.json>
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let r = ironstream_dag::compile_json(&std::fs::read_to_string(&a[1]).unwrap()).unwrap();
    let (mut tris, mut bad_parts, mut bad_edges) = (0usize, 0usize, 0usize);
    for p in &r.parts {
        let m = p.solid.mesh();
        tris += m.tris.len();
        let mut c: std::collections::HashMap<(usize, usize), u32> = Default::default();
        for t in &m.tris {
            for k in 0..3 {
                let (x, y) = (t[k], t[(k + 1) % 3]);
                *c.entry((x.min(y), x.max(y))).or_insert(0) += 1;
            }
        }
        let bad = c.values().filter(|&&n| n != 2).count();
        if bad > 0 {
            bad_parts += 1;
            bad_edges += bad;
            if bad_parts <= 3 {
                eprintln!("  {}: {} bad edges", p.name, bad);
            }
        }
    }
    println!(
        "parts={} tris={} non_manifold_parts={} bad_edges={} errors={}",
        r.parts.len(),
        tris,
        bad_parts,
        bad_edges,
        r.errors.len()
    );
}
