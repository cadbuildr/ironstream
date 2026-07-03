// FILE: quant_dist.rs
// occt: Quantity_Parameter, Quantity_Length, Quantity_Angle,
//       Quantity_Color, Quantity_ColorRGBA, Quantity_NameOfColor

/// Named OCCT colors (subset of Quantity_NameOfColor).
/// occt: Quantity_NameOfColor
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NameOfColor {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Orange,
    Gray,
    DarkGray,
    LightGray,
    Gold,
    Silver,
    Brown,
    NavyBlue,
    ForestGreen,
    Purple,
    Custom,
}

impl NameOfColor {
    /// Return (r, g, b) in [0,1].
    pub fn to_rgb(self) -> [f64; 3] {
        match self {
            Self::Black     => [0.0, 0.0, 0.0],
            Self::White     => [1.0, 1.0, 1.0],
            Self::Red       => [1.0, 0.0, 0.0],
            Self::Green     => [0.0, 0.5, 0.0],
            Self::Blue      => [0.0, 0.0, 1.0],
            Self::Yellow    => [1.0, 1.0, 0.0],
            Self::Cyan      => [0.0, 1.0, 1.0],
            Self::Magenta   => [1.0, 0.0, 1.0],
            Self::Orange    => [1.0, 0.647, 0.0],
            Self::Gray      => [0.502, 0.502, 0.502],
            Self::DarkGray  => [0.251, 0.251, 0.251],
            Self::LightGray => [0.753, 0.753, 0.753],
            Self::Gold      => [1.0, 0.843, 0.0],
            Self::Silver    => [0.753, 0.753, 0.753],
            Self::Brown     => [0.647, 0.165, 0.165],
            Self::NavyBlue  => [0.0, 0.0, 0.502],
            Self::ForestGreen => [0.133, 0.545, 0.133],
            Self::Purple    => [0.502, 0.0, 0.502],
            Self::Custom    => [0.5, 0.5, 0.5],
        }
    }
}

impl Default for NameOfColor {
    fn default() -> Self { Self::White }
}

/// Color (RGB in [0,1]).
/// occt: Quantity_Color
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuantityColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl QuantityColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r: r.clamp(0.0,1.0), g: g.clamp(0.0,1.0), b: b.clamp(0.0,1.0) }
    }

    pub fn from_name(name: NameOfColor) -> Self {
        let [r,g,b] = name.to_rgb();
        Self { r, g, b }
    }

    pub fn red(&self) -> f64 { self.r }
    pub fn green(&self) -> f64 { self.g }
    pub fn blue(&self) -> f64 { self.b }

    /// Convert to 8-bit values (0-255).
    pub fn to_rgb8(&self) -> [u8; 3] {
        [(self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8]
    }

    /// Euclidean distance between two colors in RGB space.
    pub fn distance(&self, other: &QuantityColor) -> f64 {
        let dr = self.r - other.r;
        let dg = self.g - other.g;
        let db = self.b - other.b;
        (dr*dr + dg*dg + db*db).sqrt()
    }

    /// Convert to hue-saturation-value.
    pub fn to_hsv(&self) -> [f64; 3] {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let v = max;
        let s = if max > 1e-12 { (max - min) / max } else { 0.0 };
        let h = if (max - min) < 1e-12 {
            0.0
        } else if (max - self.r).abs() < 1e-12 {
            60.0 * ((self.g - self.b) / (max - min)).rem_euclid(6.0)
        } else if (max - self.g).abs() < 1e-12 {
            60.0 * ((self.b - self.r) / (max - min) + 2.0)
        } else {
            60.0 * ((self.r - self.g) / (max - min) + 4.0)
        };
        [h, s, v]
    }

    pub fn is_equal(&self, other: &QuantityColor, tol: f64) -> bool {
        self.distance(other) <= tol
    }
}

impl Default for QuantityColor {
    fn default() -> Self { Self::from_name(NameOfColor::White) }
}

/// Color with alpha (RGBA in [0,1]).
/// occt: Quantity_ColorRGBA
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuantityColorRGBA {
    pub rgb: QuantityColor,
    pub alpha: f64,
}

