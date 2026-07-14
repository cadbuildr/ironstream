// FILE: ais_dim_angle.rs
// occt-ref: PrsDim_AngleDimension, PrsDim_RadiusDimension, PrsDim_DiameterDimension
//       AIS_LengthDimension (additional variants)

/// Dimension kind discriminant.
// occt-ref: PrsDim_DimensionSelectionMode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AisDimensionKind {
    #[default]
    Length,
    Angle,
    Radius,
    Diameter,
}

/// Common dimension display attributes.
/// occt-note: AIS_DimensionAspect (simplified)
#[derive(Clone, Debug)]
pub struct AisDimensionAspect {
    pub text_size: f64,
    pub arrow_size: f64,
    pub is_units_displayed: bool,
    pub is_value_stringified: bool,
    pub units_string: String,
}

impl Default for AisDimensionAspect {
    fn default() -> Self {
        Self {
            text_size: 1.0,
            arrow_size: 0.5,
            is_units_displayed: true,
            is_value_stringified: false,
            units_string: "mm".to_string(),
        }
    }
}

impl AisDimensionAspect {
    pub fn new() -> Self { Self::default() }
    pub fn set_text_size(&mut self, v: f64) { self.text_size = v.max(0.0); }
    pub fn set_arrow_size(&mut self, v: f64) { self.arrow_size = v.max(0.0); }
    pub fn set_units_string(&mut self, s: &str) { self.units_string = s.to_string(); }
}

/// Angle dimension between two lines/edges.
// occt-ref: PrsDim_AngleDimension
#[derive(Clone, Debug)]
pub struct AisAngleDimension {
    pub center: [f64; 3],
    pub first_point: [f64; 3],
    pub second_point: [f64; 3],
    pub custom_value: Option<f64>,
    pub aspect: AisDimensionAspect,
}

impl AisAngleDimension {
    pub fn new(center: [f64; 3], first: [f64; 3], second: [f64; 3]) -> Self {
        Self {
            center,
            first_point: first,
            second_point: second,
            custom_value: None,
            aspect: AisDimensionAspect::default(),
        }
    }

    /// Compute angle from center to the two points.
    pub fn computed_value(&self) -> f64 {
        let v1 = sub3(self.first_point, self.center);
        let v2 = sub3(self.second_point, self.center);
        let dot = dot3(v1, v2);
        let l1 = len3(v1);
        let l2 = len3(v2);
        if l1 < 1e-14 || l2 < 1e-14 { return 0.0; }
        (dot / (l1 * l2)).clamp(-1.0, 1.0).acos()
    }

    pub fn value(&self) -> f64 {
        self.custom_value.unwrap_or_else(|| self.computed_value())
    }

    pub fn value_deg(&self) -> f64 { self.value().to_degrees() }

    pub fn set_custom_value(&mut self, v: f64) { self.custom_value = Some(v); }
    pub fn unset_custom_value(&mut self) { self.custom_value = None; }
    pub fn kind(&self) -> AisDimensionKind { AisDimensionKind::Angle }
}

/// Radius dimension for a circular edge or face.
// occt-ref: PrsDim_RadiusDimension
#[derive(Clone, Debug)]
pub struct AisRadiusDimension {
    pub center: [f64; 3],
    pub radius: f64,
    pub custom_value: Option<f64>,
    pub aspect: AisDimensionAspect,
}

impl AisRadiusDimension {
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        Self { center, radius: radius.max(0.0), custom_value: None, aspect: AisDimensionAspect::default() }
    }

    pub fn value(&self) -> f64 { self.custom_value.unwrap_or(self.radius) }
    pub fn set_custom_value(&mut self, v: f64) { self.custom_value = Some(v.max(0.0)); }
    pub fn kind(&self) -> AisDimensionKind { AisDimensionKind::Radius }
}

/// Diameter dimension (2 × radius).
// occt-ref: PrsDim_DiameterDimension
#[derive(Clone, Debug)]
pub struct AisDiameterDimension {
    pub center: [f64; 3],
    pub radius: f64,
    pub custom_value: Option<f64>,
    pub aspect: AisDimensionAspect,
}

impl AisDiameterDimension {
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        Self { center, radius: radius.max(0.0), custom_value: None, aspect: AisDimensionAspect::default() }
    }

    pub fn value(&self) -> f64 { self.custom_value.unwrap_or(self.radius * 2.0) }
    pub fn set_custom_value(&mut self, v: f64) { self.custom_value = Some(v.max(0.0)); }
    pub fn kind(&self) -> AisDimensionKind { AisDimensionKind::Diameter }
    pub fn radius(&self) -> f64 { self.value() / 2.0 }
}

fn sub3(a: [f64;3], b: [f64;3]) -> [f64;3] { [a[0]-b[0], a[1]-b[1], a[2]-b[2]] }
fn dot3(a: [f64;3], b: [f64;3]) -> f64 { a[0]*b[0] + a[1]*b[1] + a[2]*b[2] }
fn len3(a: [f64;3]) -> f64 { (a[0]*a[0]+a[1]*a[1]+a[2]*a[2]).sqrt() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_dim_90_degrees() {
        let d = AisAngleDimension::new([0.0,0.0,0.0], [1.0,0.0,0.0], [0.0,1.0,0.0]);
        assert!((d.value_deg() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn angle_dim_custom_value() {
        let mut d = AisAngleDimension::new([0.0,0.0,0.0], [1.0,0.0,0.0], [0.0,1.0,0.0]);
        d.set_custom_value(std::f64::consts::PI / 6.0);
        assert!((d.value_deg() - 30.0).abs() < 1e-10);
        d.unset_custom_value();
        assert!((d.value_deg() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn radius_dim() {
        let r = AisRadiusDimension::new([0.0,0.0,0.0], 5.0);
        assert!((r.value() - 5.0).abs() < 1e-10);
        assert_eq!(r.kind(), AisDimensionKind::Radius);
    }

    #[test]
    fn diameter_dim() {
        let d = AisDiameterDimension::new([0.0,0.0,0.0], 3.0);
        assert!((d.value() - 6.0).abs() < 1e-10);
        assert!((d.radius() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn dimension_aspect_defaults() {
        let a = AisDimensionAspect::new();
        assert!((a.text_size - 1.0).abs() < 1e-10);
        assert!(a.is_units_displayed);
        assert_eq!(a.units_string, "mm");
    }
}
