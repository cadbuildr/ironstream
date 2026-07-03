// FILE: quant_extra.rs
// occt: Quantity_NameOfColor, Quantity_PhysicalQuantity,
//       Quantity_Factor, Quantity_Period

/// Named OCCT colors (subset of Quantity_NameOfColor).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NameOfColor {
    Black,
    White,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    Orange,
    Pink,
    Brown,
    Silver,
    Gold,
    Gray,
    DarkGray,
    LightGray,
    DarkRed,
    DarkGreen,
    DarkBlue,
    LightYellow,
    LightGreen,
    LightBlue,
    Coral,
    Tomato,
    SteelBlue,
    Custom([u8; 3]),
}

impl NameOfColor {
    pub fn rgb_u8(&self) -> [u8; 3] {
        match self {
            Self::Black      => [0, 0, 0],
            Self::White      => [255, 255, 255],
            Self::Red        => [255, 0, 0],
            Self::Green      => [0, 255, 0],
            Self::Blue       => [0, 0, 255],
            Self::Cyan       => [0, 255, 255],
            Self::Magenta    => [255, 0, 255],
            Self::Yellow     => [255, 255, 0],
            Self::Orange     => [255, 165, 0],
            Self::Pink       => [255, 192, 203],
            Self::Brown      => [165, 42, 42],
            Self::Silver     => [192, 192, 192],
            Self::Gold       => [255, 215, 0],
            Self::Gray       => [128, 128, 128],
            Self::DarkGray   => [64, 64, 64],
            Self::LightGray  => [211, 211, 211],
            Self::DarkRed    => [139, 0, 0],
            Self::DarkGreen  => [0, 100, 0],
            Self::DarkBlue   => [0, 0, 139],
            Self::LightYellow => [255, 255, 224],
            Self::LightGreen  => [144, 238, 144],
            Self::LightBlue   => [173, 216, 230],
            Self::Coral       => [255, 127, 80],
            Self::Tomato      => [255, 99, 71],
            Self::SteelBlue   => [70, 130, 180],
            Self::Custom(rgb) => *rgb,
        }
    }

    pub fn rgb_f32(&self) -> [f32; 3] {
        let [r, g, b] = self.rgb_u8();
        [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
    }

    pub fn luminance(&self) -> f32 {
        let [r, g, b] = self.rgb_f32();
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    pub fn is_dark(&self) -> bool { self.luminance() < 0.5 }
}

impl Default for NameOfColor {
    fn default() -> Self { Self::White }
}

/// Enumeration of physical quantity categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhysicalQuantity {
    Length,
    Mass,
    Time,
    Temperature,
    ElectricCurrent,
    LuminousIntensity,
    PlaneAngle,
    SolidAngle,
    Area,
    Volume,
    Velocity,
    Acceleration,
    Force,
    Pressure,
    Energy,
    Power,
    Frequency,
    Density,
}

impl PhysicalQuantity {
    pub fn si_unit_symbol(&self) -> &'static str {
        match self {
            Self::Length          => "m",
            Self::Mass            => "kg",
            Self::Time            => "s",
            Self::Temperature     => "K",
            Self::ElectricCurrent => "A",
            Self::LuminousIntensity => "cd",
            Self::PlaneAngle      => "rad",
            Self::SolidAngle      => "sr",
            Self::Area            => "m²",
            Self::Volume          => "m³",
            Self::Velocity        => "m/s",
            Self::Acceleration    => "m/s²",
            Self::Force           => "N",
            Self::Pressure        => "Pa",
            Self::Energy          => "J",
            Self::Power           => "W",
            Self::Frequency       => "Hz",
            Self::Density         => "kg/m³",
        }
    }

    pub fn is_derived(&self) -> bool {
        matches!(self,
            Self::Area | Self::Volume | Self::Velocity | Self::Acceleration
            | Self::Force | Self::Pressure | Self::Energy | Self::Power
            | Self::Frequency | Self::Density
        )
    }

    pub fn is_base(&self) -> bool { !self.is_derived() }
}

impl Default for PhysicalQuantity {
    fn default() -> Self { Self::Length }
}

/// A unit conversion factor entry.
#[derive(Clone, Debug)]
pub struct QuantityFactor {
    pub from_unit: String,
    pub to_unit: String,
    pub factor: f64,
    pub offset: f64,   // for temperature conversions
}

