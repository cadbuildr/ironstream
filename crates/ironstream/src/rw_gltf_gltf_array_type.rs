// FILE: rw_gltf_gltf_array_type.rs
// occt: RWGltf_GltfArrayType

//! Array type enumeration for glTF accessors.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl ArrayType {
    pub fn components(&self) -> u32 {
        match self {
            ArrayType::Scalar => 1,
            ArrayType::Vec2 => 2,
            ArrayType::Vec3 => 3,
            ArrayType::Vec4 => 4,
            ArrayType::Mat2 => 4,
            ArrayType::Mat3 => 9,
            ArrayType::Mat4 => 16,
        }
    }

    pub fn as_string(&self) -> &'static str {
        match self {
            ArrayType::Scalar => "SCALAR",
            ArrayType::Vec2 => "VEC2",
            ArrayType::Vec3 => "VEC3",
            ArrayType::Vec4 => "VEC4",
            ArrayType::Mat2 => "MAT2",
            ArrayType::Mat3 => "MAT3",
            ArrayType::Mat4 => "MAT4",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_components() {
        assert_eq!(ArrayType::Scalar.components(), 1);
        assert_eq!(ArrayType::Vec3.components(), 3);
        assert_eq!(ArrayType::Mat4.components(), 16);
    }

    #[test]
    fn test_as_string() {
        assert_eq!(ArrayType::Vec2.as_string(), "VEC2");
    }
}
