// FILE: prs3d_dimension.rs
// occt: Prs3d_DimensionAspect, Prs3d_DimensionUnits
// occt-ref: Prs3d_Dimension
//       AIS_LengthDimension, AIS_AngleDimension, AIS_RadiusDimension,
//       AIS_DiameterDimension

/// Dimension type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DimensionType {
    Length,
    Angle,
    Radius,
    Diameter,
    Distance,
    Chamfer,
    Fillet,
}

/// Text placement relative to the dimension line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DimensionTextPos {
    Center,
    HorzCenter,
    Left,
    Right,
    Above,
    Below,
}

/// Unit display mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DimensionUnit {
    Mm,
    Cm,
    M,
    Inch,
    Deg,
    Rad,
}

impl DimensionUnit {
    pub fn factor_from_mm(&self) -> f64 {
        match self {
            DimensionUnit::Mm => 1.0,
            DimensionUnit::Cm => 0.1,
            DimensionUnit::M => 0.001,
            DimensionUnit::Inch => 1.0 / 25.4,
            DimensionUnit::Deg | DimensionUnit::Rad => 1.0,
        }
    }

    pub fn suffix(&self) -> &'static str {
        match self {
            DimensionUnit::Mm => " mm",
            DimensionUnit::Cm => " cm",
            DimensionUnit::M => " m",
            DimensionUnit::Inch => "\"",
            DimensionUnit::Deg => "°",
            DimensionUnit::Rad => " rad",
        }
    }
}

// occt: Prs3d_DimensionAspect
#[derive(Clone, Debug)]
pub struct DimensionAspect {
    pub line_color: [f32; 3],
    pub text_color: [f32; 3],
    pub arrow_color: [f32; 3],
    pub line_width: f32,
    pub text_height: f32,
    pub arrow_length: f64,
    pub arrow_angle: f64,
    pub text_pos: DimensionTextPos,
    pub units_display: bool,
    pub decimals: u32,
}

impl DimensionAspect {
    pub fn new() -> Self {
        Self {
            line_color: [1.0, 1.0, 1.0],
            text_color: [1.0, 1.0, 1.0],
            arrow_color: [1.0, 1.0, 1.0],
            line_width: 1.0,
            text_height: 14.0,
            arrow_length: 5.0,
            arrow_angle: 20.0_f64.to_radians(),
            text_pos: DimensionTextPos::Center,
            units_display: true,
            decimals: 2,
        }
    }

    pub fn with_line_color(mut self, c: [f32; 3]) -> Self { self.line_color = c; self }
    pub fn with_text_height(mut self, h: f32) -> Self { self.text_height = h; self }
    pub fn with_units(mut self, v: bool) -> Self { self.units_display = v; self }
    pub fn with_decimals(mut self, d: u32) -> Self { self.decimals = d; self }

    pub fn arrow_tip(&self, origin: [f64; 3], dir: [f64; 3]) -> [f64; 3] {
        [
            origin[0] + dir[0] * self.arrow_length,
            origin[1] + dir[1] * self.arrow_length,
            origin[2] + dir[2] * self.arrow_length,
        ]
    }
}

impl Default for DimensionAspect {
    fn default() -> Self { Self::new() }
}

// occt: Prs3d_DimensionUnits
#[derive(Clone, Debug)]
pub struct DimensionUnits {
    pub length_unit: DimensionUnit,
    pub angle_unit: DimensionUnit,
}

impl DimensionUnits {
    pub fn metric() -> Self { Self { length_unit: DimensionUnit::Mm, angle_unit: DimensionUnit::Deg } }
    pub fn imperial() -> Self { Self { length_unit: DimensionUnit::Inch, angle_unit: DimensionUnit::Deg } }
}

impl Default for DimensionUnits {
    fn default() -> Self { Self::metric() }
}

// occt: PrsDim_LengthDimension
#[derive(Clone, Debug)]
pub struct LengthDimension {
    pub point1: [f64; 3],
    pub point2: [f64; 3],
    pub direction: [f64; 3],   // projection direction
    pub aspect: DimensionAspect,
    pub units: DimensionUnits,
    pub custom_value: Option<f64>,
    pub is_valid: bool,
}

impl LengthDimension {
    pub fn new(p1: [f64; 3], p2: [f64; 3]) -> Self {
        Self {
            point1: p1, point2: p2,
            direction: [0.0, 0.0, 1.0],
            aspect: DimensionAspect::new(),
            units: DimensionUnits::metric(),
            custom_value: None,
            is_valid: true,
        }
    }

    pub fn with_direction(mut self, d: [f64; 3]) -> Self { self.direction = d; self }
    pub fn with_custom_value(mut self, v: f64) -> Self { self.custom_value = Some(v); self }

    pub fn computed_value(&self) -> f64 {
        self.custom_value.unwrap_or_else(|| {
            let d = [self.point2[0]-self.point1[0], self.point2[1]-self.point1[1], self.point2[2]-self.point1[2]];
            (d[0]*d[0] + d[1]*d[1] + d[2]*d[2]).sqrt()
        })
    }

    pub fn display_string(&self) -> String {
        let v = self.computed_value() * self.units.length_unit.factor_from_mm();
        let sfx = self.units.length_unit.suffix();
        format!("{:.prec$}{sfx}", v, prec = self.aspect.decimals as usize)
    }

