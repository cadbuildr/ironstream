// FILE: units_methods_length_unit.rs
// occt: UnitsMethods_LengthUnit

/// Length unit enumeration and conversion.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LengthUnit {
    Mm,
    Cm,
    M,
    Km,
    Inch,
    Foot,
    Mile,
}

impl LengthUnit {
    pub fn to_mm(&self, value: f64) -> f64 {
        match self {
            Self::Mm => value,
            Self::Cm => value * 10.0,
            Self::M => value * 1000.0,
            Self::Km => value * 1_000_000.0,
            Self::Inch => value * 25.4,
            Self::Foot => value * 304.8,
            Self::Mile => value * 1_609_344.0,
        }
    }

    pub fn from_mm(&self, mm_value: f64) -> f64 {
        match self {
            Self::Mm => mm_value,
            Self::Cm => mm_value / 10.0,
            Self::M => mm_value / 1000.0,
            Self::Km => mm_value / 1_000_000.0,
            Self::Inch => mm_value / 25.4,
            Self::Foot => mm_value / 304.8,
            Self::Mile => mm_value / 1_609_344.0,
        }
    }
}
