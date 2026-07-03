// FILE: extrema_pc_parabola.rs
// occt: ExtremaPC_Parabola

//! Find extrema between a point and a parabola.

pub struct ParabolaExtrema {
    focal_length: f64,
    vertex: (f64, f64, f64),
}

impl ParabolaExtrema {
    pub fn new(focal_length: f64, vertex: (f64, f64, f64)) -> Self {
        ParabolaExtrema {
            focal_length,
            vertex,
        }
    }

    pub fn extrema_distance(&self, _point: (f64, f64, f64)) -> Option<f64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let extrema = ParabolaExtrema::new(2.0, (0.0, 0.0, 0.0));
        assert_eq!(extrema.focal_length, 2.0);
    }
}
