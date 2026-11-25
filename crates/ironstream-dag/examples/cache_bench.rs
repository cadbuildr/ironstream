//! Cold / warm / edit timings with a persistent `Compiler`.
//! usage: cache_bench <dag.json> [<param-node-id>=<value>]
fn main() {
    let path = std::env::args().nth(1).expect("dag json path");
    let edit = std::env::args().nth(2);
    let raw = std::fs::read_to_string(&path).unwrap();
    let input = ironstream_dag::dag::CompilerInputDag::from_json(&raw).unwrap();
    let mut c = ironstream_dag::Compiler::new(Default::default());
    let mut run = |label: &str, input: &ironstream_dag::dag::CompilerInputDag| {
        let t0 = std::time::Instant::now();
        let r = c.compile(input);
        let ms = t0.elapsed().as_secs_f64() * 1e3;
        let tris: usize = r.parts.iter().map(|p| p.solid.mesh().tris.len()).sum();
        let vol: f64 = r.parts.iter().map(|p| p.solid.volume()).sum();
        let uniq: std::collections::HashSet<String> = r
            .parts
            .iter()
            .filter_map(|p| p.instance.as_ref().map(|i| i.key.clone()))
            .collect();
        println!(
            "{label:5} ms={ms:8.1} parts={} tris={tris} unique_keys={} vol={vol:.3} errors={} {:?}",
            r.parts.len(),
            uniq.len(),
            r.errors.len(),
            r.stats
        );
    };
    run("cold", &input);
    run("warm", &input);
    if let Some(e) = edit {
        let (id, v) = e.split_once('=').unwrap();
        let mut input2 = input.clone();
        let n = input2.dag.get_mut(id).expect("edit node");
        n.params["value"] = serde_json::json!(v.parse::<f64>().unwrap());
        run("edit", &input2);
    }
}