    pub fn midpoint(&self) -> [f64; 3] {
        [
            (self.point1[0] + self.point2[0]) * 0.5,
            (self.point1[1] + self.point2[1]) * 0.5,
            (self.point1[2] + self.point2[2]) * 0.5,
        ]
    }
}

// occt: PrsDim_AngleDimension
#[derive(Clone, Debug)]
pub struct AngleDimension {
    pub vertex: [f64; 3],
    pub point1: [f64; 3],
    pub point2: [f64; 3],
    pub aspect: DimensionAspect,
    pub units: DimensionUnits,
    pub custom_value: Option<f64>,
    pub is_valid: bool,
}

impl AngleDimension {
    pub fn new(vertex: [f64; 3], p1: [f64; 3], p2: [f64; 3]) -> Self {
        Self {
            vertex, point1: p1, point2: p2,
            aspect: DimensionAspect::new(),
            units: DimensionUnits::metric(),
            custom_value: None,
            is_valid: true,
        }
    }

    pub fn computed_angle_rad(&self) -> f64 {
        if let Some(v) = self.custom_value { return v; }
        let u = [self.point1[0]-self.vertex[0], self.point1[1]-self.vertex[1], self.point1[2]-self.vertex[2]];
        let v = [self.point2[0]-self.vertex[0], self.point2[1]-self.vertex[1], self.point2[2]-self.vertex[2]];
        let lu = (u[0]*u[0]+u[1]*u[1]+u[2]*u[2]).sqrt();
        let lv = (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt();
        if lu < 1e-14 || lv < 1e-14 { return 0.0; }
        let dot = (u[0]*v[0]+u[1]*v[1]+u[2]*v[2]) / (lu*lv);
        dot.clamp(-1.0, 1.0).acos()
    }

    pub fn computed_angle_deg(&self) -> f64 { self.computed_angle_rad().to_degrees() }

    pub fn display_string(&self) -> String {
        let v = match self.units.angle_unit {
            DimensionUnit::Rad => self.computed_angle_rad(),
            _ => self.computed_angle_deg(),
        };
        format!("{:.prec$}{}", v, self.units.angle_unit.suffix(), prec = self.aspect.decimals as usize)
    }
}

// occt: PrsDim_RadiusDimension
#[derive(Clone, Debug)]
pub struct RadiusDimension {
    pub center: [f64; 3],
    pub radius: f64,
    pub anchor_point: [f64; 3],
    pub aspect: DimensionAspect,
    pub units: DimensionUnits,
    pub is_valid: bool,
}

impl RadiusDimension {
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        let anchor = [center[0] + radius, center[1], center[2]];
        Self {
            center, radius, anchor_point: anchor,
            aspect: DimensionAspect::new(),
            units: DimensionUnits::metric(),
            is_valid: radius > 0.0,
        }
    }

    pub fn display_string(&self) -> String {
        let v = self.radius * self.units.length_unit.factor_from_mm();
        format!("R{:.prec$}{}", v, self.units.length_unit.suffix(), prec = self.aspect.decimals as usize)
    }
}

// occt: PrsDim_DiameterDimension
#[derive(Clone, Debug)]
pub struct DiameterDimension {
    pub center: [f64; 3],
    pub radius: f64,
    pub anchor_point: [f64; 3],
    pub aspect: DimensionAspect,
    pub units: DimensionUnits,
    pub is_valid: bool,
}

impl DiameterDimension {
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        let anchor = [center[0] + radius, center[1], center[2]];
        Self {
            center, radius, anchor_point: anchor,
            aspect: DimensionAspect::new(),
            units: DimensionUnits::metric(),
            is_valid: radius > 0.0,
        }
    }

    pub fn diameter(&self) -> f64 { self.radius * 2.0 }

    pub fn display_string(&self) -> String {
        let v = self.diameter() * self.units.length_unit.factor_from_mm();
        format!("⌀{:.prec$}{}", v, self.units.length_unit.suffix(), prec = self.aspect.decimals as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_dimension() {
        let d = LengthDimension::new([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
        assert!((d.computed_value() - 5.0).abs() < 1e-10);
        let mid = d.midpoint();
        assert!((mid[0] - 1.5).abs() < 1e-10);
    }

    #[test]
    fn length_display_string() {
        let d = LengthDimension::new([0.0, 0.0, 0.0], [10.0, 0.0, 0.0]);
        let s = d.display_string();
        assert!(s.contains("10"));
        assert!(s.contains("mm"));
    }

    #[test]
    fn angle_dimension() {
        let d = AngleDimension::new(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
        );
        assert!((d.computed_angle_deg() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn radius_dimension() {
        let r = RadiusDimension::new([0.0, 0.0, 0.0], 5.0);
        assert!(r.is_valid);
        assert!(r.display_string().starts_with('R'));
    }

    #[test]
    fn diameter_dimension() {
        let d = DiameterDimension::new([0.0, 0.0, 0.0], 5.0);
        assert!((d.diameter() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn unit_factors() {
        assert!((DimensionUnit::Cm.factor_from_mm() - 0.1).abs() < 1e-15);
        assert_eq!(DimensionUnit::Inch.suffix(), "\"");
    }
}
