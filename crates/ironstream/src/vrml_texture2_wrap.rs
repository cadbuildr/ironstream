// FILE: vrml_texture2_wrap.rs
// occt: Vrml_Texture2Wrap
//
// Faithful port of OCCT Vrml_Texture2Wrap (DataExchange/TKDEVRML/Vrml/
// Vrml_Texture2Wrap.hxx/.cxx): enumeration for VRML texture wrapping modes.

/// Port of Vrml_Texture2Wrap enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlTexture2Wrap {
    Repeat,
    Clamp,
}

impl VrmlTexture2Wrap {
    pub fn to_string(&self) -> &'static str {
        match self {
            VrmlTexture2Wrap::Repeat => "REPEAT",
            VrmlTexture2Wrap::Clamp => "CLAMP",
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "REPEAT" => Some(VrmlTexture2Wrap::Repeat),
            "CLAMP" => Some(VrmlTexture2Wrap::Clamp),
            _ => None,
        }
    }
}

impl std::fmt::Display for VrmlTexture2Wrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_to_string() {
        assert_eq!(VrmlTexture2Wrap::Repeat.to_string(), "REPEAT");
    }

    #[test]
    fn clamp_to_string() {
        assert_eq!(VrmlTexture2Wrap::Clamp.to_string(), "CLAMP");
    }

    #[test]
    fn from_string_repeat() {
        let wrap = VrmlTexture2Wrap::from_string("REPEAT");
        assert_eq!(wrap, Some(VrmlTexture2Wrap::Repeat));
    }

    #[test]
    fn from_string_clamp() {
        let wrap = VrmlTexture2Wrap::from_string("CLAMP");
        assert_eq!(wrap, Some(VrmlTexture2Wrap::Clamp));
    }

    #[test]
    fn from_string_lowercase() {
        let wrap = VrmlTexture2Wrap::from_string("repeat");
        assert_eq!(wrap, Some(VrmlTexture2Wrap::Repeat));
    }

    #[test]
    fn from_string_invalid() {
        let wrap = VrmlTexture2Wrap::from_string("INVALID");
        assert_eq!(wrap, None);
    }

    #[test]
    fn display_trait() {
        let repeat = VrmlTexture2Wrap::Repeat;
        let clamp = VrmlTexture2Wrap::Clamp;
        assert_eq!(format!("{}", repeat), "REPEAT");
        assert_eq!(format!("{}", clamp), "CLAMP");
    }

    #[test]
    fn equality() {
        let a = VrmlTexture2Wrap::Repeat;
        let b = VrmlTexture2Wrap::Repeat;
        let c = VrmlTexture2Wrap::Clamp;
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
