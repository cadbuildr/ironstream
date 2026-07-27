// FILE: tests/occt_osd_env.rs
extern crate ironstream;
use ironstream::osd_env::*;

#[test]
fn env_var_set_get() {
    let mut e = OsdEnvironment::new("MY_VAR");
    assert!(!e.is_set());
    e.set_value("hello");
    assert!(e.is_set());
    assert_eq!(e.value(), "hello");
    // clear → not set
    e.clear();
    assert!(!e.is_set());
}

#[test]
fn process_defaults() {
    let p = OsdProcess::new();
    assert!(p.pid() > 0);
    assert!(!p.is_terminated());
}

#[test]
fn osd_path_parse() {
    let p = OsdPath::from_str("/home/user/model.step");
    assert_eq!(p.extension(), "step");
    assert_eq!(p.name(), "model");
    assert!(p.system_path().contains("model.step"));
}

#[test]
fn osd_directory_sub_dirs() {
    let d = OsdDirectory::new("/tmp/test");
    assert!(d.exists());
    let subs = d.sub_dirs();
    assert_eq!(subs.len(), 2);
}

#[test]
fn env_table_set_get_unset() {
    let mut t = OsdEnvTable::new();
    t.set("PATH", "/usr/bin");
    t.set("HOME", "/home/user");
    assert_eq!(t.get("PATH"), Some("/usr/bin"));
    assert_eq!(t.nb_vars(), 2);
    t.unset("PATH");
    assert!(t.get("PATH").is_none());
    assert_eq!(t.nb_vars(), 1);
}
