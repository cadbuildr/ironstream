// FILE: gcc_int_bisec.rs
// occt: GccInt_Bisec

use crate::gcc_int_i_type::GccIntIType;

/// Base trait for bisecting curves between geometric objects.
pub trait GccIntBisec {
    /// Returns the type of bisecting curve.
    fn arc_type(&self) -> GccIntIType;

    /// Returns point (panics if not Pnt type).
    fn point(&self) -> (f64, f64) {
        panic!("Point() called on non-Pnt bisecting curve")
    }

    /// Returns line (panics if not Lin type).
    fn line(&self) -> (f64, f64, f64, f64) {
        panic!("Line() called on non-Lin bisecting curve")
    }

    /// Returns circle (panics if not Cir type).
    fn circle(&self) -> (f64, f64, f64) {
        panic!("Circle() called on non-Cir bisecting curve")
    }

    /// Returns hyperbola (panics if not Hpr type).
    fn hyperbola(&self) -> (f64, f64, f64, f64, f64) {
        panic!("Hyperbola() called on non-Hpr bisecting curve")
    }

    /// Returns parabola (panics if not Par type).
    fn parabola(&self) -> (f64, f64, f64, f64, f64) {
        panic!("Parabola() called on non-Par bisecting curve")
    }

    /// Returns ellipse (panics if not Ell type).
    fn ellipse(&self) -> (f64, f64, f64, f64, f64) {
        panic!("Ellipse() called on non-Ell bisecting curve")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockBisec {
        itype: GccIntIType,
    }

    impl GccIntBisec for MockBisec {
        fn arc_type(&self) -> GccIntIType {
            self.itype
        }

        fn circle(&self) -> (f64, f64, f64) {
            (1.0, 2.0, 3.0)
        }
    }

    #[test]
    fn test_mock_bisec() {
        let mock = MockBisec { itype: GccIntIType::Cir };
        assert_eq!(mock.arc_type(), GccIntIType::Cir);
    }

    #[test]
    #[should_panic]
    fn test_point_panic() {
        let mock = MockBisec { itype: GccIntIType::Cir };
        let _ = mock.point();
    }
}
