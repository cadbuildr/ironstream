// FILE: step_basic_si_unit_name.rs
// occt: StepBasic_SiUnitName

/// Enumeration representing SI unit names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepBasicSiUnitName {
    Metre,
    Gram,
    Second,
    Ampere,
    Kelvin,
    Mole,
    Candela,
    Radian,
    Steradian,
    Hertz,
    Newton,
    Pascal,
    Joule,
    Watt,
    Coulomb,
    Volt,
    Farad,
    Ohm,
    Siemens,
    Weber,
    Tesla,
    Henry,
    DegreeCelsius,
    Lumen,
    Lux,
    Becquerel,
    Gray,
    Sievert,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_si_unit_names() {
        let _ = StepBasicSiUnitName::Metre;
        let _ = StepBasicSiUnitName::Gram;
        let _ = StepBasicSiUnitName::Newton;
        let _ = StepBasicSiUnitName::Joule;
        let _ = StepBasicSiUnitName::Kelvin;
    }
}
