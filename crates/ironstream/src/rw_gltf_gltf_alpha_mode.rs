// FILE: rw_gltf_gltf_alpha_mode.rs
// occt: RWGltf_GltfAlphaMode

//! Alpha blend mode enumeration for glTF materials.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}

impl AlphaMode {
    pub fn as_string(&self) -> &'static str {
        match self {
            AlphaMode::Opaque => "OPAQUE",
            AlphaMode::Mask => "MASK",
            AlphaMode::Blend => "BLEND",
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "OPAQUE" => Some(AlphaMode::Opaque),
            "MASK" => Some(AlphaMode::Mask),
            "BLEND" => Some(AlphaMode::Blend),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_string() {
        assert_eq!(AlphaMode::Opaque.as_string(), "OPAQUE");
        assert_eq!(AlphaMode::Blend.as_string(), "BLEND");
    }

    #[test]
    fn test_from_string() {
        assert_eq!(AlphaMode::from_string("OPAQUE"), Some(AlphaMode::Opaque));
        assert_eq!(AlphaMode::from_string("INVALID"), None);
    }
}
