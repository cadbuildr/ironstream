// FILE: rw_gltf_gltf_accessor_comp_type.rs
// occt: RWGltf_GltfAccessorCompType

//! Component type enumeration for glTF accessor.

/// Accessor component type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessorComponentType {
    Byte = 5120,
    UnsignedByte = 5121,
    Short = 5122,
    UnsignedShort = 5123,
    UnsignedInt = 5125,
    Float = 5126,
}

impl AccessorComponentType {
    pub fn size(&self) -> usize {
        match self {
            AccessorComponentType::Byte | AccessorComponentType::UnsignedByte => 1,
            AccessorComponentType::Short | AccessorComponentType::UnsignedShort => 2,
            AccessorComponentType::UnsignedInt | AccessorComponentType::Float => 4,
        }
    }

    pub fn from_value(value: u32) -> Option<Self> {
        match value {
            5120 => Some(AccessorComponentType::Byte),
            5121 => Some(AccessorComponentType::UnsignedByte),
            5122 => Some(AccessorComponentType::Short),
            5123 => Some(AccessorComponentType::UnsignedShort),
            5125 => Some(AccessorComponentType::UnsignedInt),
            5126 => Some(AccessorComponentType::Float),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizes() {
        assert_eq!(AccessorComponentType::Byte.size(), 1);
        assert_eq!(AccessorComponentType::Short.size(), 2);
        assert_eq!(AccessorComponentType::Float.size(), 4);
    }

    #[test]
    fn test_from_value() {
        assert_eq!(AccessorComponentType::from_value(5126), Some(AccessorComponentType::Float));
        assert_eq!(AccessorComponentType::from_value(9999), None);
    }
}
