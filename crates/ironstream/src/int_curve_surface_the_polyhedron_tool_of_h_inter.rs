// FILE: int_curve_surface_the_polyhedron_tool_of_h_inter.rs
// occt: IntCurveSurface_ThePolyhedronToolOfHInter

pub struct IntCurveSurfaceThePolyhedronToolOfHInter;

impl IntCurveSurfaceThePolyhedronToolOfHInter {
    pub fn new() -> Self {
        IntCurveSurfaceThePolyhedronToolOfHInter
    }

    pub fn nb_triangles() -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let _tool = IntCurveSurfaceThePolyhedronToolOfHInter::new();
    }

    #[test]
    fn test_nb_triangles() {
        assert_eq!(IntCurveSurfaceThePolyhedronToolOfHInter::nb_triangles(), 0);
    }
}
