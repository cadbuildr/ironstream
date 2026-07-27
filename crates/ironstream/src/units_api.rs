// FILE: units_api.rs

// occt: UnitsAPI_SystemUnits
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitSystem {
    SI,
    MDTVCurrentUnit,
    MDTVMDTVUnit,
}

// occt-note: (dimension kind, not a direct OCCT class but models Units_Dimensions)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitKind {
    Length,
    Angle,
    Mass,
    Time,
    Force,
    Pressure,
    Velocity,
    Acceleration,
    Energy,
}

pub struct UnitsConversionFactor {
    pub from_unit: &'static str,
    pub to_unit: &'static str,
    pub factor: f64,
}

// occt: UnitsAPI
pub struct UnitsApi;

impl UnitsApi {
    pub fn to_si(value: f64, unit: &str) -> Option<f64> {
        let factor = match unit {
            "mm"  => 1e-3,
            "cm"  => 1e-2,
            "m"   => 1.0,
            "in"  => 0.0254,
            "ft"  => 0.3048,
            "deg" => std::f64::consts::PI / 180.0,
            "rad" => 1.0,
            "kg"  => 1.0,
            "g"   => 1e-3,
            "lb"  => 0.453_592,
            "N"   => 1.0,
            "kN"  => 1_000.0,
            "Pa"  => 1.0,
            "MPa" => 1e6,
            "GPa" => 1e9,
            _     => return None,
        };
        Some(value * factor)
    }

    pub fn from_si(value: f64, unit: &str) -> Option<f64> {
        let factor = match unit {
            "mm"  => 1e-3,
            "cm"  => 1e-2,
            "m"   => 1.0,
            "in"  => 0.0254,
            "ft"  => 0.3048,
            "deg" => std::f64::consts::PI / 180.0,
            "rad" => 1.0,
            "kg"  => 1.0,
            "g"   => 1e-3,
            "lb"  => 0.453_592,
            "N"   => 1.0,
            "kN"  => 1_000.0,
            "Pa"  => 1.0,
            "MPa" => 1e6,
            "GPa" => 1e9,
            _     => return None,
        };
        Some(value / factor)
    }

    pub fn convert(value: f64, from: &str, to: &str) -> Option<f64> {
        Self::to_si(value, from).and_then(|v| Self::from_si(v, to))
    }

    pub fn unit_kind(unit: &str) -> Option<UnitKind> {
        match unit {
            "mm" | "cm" | "m" | "in" | "ft" => Some(UnitKind::Length),
            "deg" | "rad"                    => Some(UnitKind::Angle),
            "kg" | "g" | "lb"               => Some(UnitKind::Mass),
            "s"                              => Some(UnitKind::Time),
            "N" | "kN"                       => Some(UnitKind::Force),
            "Pa" | "MPa" | "GPa"            => Some(UnitKind::Pressure),
            _                               => None,
        }
    }

    pub fn is_length_unit(unit: &str) -> bool {
        matches!(Self::unit_kind(unit), Some(UnitKind::Length))
    }

    pub fn is_angle_unit(unit: &str) -> bool {
        matches!(Self::unit_kind(unit), Some(UnitKind::Angle))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
    }

    #[test]
    fn to_si_mm() {
        near(UnitsApi::to_si(1.0, "mm").unwrap(), 1e-3, 1e-15);
    }

    #[test]
    fn to_si_cm() {
        near(UnitsApi::to_si(1.0, "cm").unwrap(), 1e-2, 1e-15);
    }

    #[test]
    fn to_si_in() {
        near(UnitsApi::to_si(1.0, "in").unwrap(), 0.0254, 1e-10);
    }

    #[test]
    fn to_si_ft() {
        near(UnitsApi::to_si(1.0, "ft").unwrap(), 0.3048, 1e-10);
    }

    #[test]
    fn to_si_deg() {
        near(UnitsApi::to_si(180.0, "deg").unwrap(), std::f64::consts::PI, 1e-12);
    }

    #[test]
    fn to_si_rad_identity() {
        near(UnitsApi::to_si(1.5, "rad").unwrap(), 1.5, 1e-15);
    }

    #[test]
    fn to_si_unknown_returns_none() {
        assert!(UnitsApi::to_si(1.0, "furlong").is_none());
    }

    #[test]
    fn convert_ft_to_m() {
        near(UnitsApi::convert(1.0, "ft", "m").unwrap(), 0.3048, 1e-10);
    }

    #[test]
    fn convert_deg_to_rad() {
        near(UnitsApi::convert(90.0, "deg", "rad").unwrap(), std::f64::consts::FRAC_PI_2, 1e-12);
    }

    #[test]
    fn from_si_roundtrip_mm() {
        let si = UnitsApi::to_si(25.4, "mm").unwrap();
        let back = UnitsApi::from_si(si, "mm").unwrap();
        near(back, 25.4, 1e-10);
    }

    #[test]
    fn unit_kind_length() {
        assert_eq!(UnitsApi::unit_kind("mm"), Some(UnitKind::Length));
        assert_eq!(UnitsApi::unit_kind("in"), Some(UnitKind::Length));
        assert_eq!(UnitsApi::unit_kind("ft"), Some(UnitKind::Length));
    }

    #[test]
    fn unit_kind_angle() {
        assert_eq!(UnitsApi::unit_kind("deg"), Some(UnitKind::Angle));
        assert_eq!(UnitsApi::unit_kind("rad"), Some(UnitKind::Angle));
    }

    #[test]
    fn unit_kind_pressure() {
        assert_eq!(UnitsApi::unit_kind("MPa"), Some(UnitKind::Pressure));
        assert_eq!(UnitsApi::unit_kind("GPa"), Some(UnitKind::Pressure));
    }

    #[test]
    fn is_length_and_angle_helpers() {
        assert!(UnitsApi::is_length_unit("cm"));
        assert!(!UnitsApi::is_length_unit("deg"));
        assert!(UnitsApi::is_angle_unit("rad"));
        assert!(!UnitsApi::is_angle_unit("kg"));
    }
}
