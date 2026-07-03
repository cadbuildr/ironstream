// FILE: tests/occt_de_config.rs
extern crate ironstream;
use ironstream::de_config::*;

#[test]
fn config_value_typed_accessors() {
    assert_eq!(ConfigValue::Bool(true).as_bool(), Some(true));
    assert_eq!(ConfigValue::Int(42).as_int(), Some(42));
    assert!((ConfigValue::Float(3.14).as_float().unwrap() - 3.14).abs() < 1e-10);
    assert_eq!(ConfigValue::Str("hello".into()).as_str(), Some("hello"));
}

#[test]
fn config_node_get_set() {
    let mut n = ConfigNode::new("STEP");
    n.set("precision.mode", ConfigValue::Int(1));
    n.set("precision.val", ConfigValue::Float(1e-4));
    assert_eq!(n.get_int("precision.mode"), 1);
    assert!((n.get_float("precision.val") - 1e-4).abs() < 1e-15);
    assert_eq!(n.nb_params(), 2);
    assert!(n.has("precision.mode"));
    assert!(!n.has("nonexistent"));
}

#[test]
fn shape_fix_params_all_enabled() {
    let p = ShapeFixParameters::default_fix();
    assert_eq!(p.nb_fixes_enabled(), 6);
}

#[test]
fn shape_fix_params_none_enabled() {
    let p = ShapeFixParameters::none();
    assert_eq!(p.nb_fixes_enabled(), 0);
    assert_eq!(p.surface_mode, SurfaceFixMode::Off);
}

#[test]
fn de_wrapper_find_node_and_param() {
    let mut w = DeWrapper::new();
    let mut n = ConfigNode::new("IGES");
    n.set("surface.mode", ConfigValue::Bool(true));
    w.add_node(n);
    assert_eq!(w.nb_nodes(), 1);
    assert!(w.find_node("IGES").is_some());
    assert!(w.find_node("missing").is_none());
    let v = w.get_param("IGES", "surface.mode").and_then(|v| v.as_bool());
    assert_eq!(v, Some(true));
}
