// FILE: rust/ironstream/crates/ironstream/src/prs3d_point_aspect.rs

// occt: Graphic3d_AspectMarker3d
/// Marker shape variants, mirroring the marker-type constants exposed by
/// Graphic3d_AspectMarker3d (Aspect_TypeOfMarker in OCCT).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerType {
    /// Single pixel dot.
    Point,
    /// Plus / cross (+).
    Plus,
    /// Six-pointed star (*).
    Star,
    /// Diagonal cross (x).
    X,
    /// Circle (O).
    O,
    /// Filled / open square.
    Square,
    /// Filled / open diamond.
    Diamond,
}

// occt: Prs3d_PointAspect
/// High-level point / marker display attributes used by the Prs3d presentation layer.
/// Corresponds to Prs3d_PointAspect in OCCT.
#[derive(Debug, Clone, PartialEq)]
pub struct PointAspect {
    /// RGB colour in [0.0, 1.0] per channel.
    pub color: [f64; 3],
    /// Shape of the rendered marker.
    pub marker_type: MarkerType,
    /// Uniform scale factor applied to the marker glyph (pixels or model units,
    /// depending on the renderer).
    pub scale: f64,
}

impl PointAspect {
    /// Create a new `PointAspect` with the given marker type, colour, and scale.
    ///
    /// # Arguments
    /// * `mt`    – Marker shape.
    /// * `r`, `g`, `b` – RGB colour components in [0.0, 1.0].
    /// * `scale` – Marker scale factor.
    pub fn new(mt: MarkerType, r: f64, g: f64, b: f64, scale: f64) -> Self {
        Self {
            color: [r, g, b],
            marker_type: mt,
            scale,
        }
    }

    /// Set the RGB display colour.  Each component should be in [0.0, 1.0].
    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        self.color = [r, g, b];
    }

    /// Set the marker scale factor.
    pub fn set_scale(&mut self, s: f64) {
        self.scale = s;
    }

    /// Set the marker shape.
    pub fn set_type(&mut self, mt: MarkerType) {
        self.marker_type = mt;
    }

    /// Return the current RGB colour.
    pub fn color(&self) -> [f64; 3] {
        self.color
    }
}

impl std::default::Default for PointAspect {
    fn default() -> Self {
        PointAspect::new(MarkerType::Point, 1.0, 1.0, 1.0, 1.0)
    }
}

// occt: Graphic3d_AspectMarker3d
/// Low-level 3-D marker rendering attributes, mirroring Graphic3d_AspectMarker3d in OCCT.
#[derive(Debug, Clone, PartialEq)]
pub struct AspectMarker3d {
    /// RGB colour in [0.0, 1.0] per channel.
    pub color: [f64; 3],
    /// Shape of the rendered marker.
    pub marker_type: MarkerType,
    /// Uniform scale factor applied to the marker glyph.
    pub scale: f64,
}

impl AspectMarker3d {
    /// Create a new `AspectMarker3d` with OCCT-compatible defaults:
    /// white colour, `Point` marker type, scale 1.0.
    pub fn new() -> Self {
        Self {
            color: [1.0, 1.0, 1.0],
            marker_type: MarkerType::Point,
            scale: 1.0,
        }
    }

    /// Return a default `AspectMarker3d` configured as a plain dot (`Point` type),
    /// matching the most common use-case in Prs3d_PointAspect.
    pub fn default_point() -> Self {
        Self {
            color: [1.0, 1.0, 1.0],
            marker_type: MarkerType::Point,
            scale: 1.0,
        }
    }
}

impl std::default::Default for AspectMarker3d {
    fn default() -> Self {
        AspectMarker3d::new()
    }
}
