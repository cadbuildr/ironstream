// FILE: hlrb_rep_the_polyhedron_tool_of_inter_c_surf.rs
// occt: HLRBRep_ThePolyhedronToolOfInterCSurf

use crate::hlrb_rep_the_polyhedron_of_inter_c_surf::HLRBRepThePolyhedronOfInterCSurf;

pub struct HLRBRepThePolyhedronToolOfInterCSurf;

impl HLRBRepThePolyhedronToolOfInterCSurf {
    pub fn nb_vertices(poly: &HLRBRepThePolyhedronOfInterCSurf) -> i32 {
        poly.nb_vertices()
    }

    pub fn nb_triangles(poly: &HLRBRepThePolyhedronOfInterCSurf) -> i32 {
        poly.nb_triangles()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool() {
        let mut poly = HLRBRepThePolyhedronOfInterCSurf::new();
        poly.set_nb_vertices(8);
        assert_eq!(HLRBRepThePolyhedronToolOfInterCSurf::nb_vertices(&poly), 8);
    }
}
