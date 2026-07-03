// FILE: tests/occt_graphic3d_clip.rs
extern crate ironstream;
use ironstream::graphic3d_clip::*;

#[test]
fn test_clip_plane_from_point_normal() {
    let p = ClipPlane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    assert!((p.distance_to_point(&[0.0, 0.0, 1.0]) - 1.0).abs() < 1e-12);
    assert!((p.distance_to_point(&[0.0, 0.0, -1.0]) + 1.0).abs() < 1e-12);
    assert!(!p.is_point_clipped(&[0.0, 0.0, 0.5]));
    assert!(p.is_point_clipped(&[0.0, 0.0, -0.1]));
}

#[test]
fn test_clip_plane_flip() {
    let p = ClipPlane::new(0.0, 0.0, 1.0, 0.0);
    let flipped = p.flip_copy();
    assert!((flipped.equation[2] + 1.0).abs() < 1e-12);
}

#[test]
fn test_clip_plane_set_add_and_query() {
    let mut cs = ClipPlaneSet::new();
    cs.add(ClipPlane::new(0.0, 0.0, 1.0, 0.0));
    cs.add(ClipPlane::new(0.0, 1.0, 0.0, 0.0));
    assert_eq!(cs.nb_planes(), 2);
    assert_eq!(cs.nb_active(), 2);
    assert!(cs.is_point_clipped([0.0, 0.0, -1.0]));
    assert!(!cs.is_point_clipped([1.0, 1.0, 1.0]));
}

#[test]
fn test_clip_plane_set_disable_all() {
    let mut cs = ClipPlaneSet::new();
    cs.add(ClipPlane::new(0.0, 0.0, 1.0, 0.0));
    cs.set_all_on(false);
    assert_eq!(cs.nb_active(), 0);
    assert!(!cs.is_point_clipped([0.0, 0.0, -1.0]));
}

#[test]
fn test_clip_box_contains_and_planes() {
    let cb = ClipBox::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    assert!(cb.contains(&[0.5, 0.5, 0.5]));
    assert!(!cb.contains(&[1.5, 0.5, 0.5]));
    let planes = cb.to_clip_planes();
    assert_eq!(planes.len(), 6);
}
