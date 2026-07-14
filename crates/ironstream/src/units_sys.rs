// FILE: units_sys.rs
// occt: Units_UnitSentence, Units_MathSentence
// occt-ref: UnitsAPI, Units_Dimensions
//       Units_Lexicon, Units_Token

/// Dimension type of a physical quantity.
// occt-ref: Units_Dimensions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DimensionKind {
    Dimensionless,
    Length,
    Mass,
    Time,
    ElectricCurrent,
    Temperature,
    LuminousIntensity,
    Angle,
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
    Torque,
}

impl Default for DimensionKind {
    fn default() -> Self { Self::Dimensionless }
}

/// A unit with its SI conversion factor.
#[derive(Clone, Debug)]
pub struct Unit {
    pub name: String,
    pub symbol: String,
    pub dimension: DimensionKind,
    pub to_si: f64,   // multiply by to_si to get SI base unit
    pub from_si: f64, // multiply SI value by from_si to get this unit
}

impl Unit {
    pub fn new(name: impl Into<String>, symbol: impl Into<String>, dim: DimensionKind, to_si: f64) -> Self {
        Self {
            name: name.into(),
            symbol: symbol.into(),
            dimension: dim,
            to_si,
            from_si: if to_si.abs() > 1e-30 { 1.0 / to_si } else { 0.0 },
        }
    }
}

/// Registry of units.
// occt-ref: UnitsAPI // / Units_UnitSentence
#[derive(Clone, Debug)]
pub struct UnitsRegistry {
    units: Vec<Unit>,
}

impl UnitsRegistry {
    pub fn new() -> Self {
        let mut r = Self { units: Vec::new() };
        r.init_defaults();
        r
    }

    fn init_defaults(&mut self) {
        // Length
        self.units.push(Unit::new("meter", "m", DimensionKind::Length, 1.0));
        self.units.push(Unit::new("millimeter", "mm", DimensionKind::Length, 1e-3));
        self.units.push(Unit::new("centimeter", "cm", DimensionKind::Length, 1e-2));
        self.units.push(Unit::new("inch", "in", DimensionKind::Length, 0.0254));
        self.units.push(Unit::new("foot", "ft", DimensionKind::Length, 0.3048));
        // Mass
        self.units.push(Unit::new("kilogram", "kg", DimensionKind::Mass, 1.0));
        self.units.push(Unit::new("gram", "g", DimensionKind::Mass, 1e-3));
        self.units.push(Unit::new("pound", "lb", DimensionKind::Mass, 0.453592));
        // Time
        self.units.push(Unit::new("second", "s", DimensionKind::Time, 1.0));
        self.units.push(Unit::new("minute", "min", DimensionKind::Time, 60.0));
        self.units.push(Unit::new("hour", "h", DimensionKind::Time, 3600.0));
        // Angle
        self.units.push(Unit::new("radian", "rad", DimensionKind::Angle, 1.0));
        self.units.push(Unit::new("degree", "deg", DimensionKind::Angle, std::f64::consts::PI / 180.0));
        // Temperature
        self.units.push(Unit::new("kelvin", "K", DimensionKind::Temperature, 1.0));
        self.units.push(Unit::new("celsius", "°C", DimensionKind::Temperature, 1.0));
        // Area
        self.units.push(Unit::new("square meter", "m2", DimensionKind::Area, 1.0));
        self.units.push(Unit::new("square millimeter", "mm2", DimensionKind::Area, 1e-6));
        // Volume
        self.units.push(Unit::new("cubic meter", "m3", DimensionKind::Volume, 1.0));
        self.units.push(Unit::new("liter", "L", DimensionKind::Volume, 1e-3));
        // Force
        self.units.push(Unit::new("newton", "N", DimensionKind::Force, 1.0));
        // Pressure
        self.units.push(Unit::new("pascal", "Pa", DimensionKind::Pressure, 1.0));
        self.units.push(Unit::new("megapascal", "MPa", DimensionKind::Pressure, 1e6));
    }

    pub fn find_by_symbol(&self, sym: &str) -> Option<&Unit> {
        self.units.iter().find(|u| u.symbol == sym)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Unit> {
        self.units.iter().find(|u| u.name.eq_ignore_ascii_case(name))
    }

    /// Convert a value from unit `from` to unit `to` (both given by symbol).
    /// Returns None if either unit is unknown or dimensions don't match.
    pub fn convert(&self, value: f64, from: &str, to: &str) -> Option<f64> {
        let u_from = self.find_by_symbol(from)?;
        let u_to = self.find_by_symbol(to)?;
        if u_from.dimension != u_to.dimension { return None; }
        Some(value * u_from.to_si * u_to.from_si)
    }

    pub fn nb_units(&self) -> usize { self.units.len() }

    pub fn units(&self) -> &[Unit] { &self.units }
}

impl Default for UnitsRegistry {
    fn default() -> Self { Self::new() }
}

/// OCCT units API (static access).
// occt-ref: UnitsAPI
pub struct UnitsApi;

impl UnitsApi {
    /// Length unit active in OCCT session (stub: always returns 1.0 = mm/mm).
    pub fn any_to_ls(val: f64, unit: &str) -> f64 {
        let r = UnitsRegistry::new();
        r.convert(val, unit, "m").unwrap_or(val)
    }

    pub fn ls_to_any(val: f64, unit: &str) -> f64 {
        let r = UnitsRegistry::new();
        r.convert(val, "m", unit).unwrap_or(val)
    }
}

/// A simple token in a unit expression.
/// occt: Units_Token
#[derive(Clone, Debug)]
pub struct UnitsToken {
    pub word: String,
    pub mean: String,
    pub value: f64,
    pub dimension: DimensionKind,
}

impl UnitsToken {
    pub fn new(word: impl Into<String>, value: f64, dim: DimensionKind) -> Self {
        let word = word.into();
        let mean = word.clone();
        Self { word, mean, value, dimension: dim }
    }

    pub fn word(&self) -> &str { &self.word }
    pub fn value(&self) -> f64 { self.value }
    pub fn dimension(&self) -> DimensionKind { self.dimension }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_find() {
        let r = UnitsRegistry::new();
        let mm = r.find_by_symbol("mm").unwrap();
        assert_eq!(mm.dimension, DimensionKind::Length);
        assert!((mm.to_si - 1e-3).abs() < 1e-15);
    }

    #[test]
    fn registry_convert_same_dim() {
        let r = UnitsRegistry::new();
        let v = r.convert(1000.0, "mm", "m").unwrap();
        assert!((v - 1.0).abs() < 1e-10);
        let v2 = r.convert(1.0, "m", "mm").unwrap();
        assert!((v2 - 1000.0).abs() < 1e-7);
    }

    #[test]
    fn registry_convert_wrong_dim() {
        let r = UnitsRegistry::new();
        assert!(r.convert(1.0, "m", "kg").is_none());
    }

    #[test]
    fn registry_convert_angle() {
        let r = UnitsRegistry::new();
        use std::f64::consts::PI;
        let rad = r.convert(180.0, "deg", "rad").unwrap();
        assert!((rad - PI).abs() < 1e-10);
    }

    #[test]
    fn units_api_ls() {
        let v = UnitsApi::any_to_ls(1000.0, "mm");
        assert!((v - 1.0).abs() < 1e-9);
    }

    #[test]
    fn units_token_basic() {
        let t = UnitsToken::new("m", 1.0, DimensionKind::Length);
        assert_eq!(t.word(), "m");
        assert!((t.value() - 1.0).abs() < 1e-12);
        assert_eq!(t.dimension(), DimensionKind::Length);
    }
}
