// FILE: tests/occt_aspect_font.rs
extern crate ironstream;
use ironstream::aspect_font::*;

#[test]
fn facing_model_default() {
    assert_eq!(AspectFacingModel::default(), AspectFacingModel::FrontAndBack);
}

#[test]
fn line_dash_pattern() {
    // Dash → 2 elements
    let dash = AspectTypeOfLine::Dash.dash_pattern();
    assert_eq!(dash.len(), 2);
    // Solid → empty
    let solid = AspectTypeOfLine::Solid.dash_pattern();
    assert!(solid.is_empty());
}

#[test]
fn shading_model_default() {
    assert_eq!(Graphic3dShadingModel::default(), Graphic3dShadingModel::Facet);
}

#[test]
fn polygon_offset_enabled() {
    // Default mode is Fill → enabled
    let off = AspectPolygonOffset::default();
    assert!(off.is_enabled());
    // Mode=None → disabled
    let none = AspectPolygonOffset::new(AspectPolygonOffsetMode::None, 0.0, 0.0);
    assert!(!none.is_enabled());
}

#[test]
fn transparency_method_default() {
    assert_eq!(
        Graphic3dTransparencyMethod::default(),
        Graphic3dTransparencyMethod::AlphaBlend
    );
}
