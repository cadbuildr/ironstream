extern crate ironstream;
use ironstream::brep_check_solid::*;

#[test]
fn test_brep_check_solid_basic() {
    let mut checker = SolidChecker::new("valid_box");
    checker.perform();
    assert!(checker.is_valid());
    assert_eq!(checker.error_count(), 0);

    let mut shell = ShellChecker::new("closed_shell");
    shell.perform();
    assert_eq!(shell.free_edge_count(), 0);

    let result = check_solid("valid_box");
    assert!(result);
}