impl QuantityFactor {
    pub fn new(from_unit: &str, to_unit: &str, factor: f64) -> Self {
        Self { from_unit: from_unit.to_owned(), to_unit: to_unit.to_owned(), factor, offset: 0.0 }
    }

    pub fn with_offset(mut self, offset: f64) -> Self { self.offset = offset; self }

    pub fn convert(&self, value: f64) -> f64 { value * self.factor + self.offset }
    pub fn inverse_convert(&self, value: f64) -> f64 { (value - self.offset) / self.factor }
}

/// Built-in unit conversion catalog.
pub struct QuantitySystem;

impl QuantitySystem {
    pub fn mm_to_m() -> QuantityFactor { QuantityFactor::new("mm", "m", 1e-3) }
    pub fn cm_to_m() -> QuantityFactor { QuantityFactor::new("cm", "m", 1e-2) }
    pub fn inch_to_m() -> QuantityFactor { QuantityFactor::new("in", "m", 0.0254) }
    pub fn foot_to_m() -> QuantityFactor { QuantityFactor::new("ft", "m", 0.3048) }
    pub fn deg_to_rad() -> QuantityFactor { QuantityFactor::new("deg", "rad", std::f64::consts::PI / 180.0) }
    pub fn celsius_to_kelvin() -> QuantityFactor { QuantityFactor::new("°C", "K", 1.0).with_offset(273.15) }
    pub fn fahrenheit_to_kelvin() -> QuantityFactor { QuantityFactor::new("°F", "K", 5.0/9.0).with_offset(273.15 - 32.0 * 5.0/9.0) }
}

// occt: Quantity_Period — time duration
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Period {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: f64,
}

impl Period {
    pub fn new(h: u32, m: u32, s: f64) -> Self { Self { hours: h, minutes: m, seconds: s } }
    pub fn from_seconds(total_s: f64) -> Self {
        let total_int = total_s as u64;
        let h = (total_int / 3600) as u32;
        let m = ((total_int % 3600) / 60) as u32;
        let s = total_s - (h as f64 * 3600.0 + m as f64 * 60.0);
        Self { hours: h, minutes: m, seconds: s }
    }

    pub fn to_seconds(&self) -> f64 {
        self.hours as f64 * 3600.0 + self.minutes as f64 * 60.0 + self.seconds
    }

    pub fn add(&self, other: &Period) -> Period {
        Period::from_seconds(self.to_seconds() + other.to_seconds())
    }

    pub fn subtract(&self, other: &Period) -> Period {
        let s = (self.to_seconds() - other.to_seconds()).max(0.0);
        Period::from_seconds(s)
    }

    pub fn is_equal(&self, other: &Period, tol: f64) -> bool {
        (self.to_seconds() - other.to_seconds()).abs() <= tol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_colors() {
        let r = NameOfColor::Red.rgb_u8();
        assert_eq!(r, [255, 0, 0]);
        assert!(!NameOfColor::White.is_dark());
        assert!(NameOfColor::Black.is_dark());
    }

    #[test]
    fn color_luminance() {
        assert!((NameOfColor::White.luminance() - 1.0).abs() < 1e-4);
        assert!(NameOfColor::Black.luminance().abs() < 1e-6);
    }

    #[test]
    fn physical_quantity() {
        assert!(PhysicalQuantity::Force.is_derived());
        assert!(PhysicalQuantity::Mass.is_base());
        assert_eq!(PhysicalQuantity::Length.si_unit_symbol(), "m");
    }

    #[test]
    fn unit_conversion() {
        let f = QuantitySystem::mm_to_m();
        assert!((f.convert(1000.0) - 1.0).abs() < 1e-10);
        let r = QuantitySystem::deg_to_rad();
        assert!((r.convert(180.0) - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn period() {
        let p = Period::from_seconds(3661.5);
        assert_eq!(p.hours, 1);
        assert_eq!(p.minutes, 1);
        assert!((p.seconds - 1.5).abs() < 1e-6);
        let p2 = Period::new(0, 30, 0.0);
        let sum = p.add(&p2);
        assert!((sum.to_seconds() - p.to_seconds() - 1800.0).abs() < 1e-6);
    }
}
