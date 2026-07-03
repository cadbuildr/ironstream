// FILE: ais_dim_length.rs
// occt: AIS_LengthDimension, AIS_AngleDimension, AIS_RadiusDimension,
//       AIS_DiameterDimension

use std::f64::consts::PI;

/// Type of distance measurement.
/// occt: AIS_TypeOfDist
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisTypeOfDist {
    TwoPoints,
    Horizontal,
    Vertical,
    Orthogonal,
}

impl Default for AisTypeOfDist {
    fn default() -> Self { Self::TwoPoints }
}

/// Length dimension: distance between two 3D points.
/// occt: AIS_LengthDimension
#[derive(Clone, Debug)]
pub struct AisLengthDimension {
    pub dim_id: u32,
    pub point1: [f64; 3],
    pub point2: [f64; 3],
    pub dist_type: AisTypeOfDist,
    pub plane_normal: [f64; 3],
    pub text_position: [f64; 3],
    pub flyout: f64,
    pub is_valid: bool,
    pub unit_suffix: String,
}

impl AisLengthDimension {
    pub fn new(dim_id: u32, p1: [f64; 3], p2: [f64; 3]) -> Self {
        let valid = p1 != p2;
        let mid = [(p1[0]+p2[0])/2.0, (p1[1]+p2[1])/2.0, (p1[2]+p2[2])/2.0];
        Self {
            dim_id,
            point1: p1,
            point2: p2,
            dist_type: AisTypeOfDist::TwoPoints,
            plane_normal: [0.0, 0.0, 1.0],
            text_position: mid,
            flyout: 0.0,
            is_valid: valid,
            unit_suffix: String::new(),
        }
    }

    pub fn measured_value(&self) -> f64 {
        let d = [self.point2[0]-self.point1[0], self.point2[1]-self.point1[1], self.point2[2]-self.point1[2]];
        (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt()
    }

    pub fn set_flyout(&mut self, f: f64) { self.flyout = f; }
    pub fn set_plane_normal(&mut self, n: [f64; 3]) { self.plane_normal = n; }
    pub fn set_dist_type(&mut self, t: AisTypeOfDist) { self.dist_type = t; }
    pub fn set_unit_suffix(&mut self, s: &str) { self.unit_suffix = s.to_string(); }

    pub fn is_valid(&self) -> bool { self.is_valid }
    pub fn dim_id(&self) -> u32 { self.dim_id }
    pub fn dist_type(&self) -> AisTypeOfDist { self.dist_type }

    pub fn formatted_value(&self, decimals: usize) -> String {
        format!("{:.prec$}{}", self.measured_value(), self.unit_suffix, prec=decimals)
    }
}

/// Angle dimension: angle at a vertex between two edges.
/// occt: AIS_AngleDimension
#[derive(Clone, Debug)]
pub struct AisAngleDimension {
    pub dim_id: u32,
    pub vertex: [f64; 3],
    pub point1: [f64; 3],  // endpoint on first edge
    pub point2: [f64; 3],  // endpoint on second edge
    pub flyout: f64,
    pub is_valid: bool,
    pub unit_suffix: String,
}

impl AisAngleDimension {
    pub fn new(dim_id: u32, vertex: [f64; 3], p1: [f64; 3], p2: [f64; 3]) -> Self {
        let v1 = [p1[0]-vertex[0], p1[1]-vertex[1], p1[2]-vertex[2]];
        let v2 = [p2[0]-vertex[0], p2[1]-vertex[1], p2[2]-vertex[2]];
        let l1 = (v1[0]*v1[0]+v1[1]*v1[1]+v1[2]*v1[2]).sqrt();
        let l2 = (v2[0]*v2[0]+v2[1]*v2[1]+v2[2]*v2[2]).sqrt();
        let valid = l1 > 1e-14 && l2 > 1e-14;
        Self { dim_id, vertex, point1: p1, point2: p2, flyout: 0.0, is_valid: valid, unit_suffix: String::from("°") }
    }

    pub fn measured_value_deg(&self) -> f64 {
        if !self.is_valid { return 0.0; }
        let v1 = [self.point1[0]-self.vertex[0], self.point1[1]-self.vertex[1], self.point1[2]-self.vertex[2]];
        let v2 = [self.point2[0]-self.vertex[0], self.point2[1]-self.vertex[1], self.point2[2]-self.vertex[2]];
        let l1 = (v1[0]*v1[0]+v1[1]*v1[1]+v1[2]*v1[2]).sqrt();
        let l2 = (v2[0]*v2[0]+v2[1]*v2[1]+v2[2]*v2[2]).sqrt();
        if l1 < 1e-14 || l2 < 1e-14 { return 0.0; }
        let dot = (v1[0]*v2[0]+v1[1]*v2[1]+v1[2]*v2[2]) / (l1*l2);
        dot.clamp(-1.0, 1.0).acos() * 180.0 / PI
    }

    pub fn set_flyout(&mut self, f: f64) { self.flyout = f; }
    pub fn is_valid(&self) -> bool { self.is_valid }
    pub fn dim_id(&self) -> u32 { self.dim_id }

    pub fn formatted_value(&self, decimals: usize) -> String {
        format!("{:.prec$}{}", self.measured_value_deg(), self.unit_suffix, prec=decimals)
    }
}

/// Radius dimension: radius of a circle/arc.
/// occt: AIS_RadiusDimension
#[derive(Clone, Debug)]
pub struct AisRadiusDimension {
    pub dim_id: u32,
    pub center: [f64; 3],
    pub radius: f64,
    pub anchor_point: [f64; 3],
    pub flyout: f64,
    pub is_valid: bool,
}

impl AisRadiusDimension {
    pub fn new(dim_id: u32, center: [f64; 3], radius: f64) -> Self {
        let valid = radius > 0.0;
        Self {
            dim_id, center,
            radius,
            anchor_point: [center[0] + radius, center[1], center[2]],
            flyout: 0.0,
            is_valid: valid,
        }
    }

    pub fn measured_value(&self) -> f64 { self.radius }
    pub fn is_valid(&self) -> bool { self.is_valid }

    pub fn set_flyout(&mut self, f: f64) { self.flyout = f; }
    pub fn set_anchor_point(&mut self, p: [f64; 3]) { self.anchor_point = p; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_dim_measured_value() {
        let d = AisLengthDimension::new(1, [0.0,0.0,0.0], [3.0,4.0,0.0]);
        assert!(d.is_valid());
        assert!((d.measured_value() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn length_dim_formatted() {
        let mut d = AisLengthDimension::new(1, [0.0,0.0,0.0], [1.0,0.0,0.0]);
        d.set_unit_suffix("mm");
        let s = d.formatted_value(2);
        assert!(s.contains("mm"));
    }

    #[test]
    fn angle_dim_90_degrees() {
        let d = AisAngleDimension::new(1, [0.0,0.0,0.0], [1.0,0.0,0.0], [0.0,1.0,0.0]);
        assert!(d.is_valid());
        assert!((d.measured_value_deg() - 90.0).abs() < 1e-8);
    }

    #[test]
    fn angle_dim_invalid() {
        let d = AisAngleDimension::new(1, [0.0,0.0,0.0], [0.0,0.0,0.0], [1.0,0.0,0.0]);
        assert!(!d.is_valid());
    }

    #[test]
    fn radius_dim() {
        let d = AisRadiusDimension::new(1, [0.0,0.0,0.0], 5.0);
        assert!(d.is_valid());
        assert!((d.measured_value() - 5.0).abs() < 1e-10);
    }
}
