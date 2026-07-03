// FILE: quant_units.rs
// occt: Quantity_Unit, UnitsAPI, Units_Dimensions, Units_UnitsDictionary

/// Kind of physical quantity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuantityKind {
    Length,
    Mass,
    Time,
    Temperature,
    ElectricCurrent,
    AmountOfSubstance,
    LuminousIntensity,
    PlaneAngle,
    SolidAngle,
    Force,
    Pressure,
    Energy,
    Power,
    Velocity,
    Acceleration,
    Area,
    Volume,
    Frequency,
    ElectricCharge,
    Voltage,
    Unknown,
}

impl Default for QuantityKind {
    fn default() -> Self { Self::Unknown }
}

impl QuantityKind {
    pub fn si_unit_symbol(&self) -> &'static str {
        match self {
            Self::Length => "m",
            Self::Mass => "kg",
            Self::Time => "s",
            Self::Temperature => "K",
            Self::ElectricCurrent => "A",
            Self::AmountOfSubstance => "mol",
            Self::LuminousIntensity => "cd",
            Self::PlaneAngle => "rad",
            Self::SolidAngle => "sr",
            Self::Force => "N",
            Self::Pressure => "Pa",
            Self::Energy => "J",
            Self::Power => "W",
            Self::Velocity => "m/s",
            Self::Acceleration => "m/s²",
            Self::Area => "m²",
            Self::Volume => "m³",
            Self::Frequency => "Hz",
            Self::ElectricCharge => "C",
            Self::Voltage => "V",
            Self::Unknown => "?",
        }
    }

    pub fn is_base(&self) -> bool {
        matches!(self, Self::Length | Self::Mass | Self::Time | Self::Temperature
            | Self::ElectricCurrent | Self::AmountOfSubstance | Self::LuminousIntensity)
    }

    pub fn is_derived(&self) -> bool { !self.is_base() && *self != Self::Unknown }
}

/// A unit definition with name, symbol, kind, and conversion factor to SI.
#[derive(Clone, Debug)]
pub struct UnitDefinition {
    pub name: String,
    pub symbol: String,
    pub kind: QuantityKind,
    pub factor_to_si: f64,
    pub offset_to_si: f64,   // for temperature conversions
}

impl UnitDefinition {
    pub fn new(name: &str, symbol: &str, kind: QuantityKind, factor: f64) -> Self {
        Self { name: name.to_owned(), symbol: symbol.to_owned(), kind, factor_to_si: factor, offset_to_si: 0.0 }
    }

    pub fn with_offset(mut self, offset: f64) -> Self { self.offset_to_si = offset; self }

    pub fn to_si(&self, value: f64) -> f64 { value * self.factor_to_si + self.offset_to_si }
    pub fn from_si(&self, si_value: f64) -> f64 { (si_value - self.offset_to_si) / self.factor_to_si }
    pub fn convert_to(&self, value: f64, target: &UnitDefinition) -> f64 {
        target.from_si(self.to_si(value))
    }
}

/// Built-in unit catalogue.
pub struct UnitsCatalogue;

impl UnitsCatalogue {
    pub fn millimeter() -> UnitDefinition { UnitDefinition::new("millimeter", "mm", QuantityKind::Length, 1e-3) }
    pub fn centimeter() -> UnitDefinition { UnitDefinition::new("centimeter", "cm", QuantityKind::Length, 1e-2) }
    pub fn meter() -> UnitDefinition { UnitDefinition::new("meter", "m", QuantityKind::Length, 1.0) }
    pub fn inch() -> UnitDefinition { UnitDefinition::new("inch", "in", QuantityKind::Length, 0.0254) }
    pub fn foot() -> UnitDefinition { UnitDefinition::new("foot", "ft", QuantityKind::Length, 0.3048) }

    pub fn gram() -> UnitDefinition { UnitDefinition::new("gram", "g", QuantityKind::Mass, 1e-3) }
    pub fn kilogram() -> UnitDefinition { UnitDefinition::new("kilogram", "kg", QuantityKind::Mass, 1.0) }
    pub fn pound() -> UnitDefinition { UnitDefinition::new("pound", "lb", QuantityKind::Mass, 0.453_592_37) }