impl QuantityColorRGBA {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { rgb: QuantityColor::new(r, g, b), alpha: a.clamp(0.0, 1.0) }
    }

    pub fn from_color(c: QuantityColor, alpha: f64) -> Self {
        Self { rgb: c, alpha: alpha.clamp(0.0, 1.0) }
    }

    pub fn from_name(name: NameOfColor) -> Self {
        Self { rgb: QuantityColor::from_name(name), alpha: 1.0 }
    }

    pub fn red(&self) -> f64 { self.rgb.r }
    pub fn green(&self) -> f64 { self.rgb.g }
    pub fn blue(&self) -> f64 { self.rgb.b }
    pub fn alpha(&self) -> f64 { self.alpha }
    pub fn rgb(&self) -> QuantityColor { self.rgb }

    pub fn to_rgba8(&self) -> [u8; 4] {
        let [r,g,b] = self.rgb.to_rgb8();
        [r, g, b, (self.alpha * 255.0) as u8]
    }

    pub fn set_alpha(&mut self, a: f64) { self.alpha = a.clamp(0.0, 1.0); }

    pub fn is_opaque(&self) -> bool { self.alpha >= 1.0 - 1e-6 }
    pub fn is_transparent(&self) -> bool { self.alpha < 1e-6 }
}

impl Default for QuantityColorRGBA {
    fn default() -> Self { Self::from_name(NameOfColor::White) }
}

/// Angle conversion helpers.
/// occt: M_PI usage from Standard.hxx
pub struct QuantityAngle;

impl QuantityAngle {
    pub const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
    pub const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;

    pub fn to_radians(deg: f64) -> f64 { deg * Self::DEG_TO_RAD }
    pub fn to_degrees(rad: f64) -> f64 { rad * Self::RAD_TO_DEG }
    pub fn normalize(rad: f64) -> f64 { rad.rem_euclid(2.0 * std::f64::consts::PI) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn name_of_color_rgb() {
        let [r,g,b] = NameOfColor::Red.to_rgb();
        assert!((r-1.0).abs() < 1e-10 && g.abs() < 1e-10 && b.abs() < 1e-10);
        let [r,g,b] = NameOfColor::White.to_rgb();
        assert!((r-1.0).abs() < 1e-10 && (g-1.0).abs() < 1e-10 && (b-1.0).abs() < 1e-10);
    }

    #[test]
    fn quantity_color_ops() {
        let red = QuantityColor::from_name(NameOfColor::Red);
        assert!((red.red() - 1.0).abs() < 1e-10);
        assert!(red.green().abs() < 1e-10);
        let [r8,g8,b8] = red.to_rgb8();
        assert_eq!(r8, 255);
        assert_eq!(g8, 0);
    }

    #[test]
    fn color_distance() {
        let c1 = QuantityColor::new(1.0, 0.0, 0.0);
        let c2 = QuantityColor::new(0.0, 0.0, 0.0);
        assert!((c1.distance(&c2) - 1.0).abs() < 1e-10);
        assert!(c1.is_equal(&c1, 0.0));
        assert!(!c1.is_equal(&c2, 0.5));
    }

    #[test]
    fn color_rgba_alpha() {
        let mut c = QuantityColorRGBA::new(1.0, 0.0, 0.0, 0.5);
        assert!((c.alpha() - 0.5).abs() < 1e-12);
        assert!(!c.is_opaque());
        assert!(!c.is_transparent());
        c.set_alpha(1.0);
        assert!(c.is_opaque());
        let [r,g,b,a] = c.to_rgba8();
        assert_eq!(r, 255);
        assert_eq!(a, 255);
    }

    #[test]
    fn angle_conversion() {
        assert!((QuantityAngle::to_radians(180.0) - PI).abs() < 1e-10);
        assert!((QuantityAngle::to_degrees(PI) - 180.0).abs() < 1e-10);
        let n = QuantityAngle::normalize(3.0 * PI);
        assert!((n - PI).abs() < 1e-10);
    }
}
