// FILE: vrml_font_style_family.rs
// occt: Vrml_FontStyleFamily
//
// Faithful port of OCCT Vrml_FontStyleFamily
// (DataExchange/TKDEVRML/Vrml/Vrml_FontStyleFamily.hxx): the family field of
// the VRML 1.0 FontStyle node. C++ enumerators (declaration order):
// Vrml_SERIF, Vrml_SANS, Vrml_TYPEWRITER.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlFontStyleFamilyKind {
    VrmlSerif = 0,
    VrmlSans = 1,
    VrmlTypewriter = 2,
}

impl VrmlFontStyleFamilyKind {
    /// All enumerators in C++ declaration order.
    pub fn values() -> [VrmlFontStyleFamilyKind; 3] {
        [
            VrmlFontStyleFamilyKind::VrmlSerif,
            VrmlFontStyleFamilyKind::VrmlSans,
            VrmlFontStyleFamilyKind::VrmlTypewriter,
        ]
    }

    /// The VRML 1.0 keyword emitted for this family value.
    pub fn vrml_keyword(self) -> &'static str {
        match self {
            VrmlFontStyleFamilyKind::VrmlSerif => "SERIF",
            VrmlFontStyleFamilyKind::VrmlSans => "SANS",
            VrmlFontStyleFamilyKind::VrmlTypewriter => "TYPEWRITER",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_values_match_declaration_order() {
        assert_eq!(VrmlFontStyleFamilyKind::VrmlSerif as i32, 0);
        assert_eq!(VrmlFontStyleFamilyKind::VrmlSans as i32, 1);
        assert_eq!(VrmlFontStyleFamilyKind::VrmlTypewriter as i32, 2);
    }

    #[test]
    fn keywords() {
        let vals = VrmlFontStyleFamilyKind::values();
        assert_eq!(vals[0].vrml_keyword(), "SERIF");
        assert_eq!(vals[1].vrml_keyword(), "SANS");
        assert_eq!(vals[2].vrml_keyword(), "TYPEWRITER");
    }
}
