// FILE: hlrb_rep_the_polyhedron_of_inter_c_surf.rs
// occt: HLRBRep_ThePolyhedronOfInterCSurf

#[derive(Clone, Debug)]
pub struct HLRBRepThePolyhedronOfInterCSurf {
    nb_vertices: i32,
    nb_triangles: i32,
}

impl HLRBRepThePolyhedronOfInterCSurf {
    pub fn new() -> Self {
        HLRBRepThePolyhedronOfInterCSurf {
            nb_vertices: 0,
            nb_triangles: 0,
        }
    }

    pub fn nb_vertices(&self) -> i32 {
        self.nb_vertices
    }

    pub fn nb_triangles(&self) -> i32 {
        self.nb_triangles
    }

    pub fn set_nb_vertices(&mut self, n: i32) {
        self.nb_vertices = n;
    }

    pub fn set_nb_triangles(&mut self, n: i32) {
        self.nb_triangles = n;
    }
}

impl Default for HLRBRepThePolyhedronOfInterCSurf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let poly = HLRBRepThePolyhedronOfInterCSurf::new();
        assert_eq!(poly.nb_vertices(), 0);
    }

    #[test]
    fn test_set_counts() {
        let mut poly = HLRBRepThePolyhedronOfInterCSurf::new();
        poly.set_nb_vertices(8);
        poly.set_nb_triangles(12);
        assert_eq!(poly.nb_vertices(), 8);
        assert_eq!(poly.nb_triangles(), 12);
    }
}
