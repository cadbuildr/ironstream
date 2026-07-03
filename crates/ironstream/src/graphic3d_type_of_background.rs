// FILE: graphic3d_type_of_background.rs
// occt: Graphic3d_TypeOfBackground

//! Describes type of view background.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeOfBackground {
    None = -1,
    Gradient = 0,
    Texture = 1,
    Cubemap = 2,
}

impl TypeOfBackground {
    pub const COUNT: usize = 3;

    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            -1 => Some(TypeOfBackground::None),
            0 => Some(TypeOfBackground::Gradient),
            1 => Some(TypeOfBackground::Texture),
            2 => Some(TypeOfBackground::Cubemap),
            _ => None,
        }
    }

    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient() {
        let bg = TypeOfBackground::Gradient;
        assert_eq!(bg as i32, 0);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(TypeOfBackground::from_i32(-1), Some(TypeOfBackground::None));
        assert_eq!(TypeOfBackground::from_i32(0), Some(TypeOfBackground::Gradient));
        assert_eq!(TypeOfBackground::from_i32(2), Some(TypeOfBackground::Cubemap));
        assert_eq!(TypeOfBackground::from_i32(99), None);
    }

    #[test]
    fn test_count() {
        assert_eq!(TypeOfBackground::COUNT, 3);
    }
}
