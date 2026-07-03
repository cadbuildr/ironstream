// FILE: tests/occt_aspect_line.rs
extern crate ironstream;

use ironstream::aspect_line::{HatchStyle, LineAspect, TypeOfLine};

#[test]
fn type_of_line_is_visible() {
    assert!(TypeOfLine::Solid.is_visible());
    assert!(TypeOfLine::Dash.is_visible());
    assert!(!TypeOfLine::Empty.is_visible());
}

#[test]
fn type_of_line_dash_pattern() {
    assert!(TypeOfLine::Solid.dash_pattern().is_empty(), "solid has no dash pattern");
    assert!(!TypeOfLine::Dash.dash_pattern().is_empty(), "dash has a pattern");
    assert!(!TypeOfLine::DashDot.dash_pattern().is_empty());
}

#[test]
fn hatch_style_is_hatched_and_angle() {
    assert!(!HatchStyle::None.is_hatched());
    assert!(HatchStyle::Horizontal.is_hatched());
    assert!(HatchStyle::Diagonal45.is_hatched());
    assert!((HatchStyle::Diagonal45.angle_degrees() - 45.0).abs() < 1e-10);
    assert!((HatchStyle::Vertical.angle_degrees() - 90.0).abs() < 1e-10);
    assert!((HatchStyle::Diagonal135.angle_degrees() - 135.0).abs() < 1e-10);
}

#[test]
fn line_aspect_solid_and_with_type() {
    let la = LineAspect::solid([1.0, 0.0, 0.0]);
    assert!(la.is_visible());
    assert_eq!(la.line_type, TypeOfLine::Solid);
    assert!((la.width - 1.0).abs() < 1e-6);

    let dashed = la.with_type(TypeOfLine::Dash);
    assert_eq!(dashed.line_type, TypeOfLine::Dash);
    assert!(dashed.is_visible());
    assert!(!dashed.line_type.dash_pattern().is_empty());
}

#[test]
fn line_aspect_empty_is_invisible() {
    let la = LineAspect::solid([0.0, 0.0, 0.0]).with_type(TypeOfLine::Empty);
    assert!(!la.is_visible());
}
