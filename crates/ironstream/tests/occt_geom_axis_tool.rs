extern crate ironstream;
use ironstream::geom_axis_tool::*;

#[test]
fn test_ax1_project_and_distance() {
    let ax = Ax1::x_axis();
    assert!((ax.project([5.0, 0.0, 0.0]) - 5.0).abs() < 1e-10);
    // Point at (0, 3, 4) is distance 5 from x-axis
    assert!((ax.distance_to([0.0, 3.0, 4.0]) - 5.0).abs() < 1e-10);
}

#[test]
fn test_ax1_point_at() {
    let ax = Ax1::z_axis();
    let p = ax.point_at(7.0);
    assert!(p[0].abs() < 1e-10);
    assert!(p[1].abs() < 1e-10);
    assert!((p[2] - 7.0).abs() < 1e-10);
}

#[test]
fn test_ax1_parallel() {
    let a = Ax1::y_axis();
    let b = Ax1::new([5.0, 0.0, 0.0], [0.0, 3.0, 0.0]);
    assert!(a.is_parallel(&b, 1e-10));
    let c = Ax1::x_axis();
    assert!(!a.is_parallel(&c, 1e-10));
}

#[test]
fn test_ax2_to_world_to_local_round_trip() {
    let ax = Ax2::default();
    let p = [2.0, 3.0, 4.0];
    let local = ax.to_local(p);
    let back = ax.to_world(local);
    assert!((back[0] - p[0]).abs() < 1e-10);
    assert!((back[1] - p[1]).abs() < 1e-10);
    assert!((back[2] - p[2]).abs() < 1e-10);
}

#[test]
fn test_axis_placement_transform() {
    let ax2 = Ax2::default();
    let pl = AxisPlacement::new(ax2).with_scale(2.0);
    let p = [1.0, 0.0, 0.0];
    let t = pl.transform(p);
    assert!((t[0] - 2.0).abs() < 1e-10);
}
