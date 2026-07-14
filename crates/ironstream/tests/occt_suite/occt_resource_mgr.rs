// FILE: tests/occt_resource_mgr.rs
extern crate ironstream;

use ironstream::resource_mgr::ResourceManager;

#[test]
fn resource_manager_set_value_get() {
    let mut rm = ResourceManager::new("config");
    rm.set_value("key1", "value1");
    assert_eq!(rm.value("key1"), Some("value1"));
    assert!(rm.has("key1"));
    assert!(!rm.has("missing"));
    assert_eq!(rm.nb_resources(), 1);
}

#[test]
fn resource_manager_integer_value_and_boolean_value() {
    let mut rm = ResourceManager::new("test");
    rm.set_value("count", "42");
    rm.set_value("flag_true", "true");
    rm.set_value("flag_false", "false");
    rm.set_value("flag_yes", "yes");
    rm.set_value("flag_no", "0");
    assert_eq!(rm.integer_value("count"), Some(42));
    assert_eq!(rm.boolean_value("flag_true"), Some(true));
    assert_eq!(rm.boolean_value("flag_false"), Some(false));
    assert_eq!(rm.boolean_value("flag_yes"), Some(true));
    assert_eq!(rm.boolean_value("flag_no"), Some(false));
}

#[test]
fn resource_manager_merge() {
    let mut rm1 = ResourceManager::new("base");
    rm1.set_value("x", "1");
    rm1.set_value("shared", "original");

    let mut rm2 = ResourceManager::new("override");
    rm2.set_value("y", "2");
    rm2.set_value("shared", "new_value");

    // merge does NOT override existing keys
    rm1.merge(&rm2);
    assert!(rm1.has("y"), "merged key y should exist");
    assert_eq!(rm1.value("shared"), Some("original"), "merge should not override existing keys");
    assert_eq!(rm1.nb_resources(), 3);
}

#[test]
fn resource_manager_real_value_and_remove() {
    let mut rm = ResourceManager::new("nums");
    rm.set_value("pi", "3.14159");
    let pi = rm.real_value("pi").unwrap();
    assert!((pi - 3.14159).abs() < 1e-5);
    let removed = rm.remove("pi");
    assert!(removed);
    assert!(!rm.has("pi"));
}
