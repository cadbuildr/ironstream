// FILE: geom_plate_array1_of_h_curve.rs
// occt: GeomPlate_Array1OfHCurve

//! Deprecated: Use Vec<HCurve> directly.
//! Alias for backward compatibility with OCCT.

#[derive(Clone, Debug)]
pub struct HCurve {
    pub handle_id: usize,
}

impl HCurve {
    pub fn new(id: usize) -> Self {
        HCurve { handle_id: id }
    }
}

pub type GeomPlateArray1OfHCurve = Vec<HCurve>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let mut array: GeomPlateArray1OfHCurve = Vec::new();
        assert_eq!(array.len(), 0);

        array.push(HCurve::new(1));
        assert_eq!(array.len(), 1);
        assert_eq!(array[0].handle_id, 1);
    }

    #[test]
    fn test_array_operations() {
        let mut array: GeomPlateArray1OfHCurve = Vec::new();
        for i in 0..5 {
            array.push(HCurve::new(i));
        }

        assert_eq!(array.len(), 5);
        assert_eq!(array[0].handle_id, 0);
        assert_eq!(array[4].handle_id, 4);
    }

    #[test]
    fn test_array_clone() {
        let array = vec![HCurve::new(10), HCurve::new(20)];
        let cloned = array.clone();
        assert_eq!(cloned.len(), 2);
        assert_eq!(cloned[1].handle_id, 20);
    }
}
