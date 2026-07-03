// FILE: geom_fill_approx_style.rs
// occt: GeomFill_ApproxStyle

/// Enumeration for approximation style
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ApproxStyle {
    Deformation = 0,
    Curved = 1,
    Stretch = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_values() {
        assert_eq!(ApproxStyle::Deformation as i32, 0);
        assert_eq!(ApproxStyle::Curved as i32, 1);
        assert_eq!(ApproxStyle::Stretch as i32, 2);
    }

    #[test]
    fn test_enum_clone() {
        let style = ApproxStyle::Curved;
        let cloned = style;
        assert_eq!(style, cloned);
    }
}
