// FILE: hlrb_rep_the_polygon2d_of_the_int_p_curve_p_curve_of_c_inter.rs
// occt: HLRBRep_ThePolygon2dOfTheIntPCurvePCurveOfCInter

#[derive(Clone, Debug)]
pub struct HLRBRepThePolygon2d {
    vertices: Vec<(f64, f64)>,
}

impl HLRBRepThePolygon2d {
    pub fn new() -> Self {
        HLRBRepThePolygon2d {
            vertices: Vec::new(),
        }
    }

    pub fn nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn vertex(&self, idx: usize) -> Option<(f64, f64)> {
        self.vertices.get(idx).copied()
    }

    pub fn add_vertex(&mut self, v: (f64, f64)) {
        self.vertices.push(v);
    }
}

impl Default for HLRBRepThePolygon2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let poly = HLRBRepThePolygon2d::new();
        assert_eq!(poly.nb_vertices(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut poly = HLRBRepThePolygon2d::new();
        poly.add_vertex((0.0, 0.0));
        poly.add_vertex((1.0, 1.0));
        assert_eq!(poly.nb_vertices(), 2);
    }
}
