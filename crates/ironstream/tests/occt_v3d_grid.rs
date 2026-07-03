// FILE: rust/ironstream/crates/ironstream/tests/occt_v3d_grid.rs
extern crate ironstream;
use ironstream::v3d_grid::*;

#[test]
fn test_rectangular_grid_type_and_steps() {
    let grid = V3dGrid::rectangular(1.0, 2.0);
    assert_eq!(grid.grid_type(), GridType::Rectangular);
    assert_eq!(grid.x_step(), 1.0);
    assert_eq!(grid.y_step(), 2.0);
}

#[test]
fn test_circular_grid_type_and_steps() {
    let grid = V3dGrid::circular(5.0, 12);
    assert_eq!(grid.grid_type(), GridType::Circular);
    assert_eq!(grid.x_step(), 5.0);
    assert_eq!(grid.y_step(), 12.0);
}

#[test]
fn test_default_draw_mode_is_lines() {
    let rect = V3dGrid::rectangular(1.0, 1.0);
    assert_eq!(rect.draw_mode(), GridDrawMode::Lines);

    let circ = V3dGrid::circular(1.0, 8);
    assert_eq!(circ.draw_mode(), GridDrawMode::Lines);
}

#[test]
fn test_set_draw_mode_roundtrip() {
    let mut grid = V3dGrid::rectangular(1.0, 1.0);
    grid.set_draw_mode(GridDrawMode::Points);
    assert_eq!(grid.draw_mode(), GridDrawMode::Points);
    grid.set_draw_mode(GridDrawMode::Lines);
    assert_eq!(grid.draw_mode(), GridDrawMode::Lines);
}

#[test]
fn test_default_origin_is_zero() {
    let grid = V3dGrid::rectangular(1.0, 1.0);
    assert_eq!(grid.origin(), (0.0, 0.0));
}

#[test]
fn test_set_origin_roundtrip() {
    let mut grid = V3dGrid::rectangular(1.0, 1.0);
    grid.set_origin(3.5, -7.25);
    let (ox, oy) = grid.origin();
    assert_eq!(ox, 3.5);
    assert_eq!(oy, -7.25);
}

#[test]
fn test_default_rotation_angle_is_zero() {
    let grid = V3dGrid::circular(2.0, 6);
    assert_eq!(grid.rotation_angle(), 0.0);
}

#[test]
fn test_set_rotation_angle_roundtrip() {
    let mut grid = V3dGrid::rectangular(1.0, 1.0);
    let pi_over_4 = std::f64::consts::PI / 4.0;
    grid.set_rotation_angle(pi_over_4);
    assert!((grid.rotation_angle() - pi_over_4).abs() < 1e-15);
}

#[test]
fn test_default_is_inactive() {
    let grid = V3dGrid::rectangular(1.0, 1.0);
    assert!(!grid.is_active());
}

#[test]
fn test_activate_and_deactivate() {
    let mut grid = V3dGrid::rectangular(1.0, 1.0);
    grid.activate();
    assert!(grid.is_active());
    grid.deactivate();
    assert!(!grid.is_active());
}

#[test]
fn test_activate_deactivate_toggle() {
    let mut grid = V3dGrid::circular(1.0, 4);
    assert!(!grid.is_active());
    grid.activate();
    assert!(grid.is_active());
    grid.activate();
    assert!(grid.is_active());
    grid.deactivate();
    assert!(!grid.is_active());
    grid.deactivate();
    assert!(!grid.is_active());
}

#[test]
fn test_default_color() {
    let grid = V3dGrid::rectangular(1.0, 1.0);
    let c = grid.color();
    assert!((c[0] - 0.5f32).abs() < 1e-6);
    assert!((c[1] - 0.5f32).abs() < 1e-6);
    assert!((c[2] - 0.5f32).abs() < 1e-6);
}

