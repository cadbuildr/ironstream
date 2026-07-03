// FILE: blend_func_section_shape.rs
// occt: BlendFunc_SectionShape

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendFuncSectionShape {
    Rational = 0,
    QuasiAngular = 1,
    Polynomial = 2,
    Linear = 3,
}

impl BlendFuncSectionShape {
    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(BlendFuncSectionShape::Rational),
            1 => Some(BlendFuncSectionShape::QuasiAngular),
            2 => Some(BlendFuncSectionShape::Polynomial),
            3 => Some(BlendFuncSectionShape::Linear),
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
    fn test_section_shapes() {
        assert_eq!(BlendFuncSectionShape::Rational.to_i32(), 0);
        assert_eq!(BlendFuncSectionShape::QuasiAngular.to_i32(), 1);
        assert_eq!(BlendFuncSectionShape::Polynomial.to_i32(), 2);
        assert_eq!(BlendFuncSectionShape::Linear.to_i32(), 3);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(
            BlendFuncSectionShape::from_i32(0),
            Some(BlendFuncSectionShape::Rational)
        );
        assert_eq!(
            BlendFuncSectionShape::from_i32(3),
            Some(BlendFuncSectionShape::Linear)
        );
        assert_eq!(BlendFuncSectionShape::from_i32(99), None);
    }
}
