fn main() {
    let path = std::env::args().nth(1).expect("dag json path");
    let raw = std::fs::read_to_string(&path).unwrap();
    let t0 = std::time::Instant::now();
    let r = ironstream_dag::compile_json(&raw).unwrap();
    let t1 = t0.elapsed();
    let mut tris = 0usize;
    for p in &r.parts { tris += p.solid.mesh().tris.len(); }
    println!("compile_ms={} mesh_total_ms={} parts={} tris={} unsupported={:?} errors={:?}",
        t1.as_millis(), t0.elapsed().as_millis(), r.parts.len(), tris, r.unsupported, r.errors.iter().take(3).collect::<Vec<_>>());
}
