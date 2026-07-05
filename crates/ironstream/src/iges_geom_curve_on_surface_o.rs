// FILE: iges_geom_curve_on_surface_o.rs
// occt: IGESGeom_CurveOnSurface

/// Represents an IGES curve on a parametric surface (Type 142, Form 0).
/// Associates a given curve with a surface and identifies the curve as lying on it.
pub struct IgesGeomCurveOnSurface {
    creation_mode: i32,
    surface: Option<Box<dyn std::any::Any>>,
    curve_uv: Option<Box<dyn std::any::Any>>,
    curve_3d: Option<Box<dyn std::any::Any>>,
    preference_mode: i32,
}

impl IgesGeomCurveOnSurface {
    /// Creates a new empty CurveOnSurface entity.
    pub fn new() -> Self {
        IgesGeomCurveOnSurface {
            creation_mode: 0,
            surface: None,
            curve_uv: None,
            curve_3d: None,
            preference_mode: 0,
        }
    }

    /// Sets the fields of the CurveOnSurface entity.
    ///
    /// # Arguments
    /// - `mode`: Way the curve on the surface has been created
    ///   - 0 = Unspecified
    ///   - 1 = Projection of a given curve on the surface
    ///   - 2 = Intersection of two surfaces
    ///   - 3 = Isoparametric curve
    /// - `surface`: Surface on which the curve lies
    /// - `curve_uv`: Curve S (UV parametric curve on surface)
    /// - `curve_3d`: Curve C (3D curve in space)
    /// - `preference`: Preference mode
    ///   - 0 = Unspecified
    ///   - 1 = S o B is preferred
    ///   - 2 = C is preferred
    ///   - 3 = C and S o B are equally preferred
    pub fn init(
        &mut self,
        mode: i32,
        surface: Option<Box<dyn std::any::Any>>,
        curve_uv: Option<Box<dyn std::any::Any>>,
        curve_3d: Option<Box<dyn std::any::Any>>,
        preference: i32,
    ) {
        self.creation_mode = mode;
        self.surface = surface;
        self.curve_uv = curve_uv;
        self.curve_3d = curve_3d;
        self.preference_mode = preference;
    }

    /// Returns the mode in which the curve is created on the surface.
    pub fn creation_mode(&self) -> i32 {
        self.creation_mode
    }

    /// Returns the surface on which the curve lies.
    pub fn surface(&self) -> Option<&dyn std::any::Any> {
        self.surface.as_ref().map(|b| b.as_ref())
    }

    /// Returns curve S (UV parametric curve on surface).
    pub fn curve_uv(&self) -> Option<&dyn std::any::Any> {
        self.curve_uv.as_ref().map(|b| b.as_ref())
    }

    /// Returns curve C (3D curve in space).
    pub fn curve_3d(&self) -> Option<&dyn std::any::Any> {
        self.curve_3d.as_ref().map(|b| b.as_ref())
    }

    /// Returns the preference mode.
    pub fn preference_mode(&self) -> i32 {
        self.preference_mode
    }
}

impl Default for IgesGeomCurveOnSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_on_surface_creation() {
        let cos = IgesGeomCurveOnSurface::new();
        assert_eq!(cos.creation_mode(), 0);
        assert_eq!(cos.preference_mode(), 0);
        assert!(cos.surface().is_none());
        assert!(cos.curve_uv().is_none());
        assert!(cos.curve_3d().is_none());
    }

    #[test]
    fn test_curve_on_surface_init() {
        let mut cos = IgesGeomCurveOnSurface::new();
        cos.init(1, None, None, None, 2);
        assert_eq!(cos.creation_mode(), 1);
        assert_eq!(cos.preference_mode(), 2);
    }

    #[test]
    fn test_default() {
        let cos = IgesGeomCurveOnSurface::default();
        assert_eq!(cos.creation_mode(), 0);
    }
}
