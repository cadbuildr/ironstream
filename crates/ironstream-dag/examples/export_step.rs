//! Write one analytic STEP file per part of a compiled DAG.
//! usage: export_step <dag.json> <out dir>
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let raw = std::fs::read_to_string(&a[1]).unwrap();
    let r = ironstream_dag::compile_json(&raw).unwrap();
    std::fs::create_dir_all(&a[2]).unwrap();
    for (i, p) in r.parts.iter().enumerate() {
        let step = ironstream_dag::part_step(p);
        let f = format!("{}/{:03}.step", a[2], i);
        std::fs::write(&f, step).unwrap();
        println!("{f} {} vol={:.4}", p.name, p.solid.volume());
    }
}
