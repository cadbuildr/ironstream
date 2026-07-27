extern crate ironstream;
use ironstream::brep_mesh_discrete::*;

#[test]
fn test_brep_mesh_discrete_basic() {
    // MeshAlgoParams construction and defaults.
    let p = MeshAlgoParams::default();
    assert!((p.deflection - 0.1).abs() < 1e-12);
    assert!(!p.in_parallel);

    // Clamping: deflection below 1e-7 is clamped up.
    let p2 = MeshAlgoParams::new(0.0, 0.5, 1e-7, false);
    assert!(p2.deflection >= 1e-7);

    // Builder methods.
    let p3 = MeshAlgoParams::default().with_deflection(0.05).with_angle(0.3);
    assert!((p3.deflection - 0.05).abs() < 1e-12);
    assert!((p3.angle - 0.3).abs() < 1e-12);

    // DiscretResult.
    let r = DiscretResult::new(100, 200);
    assert!(r.is_done());
    assert_eq!(r.node_count, 100);
    assert_eq!(r.triangle_count, 200);

    // MeshDiscret: non-empty shape produces a Done result.
    let params = MeshAlgoParams::default();
    let mut md = MeshDiscret::new("box", params);
    let result = md.perform();
    assert!(result.is_done());
    assert!(result.node_count > 0);

    // set_parallel changes the flag.
    md.set_parallel(true);
    assert!(md.params.in_parallel);
}
