// FILE: step_basic_si_prefix.rs
// occt: StepBasic_SiPrefix

/// Enumeration representing SI prefixes for units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepBasicSiPrefix {
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
    Hecto,
    Deca,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_si_prefixes() {
        let _ = StepBasicSiPrefix::Exa;
        let _ = StepBasicSiPrefix::Kilo;
        let _ = StepBasicSiPrefix::Milli;
        let _ = StepBasicSiPrefix::Micro;
        let _ = StepBasicSiPrefix::Nano;
    }
}
