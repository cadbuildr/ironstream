extern crate ironstream;
use ironstream::prs3d_line_aspect::*;

#[test]
fn test_prs3d_line_aspect_basic() {
    // LineType enum
    assert_eq!(LineType::Solid, LineType::Solid);
    assert_ne!(LineType::Dash, LineType::Dot);

    // LineAspect construction and accessors
    let mut la = LineAspect::new(0.0, 0.5, 1.0, LineType::Dash, 2.0);
    assert_eq!(la.color, [0.0, 0.5, 1.0]);
    assert_eq!(la.line_type, LineType::Dash);
    assert_eq!(la.width, 2.0);
    assert_eq!(la.color(), [0.0, 0.5, 1.0]);

    la.set_color(1.0, 0.0, 0.0);
    assert_eq!(la.color, [1.0, 0.0, 0.0]);

    la.set_width(3.5);
    assert_eq!(la.width, 3.5);

    la.set_type(LineType::DotDash);
    assert_eq!(la.line_type, LineType::DotDash);

    // Default trait
    let ld: LineAspect = Default::default();
    assert_eq!(ld.color, [1.0, 1.0, 1.0]);
    assert_eq!(ld.line_type, LineType::Solid);
    assert_eq!(ld.width, 1.0);

    // AspectLine3d defaults
    let al = AspectLine3d::new();
    assert_eq!(al.color, [1.0, 1.0, 1.0]);
    assert_eq!(al.line_type, LineType::Solid);
    assert_eq!(al.width, 1.0);

    // Default trait
    let ald: AspectLine3d = Default::default();
    assert_eq!(ald.line_type, LineType::Solid);
}
