//! Write every part of a compiled DAG into one ASCII-free binary STL.
//! usage: dump_stl <dag.json> <out.stl>
use std::io::Write;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let raw = std::fs::read_to_string(&a[1]).unwrap();
    let r = ironstream_dag::compile_json(&raw).unwrap();
    let mut tris: Vec<[[f32; 3]; 3]> = Vec::new();
    for p in &r.parts {
        let m = p.solid.mesh();
        for t in &m.tris {
            tris.push(t.map(|i| {
                let v = m.verts[i];
                [v.x as f32, v.y as f32, v.z as f32]
            }));
        }
    }
    let mut f = std::fs::File::create(&a[2]).unwrap();
    f.write_all(&[0u8; 80]).unwrap();
    f.write_all(&(tris.len() as u32).to_le_bytes()).unwrap();
    for t in tris {
        f.write_all(&[0u8; 12]).unwrap();
        for v in t {
            for c in v {
                f.write_all(&c.to_le_bytes()).unwrap();
            }
        }
        f.write_all(&[0u8; 2]).unwrap();
    }
}
