// FILE: tests/occt_aspect_color.rs
extern crate ironstream;
use ironstream::aspect_color::*;

#[test]
fn ramp_defaults() {
    let c = AspectColorRampColorMap::new();
    assert_eq!(c.nb_entries(), 2);
    // t=0 → blue (B channel ≈ 1.0)
    let s = c.sample(0.0);
    assert!((s[2] - 1.0).abs() < 1e-10);
}

#[test]
fn ramp_add_sorted() {
    let mut c = AspectColorRampColorMap::new();
    c.add(AspectColorRampEntry::new(0.5, [0.0, 1.0, 0.0, 1.0]));
    assert_eq!(c.nb_entries(), 3);
    // At t=0.5 exactly, sample returns that entry's green
    let s = c.sample(0.5);
    assert!((s[1] - 1.0).abs() < 1e-10);
}

#[test]
fn ramp_clamp_below() {
    let c = AspectColorRampColorMap::default();
    // t=-5 → first entry (blue)
    let below = c.sample(-5.0);
    assert!((below[2] - 1.0).abs() < 1e-10);
}

#[test]
fn ramp_clamp_above() {
    let c = AspectColorRampColorMap::default();
    // t=10 → last entry (red)
    let above = c.sample(10.0);
    assert!((above[0] - 1.0).abs() < 1e-10);
}

#[test]
fn mark_aspect_defaults() {
    let m = AspectMarkAspect::new();
    assert_eq!(m.marker_type, AspectTypeOfMarker::Point);
    assert!((m.scale - 1.0).abs() < 1e-6);
    assert!(!m.is_custom());
}

#[test]
fn hatch_interior_defaults() {
    assert_eq!(AspectHatchStyle::default(), AspectHatchStyle::Solid);
    assert_eq!(AspectInteriorStyle::default(), AspectInteriorStyle::Empty);
    assert_eq!(AspectFillMethod::default(), AspectFillMethod::None);
}
