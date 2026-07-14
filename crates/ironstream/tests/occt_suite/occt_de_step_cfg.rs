use ironstream::de_step_cfg::*;

#[test]
fn step_config_defaults_ap242_schema_is_active() {
    let c = DeStepConfig::new();
    assert_eq!(c.schema(), StepSchema::Ap242);
    assert!(c.is_active());
    assert!((c.max_tolerance() - 1e-3).abs() < 1e-10);
}

#[test]
fn step_config_set_schema() {
    let mut c = DeStepConfig::new();
    c.set_schema(StepSchema::Ap203);
    assert_eq!(c.schema(), StepSchema::Ap203);
}

#[test]
fn step_config_tolerance_clamped() {
    let mut c = DeStepConfig::new();
    c.set_max_tolerance(-1.0);
    assert!(c.max_tolerance() >= 1e-10);
}

#[test]
fn iges_config_defaults_only_visible_is_active() {
    let c = DeIgesConfig::new();
    assert_eq!(c.read_mode(), IgesReadMode::OnlyVisible);
    assert!(c.is_active());
}

#[test]
fn iges_config_bspline_continuity_clamped_to_2() {
    let mut c = DeIgesConfig::new();
    c.set_bspline_continuity(10);
    assert_eq!(c.read_bspline_continuity, 2);
    c.set_bspline_continuity(1);
    assert_eq!(c.read_bspline_continuity, 1);
}
