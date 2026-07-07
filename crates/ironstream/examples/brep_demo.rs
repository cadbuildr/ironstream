//! Throwaway demo: tessellate Stage-2 exact-BOP results to STL for rendering.
use ironstream::brep::{box_brep, cylinder_brep, intersect_convex, TessParams};
use ironstream::gp::Pnt;
use ironstream::mesh_io::write_binary_stl;

fn main() {
    let out = std::env::args().nth(1).expect("out dir");
    let tp = TessParams::default();

    // 1. angled clip: unit cube cut by x + z = 1
    let wedge = box_brep(Pnt::origin(), 1.0, 1.0, 1.0)
        .clip_by_plane(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 1.0));
    std::fs::write(format!("{out}/wedge.stl"), write_binary_stl(&wedge.tessellate(&tp))).unwrap();
    println!("wedge vol = {}", wedge.volume());

    // 2. box ∩ box
    let a = box_brep(Pnt::origin(), 10.0, 10.0, 10.0);
    let b = box_brep(Pnt::new(5.0, 5.0, 5.0), 10.0, 10.0, 10.0);
    let x = intersect_convex(&a, &b);
    std::fs::write(format!("{out}/boxcap.stl"), write_binary_stl(&x.tessellate(&tp))).unwrap();
    println!("box∩box vol = {}", x.volume());

    // 3. box clipped by two angled planes (composed clips → prism)
    let chamfered = box_brep(Pnt::origin(), 10.0, 10.0, 10.0)
        .clip_by_plane(Pnt::new(10.0, 0.0, 6.0), Pnt::new(1.0, 0.0, 1.0))
        .clip_by_plane(Pnt::new(0.0, 10.0, 6.0), Pnt::new(-1.0, 0.0, 1.0));
    std::fs::write(format!("{out}/chamfer.stl"), write_binary_stl(&chamfered.tessellate(&tp))).unwrap();
    println!("double-clip vol = {}", chamfered.volume());

    // 4. cylinder B-rep tessellation
    let c = cylinder_brep(5.0, 12.0);
    std::fs::write(format!("{out}/cyl.stl"), write_binary_stl(&c.tessellate(&tp))).unwrap();
    println!("cylinder vol = {}", c.volume());
}
