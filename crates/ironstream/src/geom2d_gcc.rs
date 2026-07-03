// FILE: geom2d_gcc.rs
// occt: Geom2dGcc

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Geom2dGccQualifier {
    Unqualified,
    Enclosing,
    Enclosed,
    Outside,
}

/// Utilities for Geom2dGcc qualified curves.
pub struct Geom2dGcc;

impl Geom2dGcc {
    pub fn unqualified() -> Geom2dGccQualifier {
        Geom2dGccQualifier::Unqualified
    }

    pub fn enclosing() -> Geom2dGccQualifier {
        Geom2dGccQualifier::Enclosing
    }

    pub fn enclosed() -> Geom2dGccQualifier {
        Geom2dGccQualifier::Enclosed
    }

    pub fn outside() -> Geom2dGccQualifier {
        Geom2dGccQualifier::Outside
    }

    pub fn qualifier_to_string(q: Geom2dGccQualifier) -> &'static str {
        match q {
            Geom2dGccQualifier::Unqualified => "Unqualified",
            Geom2dGccQualifier::Enclosing => "Enclosing",
            Geom2dGccQualifier::Enclosed => "Enclosed",
            Geom2dGccQualifier::Outside => "Outside",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qualifiers() {
        assert_eq!(Geom2dGcc::unqualified(), Geom2dGccQualifier::Unqualified);
        assert_eq!(Geom2dGcc::enclosing(), Geom2dGccQualifier::Enclosing);
    }

    #[test]
    fn test_qualifier_to_string() {
        assert_eq!(Geom2dGcc::qualifier_to_string(Geom2dGccQualifier::Outside), "Outside");
    }
}
