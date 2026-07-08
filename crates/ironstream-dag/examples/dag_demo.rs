//! Demo: evaluate vendored CADbuildr DAG fixtures and write STLs.
//! `cargo run -p ironstream-dag --example dag_demo -- <out_dir>`
use ironstream::prelude::*;
use ironstream_dag::{compile, dag::CompilerInputDag};

fn main() {
    let out = std::env::args().nth(1).expect("out dir");
    let fixtures = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures");
    for name in ["chess_pawn", "gold_star", "donut", "assy_tree"] {
        let raw = std::fs::read_to_string(format!("{fixtures}/{name}.json")).unwrap();
        let input = CompilerInputDag::from_json(&raw).unwrap();
        let result = compile(&input, MeshParams::default());
        let mut mesh = TriMesh::new();
        for p in &result.parts {
            let m = p.solid.mesh();
            let base = mesh.verts.len();
            mesh.verts.extend_from_slice(&m.verts);
            mesh.tris
                .extend(m.tris.iter().map(|t| [t[0] + base, t[1] + base, t[2] + base]));
        }
        std::fs::write(
            format!("{out}/{name}.stl"),
            ironstream::mesh_io::write_binary_stl(&mesh),
        )
        .unwrap();
        let vol: f64 = result.parts.iter().map(|p| p.solid.volume()).sum();
        println!("{name}: parts={} vol={vol:.2}", result.parts.len());
    }
}
