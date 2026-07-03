// FILE: int_curve_surface_the_polygon_tool_of_h_inter.rs
// occt: IntCurveSurface_ThePolygonToolOfHInter

pub struct IntCurveSurfaceThePolygonToolOfHInter;

impl IntCurveSurfaceThePolygonToolOfHInter {
    pub fn new() -> Self {
        IntCurveSurfaceThePolygonToolOfHInter
    }

    pub fn nb_segments() -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let _tool = IntCurveSurfaceThePolygonToolOfHInter::new();
    }

    #[test]
    fn test_nb_segments() {
        assert_eq!(IntCurveSurfaceThePolygonToolOfHInter::nb_segments(), 0);
    }
}
