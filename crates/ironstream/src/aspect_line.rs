// FILE: aspect_line.rs
// occt: Aspect_TypeOfLine, Aspect_TypeOfMarker, Aspect_TypeOfDisplayText,
//       Aspect_TypeOfDeflection, Aspect_HatchStyle

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypeOfLine {
    Solid,
    Dash,
    Dot,
    DashDot,
    DashDotDot,
    Empty,
    UserDefined,
}

impl TypeOfLine {
    pub fn is_visible(&self) -> bool { *self != TypeOfLine::Empty }
    pub fn dash_pattern(&self) -> &[f32] {
        match self {
            TypeOfLine::Solid => &[],
            TypeOfLine::Dash => &[8.0, 4.0],
            TypeOfLine::Dot => &[1.0, 4.0],
            TypeOfLine::DashDot => &[8.0, 4.0, 1.0, 4.0],
            TypeOfLine::DashDotDot => &[8.0, 4.0, 1.0, 4.0, 1.0, 4.0],
            _ => &[],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypeOfMarker {
    Point,
    Plus,
    Star,
    X,
    O,       // circle
    Square,
    Triangle,
    UserDefined,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfDisplayText {
    Normal,
    Subtitle,
    DecalOut,
    DecalIn,
    Dimension,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfDeflection {
    Chord,       // chord height deflection
    Angular,     // angular deflection
    Both,
    UserDefined,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HatchStyle {
    None,
    Horizontal,
    Vertical,
    Diagonal45,
    Diagonal135,
    HorizontalVertical,
    Diagonal,
    Grid,
}

impl HatchStyle {
    pub fn is_hatched(&self) -> bool { *self != HatchStyle::None }
    pub fn angle_degrees(&self) -> f64 {
        match self {
            HatchStyle::Horizontal => 0.0,
            HatchStyle::Vertical => 90.0,
            HatchStyle::Diagonal45 => 45.0,
            HatchStyle::Diagonal135 => 135.0,
            _ => 0.0,
        }
    }
}

/// Line aspect: combines type, color, width.
#[derive(Clone, Debug)]
pub struct LineAspect {
    pub color: [f32; 3],
    pub line_type: TypeOfLine,
    pub width: f32,
}

impl LineAspect {
    pub fn new(color: [f32; 3], line_type: TypeOfLine, width: f32) -> Self {
        Self { color, line_type, width }
    }

    pub fn solid(color: [f32; 3]) -> Self { Self::new(color, TypeOfLine::Solid, 1.0) }
    pub fn with_width(mut self, w: f32) -> Self { self.width = w; self }
    pub fn with_type(mut self, t: TypeOfLine) -> Self { self.line_type = t; self }
    pub fn is_visible(&self) -> bool { self.line_type.is_visible() }
}

impl Default for LineAspect {
    fn default() -> Self { Self::solid([1.0, 1.0, 1.0]) }
}

/// Marker aspect: marker type, size, color.
#[derive(Clone, Debug)]
pub struct MarkerAspect {
    pub color: [f32; 3],
    pub marker_type: TypeOfMarker,
    pub scale: f32,
}

impl MarkerAspect {
    pub fn new(color: [f32; 3], marker_type: TypeOfMarker, scale: f32) -> Self {
        Self { color, marker_type, scale }
    }
}

impl Default for MarkerAspect {
    fn default() -> Self { Self::new([1.0,1.0,0.0], TypeOfMarker::Point, 1.0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_type_visibility() {
        assert!(TypeOfLine::Solid.is_visible());
        assert!(!TypeOfLine::Empty.is_visible());
    }

    #[test]
    fn hatch_style_angle() {
        assert!((HatchStyle::Diagonal45.angle_degrees() - 45.0).abs() < 1e-10);
        assert!(!HatchStyle::None.is_hatched());
        assert!(HatchStyle::Horizontal.is_hatched());
    }

    #[test]
    fn line_aspect_defaults() {
        let la = LineAspect::solid([1.0, 0.0, 0.0]);
        assert!(la.is_visible());
        assert_eq!(la.line_type, TypeOfLine::Solid);
    }

    #[test]
    fn line_aspect_with_type() {
        let la = LineAspect::solid([1.0,1.0,1.0]).with_type(TypeOfLine::Dash);
        assert_eq!(la.line_type, TypeOfLine::Dash);
        assert!(!la.line_type.dash_pattern().is_empty());
    }
}
