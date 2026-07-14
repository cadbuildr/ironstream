// FILE: quantity_ext.rs
// occt-ref: Quantity_Length, Quantity_Angle, Quantity_Speed
//       Quantity_ColorRGBA, Quantity_ColorRGBaf

/// Represents a physical length value (in mm by default).
// occt-ref: Quantity_Length
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct QuantityLength(pub f64);

impl QuantityLength {
    pub fn from_mm(v: f64) -> Self { Self(v) }
    pub fn from_m(v: f64) -> Self { Self(v * 1000.0) }
    pub fn from_in(v: f64) -> Self { Self(v * 25.4) }
    pub fn from_ft(v: f64) -> Self { Self(v * 304.8) }

    pub fn mm(&self) -> f64 { self.0 }
    pub fn m(&self) -> f64 { self.0 / 1000.0 }
    pub fn inch(&self) -> f64 { self.0 / 25.4 }
    pub fn ft(&self) -> f64 { self.0 / 304.8 }
}

/// Represents an angle value (in radians internally).
// occt-ref: Quantity_Angle
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct QuantityAngle(pub f64);

impl QuantityAngle {
    pub fn from_radians(v: f64) -> Self { Self(v) }
    pub fn from_degrees(v: f64) -> Self { Self(v.to_radians()) }

    pub fn radians(&self) -> f64 { self.0 }
    pub fn degrees(&self) -> f64 { self.0.to_degrees() }

    pub fn normalize(&self) -> Self {
        use std::f64::consts::TAU;
        let a = self.0 % TAU;
        Self(if a < 0.0 { a + TAU } else { a })
    }
}

/// Represents a speed value (in mm/s).
// occt-ref: Quantity_Speed
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct QuantitySpeed(pub f64);

impl QuantitySpeed {
    pub fn from_mm_per_s(v: f64) -> Self { Self(v) }
    pub fn from_m_per_s(v: f64) -> Self { Self(v * 1000.0) }
    pub fn from_km_per_h(v: f64) -> Self { Self(v * 1000.0 / 3.6) }

    pub fn mm_per_s(&self) -> f64 { self.0 }
    pub fn m_per_s(&self) -> f64 { self.0 / 1000.0 }
    pub fn km_per_h(&self) -> f64 { self.0 * 3.6 / 1000.0 }
}

/// RGBA color with f64 components.
// occt-ref: Quantity_ColorRGBA
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuantityColorRgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Default for QuantityColorRgba {
    fn default() -> Self { Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 } }
}

impl QuantityColorRgba {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r: r.clamp(0.0,1.0), g: g.clamp(0.0,1.0), b: b.clamp(0.0,1.0), a: a.clamp(0.0,1.0) }
    }

    pub fn rgb(r: f64, g: f64, b: f64) -> Self { Self::new(r, g, b, 1.0) }

    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 24) & 0xFF) as f64 / 255.0;
        let g = ((hex >> 16) & 0xFF) as f64 / 255.0;
        let b = ((hex >> 8)  & 0xFF) as f64 / 255.0;
        let a = ((hex)       & 0xFF) as f64 / 255.0;
        Self { r, g, b, a }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r * 255.0) as u32) << 24 |
        ((self.g * 255.0) as u32) << 16 |
        ((self.b * 255.0) as u32) << 8  |
        ((self.a * 255.0) as u32)
    }

    pub fn is_opaque(&self) -> bool { self.a >= 1.0 - 1e-6 }
    pub fn alpha(&self) -> f64 { self.a }
    pub fn blend_alpha(&self, bg: QuantityColorRgba) -> QuantityColorRgba {
        let a = self.a;
        QuantityColorRgba {
            r: self.r * a + bg.r * (1.0 - a),
            g: self.g * a + bg.g * (1.0 - a),
            b: self.b * a + bg.b * (1.0 - a),
            a: 1.0,
        }
    }
}

/// RGBA color with f32 components (GPU-ready).
// occt-ref: Quantity_ColorRGBaf
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QuantityColorRgbaf {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl QuantityColorRgbaf {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r: r.clamp(0.0,1.0), g: g.clamp(0.0,1.0), b: b.clamp(0.0,1.0), a: a.clamp(0.0,1.0) }
    }

    pub fn from_f64(c: QuantityColorRgba) -> Self {
        Self { r: c.r as f32, g: c.g as f32, b: c.b as f32, a: c.a as f32 }
    }

    pub fn to_array(&self) -> [f32; 4] { [self.r, self.g, self.b, self.a] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_conversions() {
        let l = QuantityLength::from_m(1.0);
        assert!((l.mm() - 1000.0).abs() < 1e-10);
        assert!((l.inch() - 39.37).abs() < 0.01);
    }

    #[test]
    fn angle_conversions() {
        let a = QuantityAngle::from_degrees(180.0);
        assert!((a.radians() - std::f64::consts::PI).abs() < 1e-10);
        let a2 = QuantityAngle::from_radians(std::f64::consts::PI / 2.0);
        assert!((a2.degrees() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn angle_normalize() {
        let a = QuantityAngle::from_degrees(-90.0).normalize();
        assert!((a.degrees() - 270.0).abs() < 1e-8);
    }

    #[test]
    fn speed_conversions() {
        let s = QuantitySpeed::from_km_per_h(36.0);
        assert!((s.m_per_s() - 10.0).abs() < 1e-6);
    }

    #[test]
    fn color_rgba_hex() {
        let c = QuantityColorRgba::from_hex(0xFF0000FF);
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.g).abs() < 0.01);
        assert!(c.is_opaque());
    }

    #[test]
    fn color_rgba_blend() {
        let fg = QuantityColorRgba::new(1.0, 0.0, 0.0, 0.5);
        let bg = QuantityColorRgba::rgb(0.0, 0.0, 1.0);
        let blended = fg.blend_alpha(bg);
        assert!((blended.r - 0.5).abs() < 1e-10);
        assert!((blended.b - 0.5).abs() < 1e-10);
        assert!(blended.is_opaque());
    }
}
