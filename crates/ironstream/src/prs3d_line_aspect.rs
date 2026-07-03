// FILE: rust/ironstream/crates/ironstream/src/prs3d_line_aspect.rs

// occt: Graphic3d_AspectLine3d
/// Stipple / dash pattern for a rendered line, mirroring the line-type variants
/// exposed by Graphic3d_AspectLine3d (Aspect_TypeOfLine in OCCT).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineType {
    Solid,
    Dash,
    Dot,
    DotDash,
}

// occt: Prs3d_LineAspect
/// High-level line display attributes used by the Prs3d presentation layer.
/// Corresponds to Prs3d_LineAspect in OCCT.
#[derive(Debug, Clone, PartialEq)]
pub struct LineAspect {
    /// RGB colour in [0.0, 1.0] per channel.
    pub color: [f64; 3],
    /// Stipple / dash pattern for the line.
    pub line_type: LineType,
    /// Line width in pixels (or model units, depending on the renderer).
    pub width: f64,
}

impl LineAspect {
    /// Create a new `LineAspect` with the given colour, line type, and width.
    ///
    /// # Arguments
    /// * `r`, `g`, `b` – RGB colour components in [0.0, 1.0].
    /// * `lt` – Stipple pattern.
    /// * `w` – Line width.
    pub fn new(r: f64, g: f64, b: f64, lt: LineType, w: f64) -> Self {
        Self {
            color: [r, g, b],
            line_type: lt,
            width: w,
        }
    }

    /// Set the RGB display colour.  Each component should be in [0.0, 1.0].
    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        self.color = [r, g, b];
    }

    /// Set the line width.
    pub fn set_width(&mut self, w: f64) {
        self.width = w;
    }

    /// Set the stipple / dash pattern.
    pub fn set_type(&mut self, lt: LineType) {
        self.line_type = lt;
    }

    /// Return the current RGB colour.
    pub fn color(&self) -> [f64; 3] {
        self.color
    }
}

impl std::default::Default for LineAspect {
    fn default() -> Self {
        LineAspect::new(1.0, 1.0, 1.0, LineType::Solid, 1.0)
    }
}

// occt: Graphic3d_AspectLine3d
/// Low-level 3-D line rendering attributes, mirroring Graphic3d_AspectLine3d in OCCT.
#[derive(Debug, Clone, PartialEq)]
pub struct AspectLine3d {
    /// RGB colour in [0.0, 1.0] per channel.
    pub color: [f64; 3],
    /// Stipple / dash pattern for the line.
    pub line_type: LineType,
    /// Line width in pixels (or model units, depending on the renderer).
    pub width: f64,
}

impl AspectLine3d {
    /// Create a new `AspectLine3d` with OCCT-compatible defaults:
    /// white colour, solid line type, width 1.0.
    pub fn new() -> Self {
        Self {
            color: [1.0, 1.0, 1.0],
            line_type: LineType::Solid,
            width: 1.0,
        }
    }

    /// Return a default `AspectLine3d` (delegates to [`new`]).
    pub fn default() -> Self {
        Self::new()
    }
}

impl std::default::Default for AspectLine3d {
    fn default() -> Self {
        AspectLine3d::new()
    }
}
