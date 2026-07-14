extern crate ironstream;
use ironstream::geom_thickness::*;

#[test]
fn occt_thick_solid_smoke() {
    let ts = ThickSolid::new("box_1", 2.0);
    assert!(ts.is_done());
    let result = ts.build();
    assert!(result.contains("box_1"));
}

#[test]
fn occt_shell_thickness_smoke() {
    let st = ShellThickness::new("shell_1", 0.5);
    let result = st.build();
    assert!(result.contains("shell_1"));
}

#[test]
fn occt_make_thick_solid_free_fn() {
    let result = make_thick_solid("body", &["top"], 1.5);
    assert!(result.contains("body"));
    assert!(result.contains("top"));
}
