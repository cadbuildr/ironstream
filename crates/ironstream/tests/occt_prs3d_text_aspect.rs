extern crate ironstream;
use ironstream::prs3d_text_aspect::*;

#[test]
fn test_prs3d_text_aspect_basic() {
    // FontSlant and FontWeight enums
    let slant = FontSlant::Italic;
    assert_eq!(slant, FontSlant::Italic);
    assert_ne!(slant, FontSlant::Regular);

    let weight = FontWeight::Bold;
    assert_eq!(weight, FontWeight::Bold);

    // TextAspect defaults
    let mut ta = TextAspect::new();
    assert_eq!(ta.color, [0.0, 0.0, 0.0]);
    assert_eq!(ta.font, "Courier");
    assert_eq!(ta.height, 12.0);
    assert_eq!(ta.angle, 0.0);
    assert_eq!(ta.h_align, 0);
    assert_eq!(ta.v_align, 0);

    // setters
    ta.set_color(1.0, 0.5, 0.0);
    assert_eq!(ta.color, [1.0, 0.5, 0.0]);
    ta.set_font("Arial");
    assert_eq!(ta.font, "Arial");
    ta.set_height(24.0);
    assert_eq!(ta.height, 24.0);
    ta.set_angle(std::f64::consts::PI / 4.0);
    assert!((ta.angle - std::f64::consts::PI / 4.0).abs() < 1e-12);

    // Default trait
    let td: TextAspect = Default::default();
    assert_eq!(td.font, "Courier");

    // AspectText3d defaults
    let mut at = AspectText3d::new();
    assert_eq!(at.color, [0.0, 0.0, 0.0]);
    assert_eq!(at.font_name, "Courier");
    assert_eq!(at.height, 12.0);
    assert_eq!(at.slant, FontSlant::Regular);
    assert_eq!(at.weight, FontWeight::Normal);

    at.set_color(0.0, 1.0, 0.0);
    assert_eq!(at.color, [0.0, 1.0, 0.0]);
    at.set_font("Helvetica", 16.0);
    assert_eq!(at.font_name, "Helvetica");
    assert_eq!(at.height, 16.0);

    // Default trait
    let atd: AspectText3d = Default::default();
    assert_eq!(atd.weight, FontWeight::Normal);
}
