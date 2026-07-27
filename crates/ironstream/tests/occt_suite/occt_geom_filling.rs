extern crate ironstream;
use ironstream::geom_filling::*;

#[test]
fn test_geom_filling_basic() {
    // BSplineFilling with 4 boundaries (unit square).
    let mut fill = BSplineFilling::new(FillingStyle::Coons);
    assert!(!fill.is_done()); // No boundaries yet.

    fill.add_boundary(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    fill.add_boundary(vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]]);
    fill.add_boundary(vec![[1.0, 1.0, 0.0], [0.0, 1.0, 0.0]]);
    fill.add_boundary(vec![[0.0, 1.0, 0.0], [0.0, 0.0, 0.0]]);
    assert!(fill.is_done());

    let pts = fill.build();
    assert!(!pts.is_empty());
    // All interior points should lie in the Z=0 plane.
    for p in &pts {
        assert!(p[2].abs() < 1e-10, "z={}", p[2]);
    }

    // Strech style also works.
    let mut fill2 = BSplineFilling::new(FillingStyle::Strech);
    fill2.add_boundary(vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]);
    fill2.add_boundary(vec![[2.0, 0.0, 0.0], [2.0, 2.0, 0.0]]);
    fill2.add_boundary(vec![[2.0, 2.0, 0.0], [0.0, 2.0, 0.0]]);
    fill2.add_boundary(vec![[0.0, 2.0, 0.0], [0.0, 0.0, 0.0]]);
    assert!(fill2.is_done());
    let pts2 = fill2.build();
    assert!(!pts2.is_empty());

    // CoonsPatch with a flat unit-square boundary.
    let mut patch = CoonsPatch::new();
    patch.set_side(0, vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]); // bottom
    patch.set_side(1, vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]]); // right
    patch.set_side(2, vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]); // top
    patch.set_side(3, vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0]]); // left
    let center = patch.evaluate(0.5, 0.5);
    // For the unit square the centre should be at approximately [0.5, 0.5, 0.0].
    assert!((center[0] - 0.5).abs() < 0.15, "cx={}", center[0]);
    assert!((center[1] - 0.5).abs() < 0.15, "cy={}", center[1]);
}
