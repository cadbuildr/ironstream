extern crate ironstream;
use ironstream::brep_solid_tools::*;

#[test]
fn make_solid_from_closed_shell() {
    let mut s = MakeSolid::new();
    s.add_shell(ShellInput::new(6, true, 8.0));
    s.build();
    assert!(s.is_done());
    assert_eq!(s.nb_shells(), 1);
    assert!((s.total_volume() - 8.0).abs() < 1e-10);
}

#[test]
fn make_solid_open_shell_fails() {
    let mut s = MakeSolid::new();
    s.add_shell(ShellInput::new(5, false, 0.0));
    s.build();
    assert!(!s.is_done());
    assert!(s.error.is_some());
}

#[test]
fn free_bounds_analysis() {
    let mut f = FreeBoundsAnalysis::new();
    f.perform(10, 8);
    assert_eq!(f.nb_closed_wires(), 1);
    assert_eq!(f.nb_open_wires(), 1);
    assert!(f.has_free_bounds());
    let f2_closed_only = {
        let mut x = FreeBoundsAnalysis::new();
        x.perform(4, 4);
        x
    };
    assert!(!f2_closed_only.has_free_bounds());
}

#[test]
fn check_solid_valid_no_free_edges() {
    let c = CheckSolid::new(1, 6, false);
    assert!(c.is_valid());
    assert_eq!(c.nb_errors(), 0);
}

#[test]
fn make_offset_build() {
    let mut m = MakeOffset::new(6, 0.5);
    m.set_tolerance(1e-5);
    m.build();
    assert!(m.is_done());
    assert!((m.offset_value() - 0.5).abs() < 1e-10);
}
