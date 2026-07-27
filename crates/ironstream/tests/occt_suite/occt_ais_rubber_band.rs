use ironstream::ais_rubber_band::*;

#[test]
fn rubber_band_default_rectangle_not_visible() {
    let r = AisRubberBand::new();
    assert_eq!(r.band_type(), RubberBandType::Rectangle);
    assert!(!r.is_visible());
    assert_eq!(r.nb_points(), 0);
}

#[test]
fn rubber_band_new_rectangle_bounding_rect() {
    let r = AisRubberBand::new_rectangle(10.0, 20.0, 100.0, 80.0);
    assert_eq!(r.nb_points(), 2);
    assert_eq!(r.band_type(), RubberBandType::Rectangle);
    let bb = r.bounding_rect().unwrap();
    assert!((bb[0] - 10.0).abs() < 1e-10);
    assert!((bb[1] - 20.0).abs() < 1e-10);
    assert!((bb[2] - 100.0).abs() < 1e-10);
    assert!((bb[3] - 80.0).abs() < 1e-10);
}

#[test]
fn rubber_band_add_point_becomes_polygon() {
    let mut r = AisRubberBand::new();
    r.add_point(0.0, 0.0);
    r.add_point(10.0, 0.0);
    r.add_point(5.0, 8.0);
    assert_eq!(r.band_type(), RubberBandType::Polygon);
    assert_eq!(r.nb_points(), 3);
}

#[test]
fn rubber_band_set_visible_toggle() {
    let mut r = AisRubberBand::new();
    assert!(!r.is_visible());
    r.set_visible(true);
    assert!(r.is_visible());
    r.set_visible(false);
    assert!(!r.is_visible());
}

#[test]
fn view_cube_is_visible_and_size() {
    let mut vc = AisViewCube::new(1);
    assert!(vc.is_visible());
    assert!((vc.size() - 70.0).abs() < 1e-10);
    vc.set_visible(false);
    assert!(!vc.is_visible());
    vc.set_size(120.0);
    assert!((vc.size() - 120.0).abs() < 1e-10);
}
