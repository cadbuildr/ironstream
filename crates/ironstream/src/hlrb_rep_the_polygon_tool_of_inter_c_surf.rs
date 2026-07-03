// FILE: hlrb_rep_the_polygon_tool_of_inter_c_surf.rs
// occt: HLRBRep_ThePolygonToolOfInterCSurf

use crate::hlrb_rep_the_polygon_of_inter_c_surf::HLRBRepThePolygonOfInterCSurf;

pub struct HLRBRepThePolygonToolOfInterCSurf;

impl HLRBRepThePolygonToolOfInterCSurf {
    pub fn nb_vertices(poly: &HLRBRepThePolygonOfInterCSurf) -> usize {
        poly.nb_vertices()
    }

    pub fn vertex(poly: &HLRBRepThePolygonOfInterCSurf, idx: usize) -> Option<(f64, f64, f64)> {
        poly.vertex(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool() {
        let mut poly = HLRBRepThePolygonOfInterCSurf::new();
        poly.add_vertex((1.0, 2.0, 3.0));
        assert_eq!(HLRBRepThePolygonToolOfInterCSurf::nb_vertices(&poly), 1);
    }
}
