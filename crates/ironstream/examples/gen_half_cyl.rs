fn main() {
    use ironstream::prelude::*;
    let cyl = make_cylinder(10.0, 20.0, MeshParams::default());
    let block = make_box(Pnt::new(-11.0, -11.0, -1.0), 11.0, 22.0, 22.0);
    let half = cut(&cyl, &block);
    std::fs::write("/tmp/half_cylinder_analytic.step", write_step_analytic(&half, "half_cylinder")).unwrap();
    std::fs::write("/tmp/half_cylinder_faceted.step", write_step(half.mesh(), "half_cylinder")).unwrap();
    println!("volume={:.2}", half.volume());
}