    pub fn second() -> UnitDefinition { UnitDefinition::new("second", "s", QuantityKind::Time, 1.0) }
    pub fn minute() -> UnitDefinition { UnitDefinition::new("minute", "min", QuantityKind::Time, 60.0) }
    pub fn hour() -> UnitDefinition { UnitDefinition::new("hour", "h", QuantityKind::Time, 3600.0) }

    pub fn kelvin() -> UnitDefinition { UnitDefinition::new("kelvin", "K", QuantityKind::Temperature, 1.0) }
    pub fn celsius() -> UnitDefinition {
        UnitDefinition::new("celsius", "°C", QuantityKind::Temperature, 1.0).with_offset(273.15)
    }
    pub fn fahrenheit() -> UnitDefinition {
        UnitDefinition::new("fahrenheit", "°F", QuantityKind::Temperature, 5.0/9.0).with_offset(255.372_222)
    }

    pub fn degree() -> UnitDefinition {
        UnitDefinition::new("degree", "°", QuantityKind::PlaneAngle, std::f64::consts::PI / 180.0)
    }
    pub fn radian() -> UnitDefinition { UnitDefinition::new("radian", "rad", QuantityKind::PlaneAngle, 1.0) }

    pub fn newton() -> UnitDefinition { UnitDefinition::new("newton", "N", QuantityKind::Force, 1.0) }
    pub fn pascal() -> UnitDefinition { UnitDefinition::new("pascal", "Pa", QuantityKind::Pressure, 1.0) }
    pub fn megapascal() -> UnitDefinition { UnitDefinition::new("megapascal", "MPa", QuantityKind::Pressure, 1e6) }
    pub fn joule() -> UnitDefinition { UnitDefinition::new("joule", "J", QuantityKind::Energy, 1.0) }
}

// occt: UnitsAPI — global conversion utilities
#[derive(Clone, Debug, Default)]
pub struct UnitsApi {
    pub current_length_unit: String,
    pub current_angle_unit: String,
}

impl UnitsApi {
    pub fn new() -> Self {
        Self {
            current_length_unit: "mm".to_owned(),
            current_angle_unit: "deg".to_owned(),
        }
    }

    pub fn any_to_si(kind: QuantityKind, value: f64, from_unit: &UnitDefinition) -> f64 {
        assert_eq!(from_unit.kind, kind);
        from_unit.to_si(value)
    }

    pub fn si_to_any(kind: QuantityKind, si_value: f64, to_unit: &UnitDefinition) -> f64 {
        assert_eq!(to_unit.kind, kind);
        to_unit.from_si(si_value)
    }

    pub fn convert(value: f64, from: &UnitDefinition, to: &UnitDefinition) -> f64 {
        from.convert_to(value, to)
    }

    pub fn factor(from: &UnitDefinition, to: &UnitDefinition) -> f64 {
        from.factor_to_si / to.factor_to_si
    }

    pub fn set_local_system_length(&mut self, unit: &str) {
        self.current_length_unit = unit.to_owned();
    }

    pub fn current_to_si_length(&self, value: f64) -> f64 {
        let factor = match self.current_length_unit.as_str() {
            "mm" => 1e-3,
            "cm" => 1e-2,
            "m"  => 1.0,
            "in" | "inch" => 0.0254,
            "ft" | "foot" => 0.3048,
            _ => 1.0,
        };
        value * factor
    }
}

// occt: Units_Dimensions — physical dimension record (exponents in M·L·T·I·Θ·N·J)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UnitsDimensions {
    pub mass: i8,
    pub length: i8,
    pub time: i8,
    pub electric_current: i8,
    pub temperature: i8,
    pub amount_of_substance: i8,
    pub luminous_intensity: i8,
}

impl UnitsDimensions {
    pub fn dimensionless() -> Self { Self::default() }

