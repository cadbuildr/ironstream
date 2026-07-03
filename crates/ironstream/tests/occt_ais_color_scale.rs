use ironstream::ais_color_scale::*;

#[test]
fn defaults_min_max_intervals_visible() {
    let cs = AisColorScale::new(1);
    assert!((cs.min_value() - 0.0).abs() < 1e-10);
    assert!((cs.max_value() - 1.0).abs() < 1e-10);
    assert_eq!(cs.nb_intervals(), 20);
    assert!(cs.is_visible());
}

#[test]
fn set_range_updates_bounds() {
    let mut cs = AisColorScale::new(1);
    cs.set_range(-10.0, 50.0);
    assert!((cs.min_value() + 10.0).abs() < 1e-10);
    assert!((cs.max_value() - 50.0).abs() < 1e-10);
}

#[test]
fn set_range_invalid_no_op() {
    let mut cs = AisColorScale::new(2);
    cs.set_range(5.0, 1.0); // max < min — should be ignored
    assert!((cs.min_value() - 0.0).abs() < 1e-10);
    assert!((cs.max_value() - 1.0).abs() < 1e-10);
}

#[test]
fn color_at_value_extremes_valid_rgb() {
    let cs = AisColorScale::new(1);
    let c_min = cs.color_at_value(0.0);
    let c_max = cs.color_at_value(1.0);
    for k in 0..3 {
        assert!(c_min[k] >= 0.0 && c_min[k] <= 1.0,
            "c_min[{}] = {} out of [0,1]", k, c_min[k]);
        assert!(c_max[k] >= 0.0 && c_max[k] <= 1.0,
            "c_max[{}] = {} out of [0,1]", k, c_max[k]);
    }
    // min and max should produce different colours on the default rainbow
    assert_ne!(c_min, c_max);
}

#[test]
fn custom_colors_returned_at_value() {
    let mut cs = AisColorScale::new(1);
    cs.add_custom_color([1.0, 0.0, 0.0]);
    cs.add_custom_color([0.0, 0.0, 1.0]);
    assert!(cs.has_custom_colors());
    let c_start = cs.color_at_value(0.0);
    assert!((c_start[0] - 1.0).abs() < 1e-10, "should return first custom colour at t=0");
}

#[test]
fn nb_intervals_clamped() {
    let mut cs = AisColorScale::new(1);
    cs.set_nb_intervals(0);
    assert_eq!(cs.nb_intervals(), 1, "0 should clamp to 1");
    cs.set_nb_intervals(9999);
    assert_eq!(cs.nb_intervals(), 1000, "9999 should clamp to 1000");
}
