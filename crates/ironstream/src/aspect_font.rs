// FILE: aspect_font.rs
// occt: Aspect_TypeOfFacingModel, Font_NameOfFont, Graphic3d_HorizontalTextAlignment,
//       Graphic3d_VerticalTextAlignment, Graphic3d_RenderTransparentMethod

/// Which side of a face to render.
/// occt: Aspect_TypeOfFacingModel
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AspectFacingModel {
    #[default]
    FrontAndBack,
    Front,
    Back,
}

/// Horizontal text alignment.
/// occt: Graphic3d_HorizontalTextAlignment
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Graphic3dHTextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Vertical text alignment.
/// occt: Graphic3d_VerticalTextAlignment
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Graphic3dVTextAlign {
    #[default]
    Bottom,
    BaseLine,
    Center,
    Top,
}

/// Transparency rendering method.
/// occt: Graphic3d_RenderTransparentMethod
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Graphic3dTransparencyMethod {
    #[default]
    AlphaBlend,
    BlendedOIT,
    WeightedBlendedOIT,
    DepthPeeling4,
    DepthPeeling6,
}

/// Display type for 3D text.
/// occt: Aspect_TypeOfDisplayText
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AspectTypeOfDisplayText {
    #[default]
    Normal,
    Subtitle,
    DecimalOffset,
    Blob,
}

/// Style of a 3D line.
/// occt: Aspect_TypeOfLine
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AspectTypeOfLine {
    #[default]
    Solid,
    Dash,
    Dot,
    DotDash,
    Empty,
    UserDefined,
}

impl AspectTypeOfLine {
    pub fn dash_pattern(&self) -> &[f32] {
        match self {
            Self::Solid => &[],
            Self::Dash => &[8.0, 4.0],
            Self::Dot => &[2.0, 4.0],
            Self::DotDash => &[8.0, 4.0, 2.0, 4.0],
            _ => &[],
        }
    }
}

/// Type of shading model.
/// occt: Graphic3d_TypeOfShadingModel
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Graphic3dShadingModel {
    #[default]
    Facet,
    Gouraud,
    Phong,
    Pbr,
    PbrFacet,
    Unlit,
}

/// Type of reflective material properties.
/// occt: Graphic3d_TypeOfReflection
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Graphic3dReflection {
    #[default]
    Ambient,
    Diffuse,
    Specular,
    Emissive,
}

/// Polygon offset mode.
/// occt: Aspect_PolygonOffsetMode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AspectPolygonOffsetMode {
    #[default]
    None,
    Fill,
    Line,
    Point,
    All,
}

/// Polygon offset parameters.
/// occt: Aspect_PolygonOffset
#[derive(Clone, Copy, Debug)]
pub struct AspectPolygonOffset {
    pub mode: AspectPolygonOffsetMode,
    pub factor: f32,
    pub units: f32,
}

impl Default for AspectPolygonOffset {
    fn default() -> Self {
        Self { mode: AspectPolygonOffsetMode::Fill, factor: 1.0, units: 1.0 }
    }
}

impl AspectPolygonOffset {
    pub fn new(mode: AspectPolygonOffsetMode, factor: f32, units: f32) -> Self {
        Self { mode, factor, units }
    }

    pub fn is_enabled(&self) -> bool { self.mode != AspectPolygonOffsetMode::None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facing_model_default() {
        assert_eq!(AspectFacingModel::default(), AspectFacingModel::FrontAndBack);
    }

    #[test]
    fn line_type_dash_pattern() {
        let p = AspectTypeOfLine::Dash.dash_pattern();
        assert_eq!(p.len(), 2);
        let p2 = AspectTypeOfLine::Solid.dash_pattern();
        assert!(p2.is_empty());
    }

    #[test]
    fn shading_model_default() {
        assert_eq!(Graphic3dShadingModel::default(), Graphic3dShadingModel::Facet);
    }

    #[test]
    fn polygon_offset_enabled() {
        let off = AspectPolygonOffset::default();
        assert!(off.is_enabled());
        let none = AspectPolygonOffset::new(AspectPolygonOffsetMode::None, 0.0, 0.0);
        assert!(!none.is_enabled());
    }

    #[test]
    fn transparency_method_default() {
        assert_eq!(Graphic3dTransparencyMethod::default(), Graphic3dTransparencyMethod::AlphaBlend);
    }
}