    pub fn for_kind(kind: QuantityKind) -> Self {
        match kind {
            QuantityKind::Length      => Self { length: 1, ..Default::default() },
            QuantityKind::Mass        => Self { mass: 1, ..Default::default() },
            QuantityKind::Time        => Self { time: 1, ..Default::default() },
            QuantityKind::Temperature => Self { temperature: 1, ..Default::default() },
            QuantityKind::Force       => Self { mass: 1, length: 1, time: -2, ..Default::default() },
            QuantityKind::Pressure    => Self { mass: 1, length: -1, time: -2, ..Default::default() },
            QuantityKind::Energy      => Self { mass: 1, length: 2, time: -2, ..Default::default() },
            QuantityKind::Power       => Self { mass: 1, length: 2, time: -3, ..Default::default() },
            QuantityKind::Velocity    => Self { length: 1, time: -1, ..Default::default() },
            QuantityKind::Area        => Self { length: 2, ..Default::default() },
            QuantityKind::Volume      => Self { length: 3, ..Default::default() },
            _ => Self::default(),
        }
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            mass:                  self.mass + other.mass,
            length:                self.length + other.length,
            time:                  self.time + other.time,
            electric_current:      self.electric_current + other.electric_current,
            temperature:           self.temperature + other.temperature,
            amount_of_substance:   self.amount_of_substance + other.amount_of_substance,
            luminous_intensity:    self.luminous_intensity + other.luminous_intensity,
        }
    }

    pub fn invert(&self) -> Self {
        Self {
            mass:                  -self.mass,
            length:                -self.length,
            time:                  -self.time,
            electric_current:      -self.electric_current,
            temperature:           -self.temperature,
            amount_of_substance:   -self.amount_of_substance,
            luminous_intensity:    -self.luminous_intensity,
        }
    }

    pub fn is_dimensionless(&self) -> bool { *self == Self::dimensionless() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_kind_si_symbols() {
        assert_eq!(QuantityKind::Length.si_unit_symbol(), "m");
        assert_eq!(QuantityKind::Mass.si_unit_symbol(), "kg");
        assert!(QuantityKind::Length.is_base());
        assert!(QuantityKind::Force.is_derived());
    }

    #[test]
    fn unit_mm_to_m() {
        let mm = UnitsCatalogue::millimeter();
        assert!((mm.to_si(1000.0) - 1.0).abs() < 1e-10);
        assert!((mm.from_si(1.0) - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn unit_inch_to_mm() {
        let inch = UnitsCatalogue::inch();
        let mm = UnitsCatalogue::millimeter();
        let result = inch.convert_to(1.0, &mm);
        assert!((result - 25.4).abs() < 1e-6);
    }

    #[test]
    fn temperature_celsius_to_kelvin() {
        let c = UnitsCatalogue::celsius();
        let k_val = c.to_si(0.0);
        assert!((k_val - 273.15).abs() < 1e-6);
    }

    #[test]
    fn units_api_convert() {
        let mm = UnitsCatalogue::millimeter();
        let inch = UnitsCatalogue::inch();
        let result = UnitsApi::convert(25.4, &mm, &inch);
        assert!((result - 1.0).abs() < 1e-6);
    }

    #[test]
    fn units_api_local_length() {
        let mut api = UnitsApi::new();
        api.set_local_system_length("mm");
        let si = api.current_to_si_length(1000.0);
        assert!((si - 1.0).abs() < 1e-10);
    }

    #[test]
    fn units_dimensions_force() {
        let f = UnitsDimensions::for_kind(QuantityKind::Force);
        assert_eq!(f.mass, 1);
        assert_eq!(f.length, 1);
        assert_eq!(f.time, -2);
    }

    #[test]
    fn units_dimensions_multiply() {
        let l = UnitsDimensions::for_kind(QuantityKind::Length);
        let area = l.multiply(&l);
        assert_eq!(area.length, 2);
    }

    #[test]
    fn degree_to_radian() {
        let deg = UnitsCatalogue::degree();
        let rad = UnitsCatalogue::radian();
        let result = deg.convert_to(180.0, &rad);
        assert!((result - std::f64::consts::PI).abs() < 1e-10);
    }
}
