// FILE: t_naming_name_type.rs
// occt: TNaming_NameType

/// Enumeration to store naming characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TNamingNameType {
    Unknown,
    Identity,
    ModifUntil,
    Generation,
    Intersection,
    Union,
    Substraction,
    ConstShape,
    FilterByNeighbourgs,
    Orientation,
    WireIn,
    ShellIn,
}

impl Default for TNamingNameType {
    fn default() -> Self {
        TNamingNameType::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naming_type_default() {
        assert_eq!(TNamingNameType::default(), TNamingNameType::Unknown);
    }

    #[test]
    fn test_naming_type_variants() {
        let variants = vec![
            TNamingNameType::Unknown,
            TNamingNameType::Identity,
            TNamingNameType::ModifUntil,
            TNamingNameType::Generation,
            TNamingNameType::Intersection,
            TNamingNameType::Union,
            TNamingNameType::Substraction,
            TNamingNameType::ConstShape,
            TNamingNameType::FilterByNeighbourgs,
            TNamingNameType::Orientation,
            TNamingNameType::WireIn,
            TNamingNameType::ShellIn,
        ];
        assert_eq!(variants.len(), 12);
    }

    #[test]
    fn test_naming_type_clone() {
        let t = TNamingNameType::Union;
        let t_cloned = t.clone();
        assert_eq!(t, t_cloned);
    }
}
