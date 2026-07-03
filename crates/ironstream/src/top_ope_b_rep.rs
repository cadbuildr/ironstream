// FILE: top_ope_b_rep.rs
// occt: TopOpeBRep

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopOpeBRepTypeLineCurve {
    Walking = 0,
    Lin3d = 1,
    Circle = 2,
    Ellipse = 3,
    Parabola = 4,
    Hyperbola = 5,
    Other = 6,
}

pub struct TopOpeBRep;

impl TopOpeBRep {
    pub fn new() -> Self {
        TopOpeBRep
    }

    pub fn print_type_line_curve(tlc: TopOpeBRepTypeLineCurve) -> &'static str {
        match tlc {
            TopOpeBRepTypeLineCurve::Walking => "Walking",
            TopOpeBRepTypeLineCurve::Lin3d => "Lin3d",
            TopOpeBRepTypeLineCurve::Circle => "Circle",
            TopOpeBRepTypeLineCurve::Ellipse => "Ellipse",
            TopOpeBRepTypeLineCurve::Parabola => "Parabola",
            TopOpeBRepTypeLineCurve::Hyperbola => "Hyperbola",
            TopOpeBRepTypeLineCurve::Other => "Other",
        }
    }
}

impl Default for TopOpeBRep {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_line_curve_values() {
        assert_eq!(TopOpeBRepTypeLineCurve::Walking as u32, 0);
        assert_eq!(TopOpeBRepTypeLineCurve::Lin3d as u32, 1);
        assert_eq!(TopOpeBRepTypeLineCurve::Circle as u32, 2);
    }

    #[test]
    fn test_print_type_line_curve() {
        assert_eq!(TopOpeBRep::print_type_line_curve(TopOpeBRepTypeLineCurve::Walking), "Walking");
        assert_eq!(TopOpeBRep::print_type_line_curve(TopOpeBRepTypeLineCurve::Circle), "Circle");
        assert_eq!(TopOpeBRep::print_type_line_curve(TopOpeBRepTypeLineCurve::Other), "Other");
    }

    #[test]
    fn test_top_ope_b_rep_creation() {
        let _tr = TopOpeBRep::new();
        let _tr_default = TopOpeBRep::default();
    }
}
