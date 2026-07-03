// FILE: hlrb_rep_the_polygon_of_inter_c_surf.rs
// occt: HLRBRep_ThePolygonOfInterCSurf

#[derive(Clone, Debug)]
pub struct HLRBRepThePolygonOfInterCSurf {
    vertices: Vec<(f64, f64, f64)>,
}

impl HLRBRepThePolygonOfInterCSurf {
    pub fn new() -> Self {
        HLRBRepThePolygonOfInterCSurf {
            vertices: Vec::new(),
        }
    }

    pub fn nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn vertex(&self, idx: usize) -> Option<(f64, f64, f64)> {
        self.vertices.get(idx).copied()
    }

    pub fn add_vertex(&mut self, v: (f64, f64, f64)) {
        self.vertices.push(v);
    }
}

impl Default for HLRBRepThePolygonOfInterCSurf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let poly = HLRBRepThePolygonOfInterCSurf::new();
        assert_eq!(poly.nb_vertices(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut poly = HLRBRepThePolygonOfInterCSurf::new();
        poly.add_vertex((1.0, 2.0, 3.0));
        assert_eq!(poly.nb_vertices(), 1);
    }
}
