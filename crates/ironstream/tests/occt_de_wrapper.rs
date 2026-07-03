extern crate ironstream;
use ironstream::de_wrapper::*;

#[test]
fn test_shape_fix_params_default_healing_enables_all() {
    let p = ShapeFixParameters::default_healing();
    assert!(p.any_enabled());
    assert!(p.fix_shape);
    assert!(p.fix_face);
}

#[test]
fn test_shape_fix_params_none_disables_all() {
    let p = ShapeFixParameters::none();
    assert!(!p.any_enabled());
}

#[test]
fn test_configuration_node_parameter_roundtrip() {
    let mut node = ConfigurationNode::new("STEP", "OCC");
    node.set_parameter("read.iges.bsplines.surfaceMode", "1");
    assert_eq!(node.get_parameter("read.iges.bsplines.surfaceMode"), Some("1"));
    assert!(node.is_supported_extension("STEP"));
    assert!(node.is_supported_extension("step"));
}

#[test]
fn test_plugin_holder_activate_deactivate() {
    let node = ConfigurationNode::new("IGES", "OCC");
    let mut ph = PluginHolder::new(node);
    assert!(ph.can_read());
    ph.deactivate();
    assert!(!ph.can_read());
    ph.activate();
    assert!(ph.can_read());
}

#[test]
fn test_de_wrapper_with_step_can_read_write() {
    let w = DeWrapper::with_step();
    assert_eq!(w.nb_plugins(), 1);
    assert!(w.can_read("STEP"));
    assert!(w.can_write("step"));
}

#[test]
fn test_de_wrapper_all_formats_count_and_missing() {
    let w = DeWrapper::with_all_formats();
    assert_eq!(w.nb_plugins(), 6);
    assert!(w.can_read("STL"));
    assert!(!w.can_read("FBX"));
}
