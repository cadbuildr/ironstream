use ironstream::ais_manipulator::*;

#[test]
fn manipulator_defaults_mode_none_nb_attached_zero() {
    let m = AisManipulator::new(1);
    assert_eq!(m.mode(), AisManipulatorMode::None);
    assert_eq!(m.nb_attached(), 0);
    assert!(m.is_visible());
}

#[test]
fn manipulator_attach_detach() {
    let mut m = AisManipulator::new(1);
    m.attach(10);
    m.attach(20);
    assert_eq!(m.nb_attached(), 2);
    m.detach();
    assert_eq!(m.nb_attached(), 0);
}

#[test]
fn manipulator_set_mode() {
    let mut m = AisManipulator::new(1);
    m.set_mode(AisManipulatorMode::Translation);
    assert_eq!(m.mode(), AisManipulatorMode::Translation);
    m.set_mode(AisManipulatorMode::Rotation);
    assert_eq!(m.mode(), AisManipulatorMode::Rotation);
}

#[test]
fn manipulator_apply_translation() {
    let mut m = AisManipulator::new(1);
    m.set_position([1.0, 2.0, 3.0]);
    let new_pos = m.apply_translation([0.5, -1.0, 0.0]);
    assert!((new_pos[0] - 1.5).abs() < 1e-10);
    assert!((new_pos[1] - 1.0).abs() < 1e-10);
    assert!((new_pos[2] - 3.0).abs() < 1e-10);
}

#[test]
fn manipulator_axis_flags() {
    let mut m = AisManipulator::new(1);
    m.enable_axis_rotation(1, false);
    assert!(!m.enable_rotation[1]);
    assert!(m.enable_rotation[0]);
    assert!(m.enable_rotation[2]);
    m.set_enable_scaling(true);
    assert!(m.enable_scaling);
}
