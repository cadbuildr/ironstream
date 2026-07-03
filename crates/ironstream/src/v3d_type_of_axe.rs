// FILE: v3d_type_of_axe.rs
// occt: V3d_TypeOfAxe

//! Determines the axis type through the coordinates X, Y, Z.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TypeOfAxe {
    #[default]
    X,
    Y,
    Z,
}

impl TypeOfAxe {
    pub fn as_u32(&self) -> u32 {
        match self {
            TypeOfAxe::X => 0,
            TypeOfAxe::Y => 1,
            TypeOfAxe::Z => 2,
        }
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(TypeOfAxe::X),
            1 => Some(TypeOfAxe::Y),
            2 => Some(TypeOfAxe::Z),
            _ => None,
        }
    }

    pub fn axis_direction(&self) -> [f64; 3] {
        match self {
            TypeOfAxe::X => [1.0, 0.0, 0.0],
            TypeOfAxe::Y => [0.0, 1.0, 0.0],
            TypeOfAxe::Z => [0.0, 0.0, 1.0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_of_axe_conversions() {
        assert_eq!(TypeOfAxe::X.as_u32(), 0);
        assert_eq!(TypeOfAxe::Y.as_u32(), 1);
        assert_eq!(TypeOfAxe::Z.as_u32(), 2);
    }

    #[test]
    fn type_of_axe_from_u32() {
        assert_eq!(TypeOfAxe::from_u32(0), Some(TypeOfAxe::X));
        assert_eq!(TypeOfAxe::from_u32(1), Some(TypeOfAxe::Y));
        assert_eq!(TypeOfAxe::from_u32(2), Some(TypeOfAxe::Z));
        assert_eq!(TypeOfAxe::from_u32(3), None);
    }

    #[test]
    fn axis_directions() {
        assert_eq!(TypeOfAxe::X.axis_direction(), [1.0, 0.0, 0.0]);
        assert_eq!(TypeOfAxe::Y.axis_direction(), [0.0, 1.0, 0.0]);
        assert_eq!(TypeOfAxe::Z.axis_direction(), [0.0, 0.0, 1.0]);
    }
}
