// FILE: prs_dim.rs
// occt-ref: PrsDim_AngleDimension, PrsDim_LengthDimension, PrsDim_RadiusDimension

use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DimensionType {
    Length,
    Angle,
    Radius,
    Diameter,
    Distance,
}

// occt-ref: PrsDim_Dimension // (base)
#[derive(Clone, Debug)]
pub struct DimensionBase {
    pub dim_type: DimensionType,
    pub value: f64,
    pub unit_factor: f64,
    pub is_text_reversed: bool,
}

impl DimensionBase {
    pub fn new(dim_type: DimensionType, value: f64) -> Self {
        Self { dim_type, value, unit_factor: 1.0, is_text_reversed: false }
    }

    pub fn display_value(&self) -> f64 {
        self.value * self.unit_factor
    }
}

// occt-ref: PrsDim_LengthDimension
#[derive(Clone, Debug)]
pub struct LengthDimension {
    pub base: DimensionBase,
    pub point1: [f64; 3],
    pub point2: [f64; 3],
    pub direction: [f64; 3],
}

impl LengthDimension {
    pub fn new(point1: [f64; 3], point2: [f64; 3]) -> Self {
        let dx = point2[0] - point1[0];
        let dy = point2[1] - point1[1];
        let dz = point2[2] - point1[2];
        let len = (dx*dx + dy*dy + dz*dz).sqrt();
        let dir = if len > 1e-14 { [dx/len, dy/len, dz/len] } else { [1.0, 0.0, 0.0] };
        Self {
            base: DimensionBase::new(DimensionType::Length, len),
            point1, point2, direction: dir,
        }
    }

    pub fn length(&self) -> f64 { self.base.value }
    pub fn set_unit_factor(&mut self, f: f64) { self.base.unit_factor = f; }
    pub fn display_value(&self) -> f64 { self.base.display_value() }
}

// occt-ref: PrsDim_AngleDimension
#[derive(Clone, Debug)]
pub struct AngleDimension {
    pub base: DimensionBase,
    pub vertex: [f64; 3],
    pub point1: [f64; 3],
    pub point2: [f64; 3],
}

impl AngleDimension {
    pub fn new(vertex: [f64; 3], point1: [f64; 3], point2: [f64; 3]) -> Self {
        let v1 = sub3(point1, vertex);
        let v2 = sub3(point2, vertex);
        let angle = angle_between(v1, v2);
        Self {
            base: DimensionBase::new(DimensionType::Angle, angle),
            vertex, point1, point2,
        }
    }

    pub fn angle_radians(&self) -> f64 { self.base.value }
    pub fn angle_degrees(&self) -> f64 { self.base.value * 180.0 / PI }
}

// occt-ref: PrsDim_RadiusDimension
#[derive(Clone, Debug)]
pub struct RadiusDimension {
    pub base: DimensionBase,
    pub center: [f64; 3],
    pub point_on_arc: [f64; 3],
}

impl RadiusDimension {
    pub fn new(center: [f64; 3], point_on_arc: [f64; 3]) -> Self {
        let r = dist3(center, point_on_arc);
        Self { base: DimensionBase::new(DimensionType::Radius, r), center, point_on_arc }
    }

    pub fn radius(&self) -> f64 { self.base.value }
    pub fn diameter(&self) -> f64 { self.base.value * 2.0 }
}

// occt-ref: PrsDim_DiameterDimension
#[derive(Clone, Debug)]
pub struct DiameterDimension {
    pub base: DimensionBase,
    pub center: [f64; 3],
    pub radius: f64,
}

impl DiameterDimension {
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        Self { base: DimensionBase::new(DimensionType::Diameter, radius * 2.0), center, radius }
    }

    pub fn diameter(&self) -> f64 { self.base.value }
}

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0]-b[0], a[1]-b[1], a[2]-b[2]]
}

fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
}

fn len3(v: [f64; 3]) -> f64 {
    (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt()
}

fn dist3(a: [f64; 3], b: [f64; 3]) -> f64 {
    len3(sub3(a, b))
}

fn angle_between(a: [f64; 3], b: [f64; 3]) -> f64 {
    let la = len3(a);
    let lb = len3(b);
    if la < 1e-14 || lb < 1e-14 { return 0.0; }
    (dot3(a, b) / (la * lb)).clamp(-1.0, 1.0).acos()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_dimension_horizontal() {
        let d = LengthDimension::new([0.0,0.0,0.0], [5.0,0.0,0.0]);
        assert!((d.length() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn angle_dimension_right_angle() {
        let d = AngleDimension::new(
            [0.0,0.0,0.0], [1.0,0.0,0.0], [0.0,1.0,0.0]
        );
        assert!((d.angle_degrees() - 90.0).abs() < 1e-8);
    }

    #[test]
    fn radius_dimension() {
        let d = RadiusDimension::new([0.0,0.0,0.0], [3.0,0.0,0.0]);
        assert!((d.radius() - 3.0).abs() < 1e-10);
        assert!((d.diameter() - 6.0).abs() < 1e-10);
    }

    #[test]
    fn diameter_dimension() {
        let d = DiameterDimension::new([0.0,0.0,0.0], 2.5);
        assert!((d.diameter() - 5.0).abs() < 1e-10);
    }
}
