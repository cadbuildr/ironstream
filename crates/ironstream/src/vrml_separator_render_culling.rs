// FILE: vrml_separator_render_culling.rs
// occt: Vrml_SeparatorRenderCulling
//
// Faithful port of OCCT Vrml_SeparatorRenderCulling
// (DataExchange/TKDEVRML/Vrml/Vrml_SeparatorRenderCulling.hxx):
// the render-culling switch of the VRML 1.0 Separator node.
// C++ enumerators (in declaration order): Vrml_OFF, Vrml_ON, Vrml_AUTO.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlSeparatorRenderCullingKind {
    VrmlOff = 0,
    VrmlOn = 1,
    VrmlAuto = 2,
}

impl VrmlSeparatorRenderCullingKind {
    /// All enumerators in C++ declaration order.
    pub fn values() -> [VrmlSeparatorRenderCullingKind; 3] {
        [
            VrmlSeparatorRenderCullingKind::VrmlOff,
            VrmlSeparatorRenderCullingKind::VrmlOn,
            VrmlSeparatorRenderCullingKind::VrmlAuto,
        ]
    }

    /// The VRML 1.0 keyword the Separator node writer emits for this value
    /// (AUTO is the default and is normally suppressed).
    pub fn vrml_keyword(self) -> &'static str {
        match self {
            VrmlSeparatorRenderCullingKind::VrmlOff => "OFF",
            VrmlSeparatorRenderCullingKind::VrmlOn => "ON",
            VrmlSeparatorRenderCullingKind::VrmlAuto => "AUTO",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_values_match_declaration_order() {
        assert_eq!(VrmlSeparatorRenderCullingKind::VrmlOff as i32, 0);
        assert_eq!(VrmlSeparatorRenderCullingKind::VrmlOn as i32, 1);
        assert_eq!(VrmlSeparatorRenderCullingKind::VrmlAuto as i32, 2);
    }

    #[test]
    fn keywords() {
        let vals = VrmlSeparatorRenderCullingKind::values();
        assert_eq!(vals[0].vrml_keyword(), "OFF");
        assert_eq!(vals[1].vrml_keyword(), "ON");
        assert_eq!(vals[2].vrml_keyword(), "AUTO");
    }
}
