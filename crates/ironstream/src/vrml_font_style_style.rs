// FILE: vrml_font_style_style.rs
// occt: Vrml_FontStyleStyle
//
// Faithful port of OCCT Vrml_FontStyleStyle
// (DataExchange/TKDEVRML/Vrml/Vrml_FontStyleStyle.hxx): the style field of
// the VRML 1.0 FontStyle node. C++ enumerators (declaration order):
// Vrml_STYLE_NORMAL, Vrml_STYLE_BOLD, Vrml_STYLE_ITALIC, Vrml_STYLE_BOLDITALIC.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlFontStyleStyleKind {
    VrmlStyleNormal = 0,
    VrmlStyleBold = 1,
    VrmlStyleItalic = 2,
    VrmlStyleBolditalic = 3,
}

impl VrmlFontStyleStyleKind {
    /// All enumerators in C++ declaration order.
    pub fn values() -> [VrmlFontStyleStyleKind; 4] {
        [
            VrmlFontStyleStyleKind::VrmlStyleNormal,
            VrmlFontStyleStyleKind::VrmlStyleBold,
            VrmlFontStyleStyleKind::VrmlStyleItalic,
            VrmlFontStyleStyleKind::VrmlStyleBolditalic,
        ]
    }

    /// The VRML 1.0 keyword emitted for this style value.
    pub fn vrml_keyword(self) -> &'static str {
        match self {
            VrmlFontStyleStyleKind::VrmlStyleNormal => "NORMAL",
            VrmlFontStyleStyleKind::VrmlStyleBold => "BOLD",
            VrmlFontStyleStyleKind::VrmlStyleItalic => "ITALIC",
            VrmlFontStyleStyleKind::VrmlStyleBolditalic => "BOLDITALIC",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_values_match_declaration_order() {
        assert_eq!(VrmlFontStyleStyleKind::VrmlStyleNormal as i32, 0);
        assert_eq!(VrmlFontStyleStyleKind::VrmlStyleBold as i32, 1);
        assert_eq!(VrmlFontStyleStyleKind::VrmlStyleItalic as i32, 2);
        assert_eq!(VrmlFontStyleStyleKind::VrmlStyleBolditalic as i32, 3);
    }

    #[test]
    fn keywords() {
        let vals = VrmlFontStyleStyleKind::values();
        assert_eq!(vals[0].vrml_keyword(), "NORMAL");
        assert_eq!(vals[1].vrml_keyword(), "BOLD");
        assert_eq!(vals[2].vrml_keyword(), "ITALIC");
        assert_eq!(vals[3].vrml_keyword(), "BOLDITALIC");
    }
}
