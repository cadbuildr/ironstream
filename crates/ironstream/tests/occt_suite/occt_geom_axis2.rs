extern crate ironstream;
use ironstream::geom_axis2::*;

#[test]
fn test_geom_axis2_basic() {
    let ax2 = Ax2::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    assert!(ax2.is_right_handed());

    let y = ax2.y_direction();
    // y should be cross(z, x) = [0,1,0]
    assert!((y[0] - 0.0).abs() < 1e-10);
    assert!((y[1] - 1.0).abs() < 1e-10);
    assert!((y[2] - 0.0).abs() < 1e-10);
}

#[test]
fn test_geom_axis2_default() {
    let ax2 = Ax2::default();
    // default: origin=[0,0,0], direction=[0,0,1], x=[1,0,0]
    assert!((ax2.direction[2] - 1.0).abs() < 1e-10);
    assert!(ax2.is_right_handed());
}

#[test]
fn test_geom_axis3_basic() {
    let ax3 = Ax3::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    assert!(ax3.is_right_handed());
    assert!(ax3.direct());

    let ax3b = Ax3::from_ax2(Ax2::default());
    assert!(ax3b.is_right_handed());
}

#[test]
fn test_cross3_and_normalize3() {
    let a = [1.0f64, 0.0, 0.0];
    let b = [0.0f64, 1.0, 0.0];
    let c = cross3(a, b);
    assert!((c[0] - 0.0).abs() < 1e-10);
    assert!((c[1] - 0.0).abs() < 1e-10);
    assert!((c[2] - 1.0).abs() < 1e-10);

    let n = normalize3([3.0, 4.0, 0.0]);
    assert!((n[0] - 0.6).abs() < 1e-10);
    assert!((n[1] - 0.8).abs() < 1e-10);
}
