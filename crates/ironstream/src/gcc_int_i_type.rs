// FILE: gcc_int_i_type.rs
// occt: GccInt_IType

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GccIntIType {
    Lin,
    Cir,
    Ell,
    Par,
    Hpr,
    Pnt,
}

impl GccIntIType {
    pub fn to_string(&self) -> &'static str {
        match self {
            GccIntIType::Lin => "Lin",
            GccIntIType::Cir => "Cir",
            GccIntIType::Ell => "Ell",
            GccIntIType::Par => "Par",
            GccIntIType::Hpr => "Hpr",
            GccIntIType::Pnt => "Pnt",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_variants() {
        assert_eq!(GccIntIType::Lin.to_string(), "Lin");
        assert_eq!(GccIntIType::Cir.to_string(), "Cir");
        assert_eq!(GccIntIType::Ell.to_string(), "Ell");
        assert_eq!(GccIntIType::Par.to_string(), "Par");
        assert_eq!(GccIntIType::Hpr.to_string(), "Hpr");
        assert_eq!(GccIntIType::Pnt.to_string(), "Pnt");
    }

    #[test]
    fn test_enum_equality() {
        assert_eq!(GccIntIType::Lin, GccIntIType::Lin);
        assert_ne!(GccIntIType::Lin, GccIntIType::Cir);
    }
}
