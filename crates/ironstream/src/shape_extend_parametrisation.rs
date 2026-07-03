// FILE: shape_extend_parametrisation.rs
// occt: ShapeExtend_Parametrisation

/// Parametrisation types for composite surfaces.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Parametrisation {
    /// Natural: each patch adds its range, Ui+1 = Ui + URange(i,1), etc.
    Natural = 0,
    /// Uniform: each patch gives range 1.: Ui = i-1, Vj = j-1
    Uniform = 1,
    /// Unitary: uniform parametrisation with global range [0,1]
    Unitary = 2,
}

impl Parametrisation {
    /// Convert from raw u32 value.
    pub fn from_raw(value: u32) -> Option<Self> {
        match value {
            0 => Some(Parametrisation::Natural),
            1 => Some(Parametrisation::Uniform),
            2 => Some(Parametrisation::Unitary),
            _ => None,
        }
    }

    /// Convert to raw u32 value.
    pub fn to_raw(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parametrisation_values() {
        assert_eq!(Parametrisation::Natural as u32, 0);
        assert_eq!(Parametrisation::Uniform as u32, 1);
        assert_eq!(Parametrisation::Unitary as u32, 2);
    }

    #[test]
    fn test_from_raw() {
        assert_eq!(Parametrisation::from_raw(0), Some(Parametrisation::Natural));
        assert_eq!(Parametrisation::from_raw(1), Some(Parametrisation::Uniform));
        assert_eq!(Parametrisation::from_raw(2), Some(Parametrisation::Unitary));
        assert_eq!(Parametrisation::from_raw(3), None);
    }

    #[test]
    fn test_to_raw() {
        assert_eq!(Parametrisation::Natural.to_raw(), 0);
        assert_eq!(Parametrisation::Uniform.to_raw(), 1);
        assert_eq!(Parametrisation::Unitary.to_raw(), 2);
    }
}