#[test]
fn test_set_color_roundtrip() {
    let mut grid = V3dGrid::rectangular(1.0, 1.0);
    grid.set_color([0.1, 0.2, 0.9]);
    let c = grid.color();
    assert!((c[0] - 0.1f32).abs() < 1e-6);
    assert!((c[1] - 0.2f32).abs() < 1e-6);
    assert!((c[2] - 0.9f32).abs() < 1e-6);
}

#[test]
fn test_default_elevation_color() {
    let grid = V3dGrid::circular(1.0, 8);
    let ec = grid.elevation_color();
    assert_eq!(ec[0], 0.0f32);
    assert_eq!(ec[1], 0.0f32);
    assert_eq!(ec[2], 1.0f32);
}

#[test]
fn test_set_elevation_color_roundtrip() {
    let mut grid = V3dGrid::circular(2.0, 6);
    grid.set_elevation_color([1.0, 0.5, 0.0]);
    let ec = grid.elevation_color();
    assert!((ec[0] - 1.0f32).abs() < 1e-6);
    assert!((ec[1] - 0.5f32).abs() < 1e-6);
    assert!((ec[2] - 0.0f32).abs() < 1e-6);
}

#[test]
fn test_grid_type_enum_copy_and_clone() {
    let gt = GridType::Rectangular;
    let gt2 = gt;
    assert_eq!(gt, gt2);

    let gd = GridDrawMode::Points;
    let gd2 = gd;
    assert_eq!(gd, gd2);
}

#[test]
fn test_grid_type_variants_are_distinct() {
    assert_ne!(GridType::Rectangular, GridType::Circular);
    assert_ne!(GridDrawMode::Lines, GridDrawMode::Points);
}

#[test]
fn test_circular_nb_divisions_stored_as_y_step() {
    let grid = V3dGrid::circular(3.0, 36);
    assert_eq!(grid.x_step(), 3.0);
    assert_eq!(grid.y_step(), 36.0);
    assert_eq!(grid.grid_type(), GridType::Circular);
}

#[test]
fn test_full_rectangular_grid_workflow() {
    let mut grid = V3dGrid::rectangular(0.5, 0.5);
    grid.set_draw_mode(GridDrawMode::Points);
    grid.set_origin(1.0, 2.0);
    grid.set_rotation_angle(0.1);
    grid.set_color([0.8, 0.8, 0.8]);
    grid.set_elevation_color([0.0, 1.0, 0.0]);
    grid.activate();

    assert_eq!(grid.grid_type(), GridType::Rectangular);
    assert_eq!(grid.draw_mode(), GridDrawMode::Points);
    assert_eq!(grid.x_step(), 0.5);
    assert_eq!(grid.y_step(), 0.5);
    assert_eq!(grid.origin(), (1.0, 2.0));
    assert!((grid.rotation_angle() - 0.1).abs() < 1e-15);
    assert!(grid.is_active());
    let c = grid.color();
    assert!((c[0] - 0.8f32).abs() < 1e-6);
    let ec = grid.elevation_color();
    assert!((ec[1] - 1.0f32).abs() < 1e-6);
}

#[test]
fn test_full_circular_grid_workflow() {
    let mut grid = V3dGrid::circular(10.0, 24);
    grid.set_draw_mode(GridDrawMode::Lines);
    grid.set_origin(-5.0, 5.0);
    grid.set_rotation_angle(std::f64::consts::PI);
    grid.set_color([0.3, 0.3, 0.3]);
    grid.activate();

    assert_eq!(grid.grid_type(), GridType::Circular);
    assert_eq!(grid.draw_mode(), GridDrawMode::Lines);
    assert_eq!(grid.x_step(), 10.0);
    assert_eq!(grid.y_step(), 24.0);
    assert_eq!(grid.origin(), (-5.0, 5.0));
    assert!((grid.rotation_angle() - std::f64::consts::PI).abs() < 1e-14);
    assert!(grid.is_active());
}

#[test]
fn test_angle_default_is_zero() {
    let grid = V3dGrid::rectangular(2.0, 3.0);
    assert_eq!(grid.angle(), 0.0);
}
