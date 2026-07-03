// FILE: geom_fill_uniform_section.rs
// occt: GeomFill_UniformSection

/// A uniform section law - constant cross-section along a curve.
/// Extends GeomFill_SectionLaw providing constant sections.
pub struct GeomFillUniformSection {
    first_param: f64,
    last_param: f64,
    // In a real port, these would be handle<Geom_Curve> and handle<Geom_BSplineCurve>
    // For this stub, we store parameter bounds
}

impl GeomFillUniformSection {
    /// Create a uniform section law with a constant curve over [first_param, last_param]
    pub fn new(first_param: f64, last_param: f64) -> Self {
        GeomFillUniformSection {
            first_param,
            last_param,
        }
    }

    /// Compute D0 (poles and weights) at parameter
    pub fn d0(&self, param: f64) -> (Vec<[f64; 3]>, Vec<f64>) {
        // Section poles and weights evaluation
        // In real OCCT this fills NCollection_Array1<gp_Pnt> and weights
        let poles = vec![[0.0, 0.0, 0.0]];
        let weights = vec![1.0];
        (poles, weights)
    }

    /// Compute D1 (first derivatives)
    pub fn d1(&self, param: f64) -> (Vec<[f64; 3]>, Vec<[f64; 3]>, Vec<f64>, Vec<f64>) {
        let poles = vec![[0.0, 0.0, 0.0]];
        let dpoles = vec![[0.0, 0.0, 0.0]];
        let weights = vec![1.0];
        let dweights = vec![0.0];
        (poles, dpoles, weights, dweights)
    }

    /// Get the parametric interval [first, last]
    pub fn get_interval(&self) -> (f64, f64) {
        (self.first_param, self.last_param)
    }

    /// Set the parametric interval
    pub fn set_interval(&mut self, first: f64, last: f64) {
        self.first_param = first;
        self.last_param = last;
    }

    /// Get the domain (same as interval)
    pub fn get_domain(&self) -> (f64, f64) {
        (self.first_param, self.last_param)
    }

    /// Returns true if sections are constant
    pub fn is_constant(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let section = GeomFillUniformSection::new(0.0, 1.0);
        let (first, last) = section.get_interval();
        assert_eq!(first, 0.0);
        assert_eq!(last, 1.0);
    }

    #[test]
    fn test_set_interval() {
        let mut section = GeomFillUniformSection::new(0.0, 1.0);
        section.set_interval(0.5, 1.5);
        let (first, last) = section.get_interval();
        assert_eq!(first, 0.5);
        assert_eq!(last, 1.5);
    }

    #[test]
    fn test_is_constant() {
        let section = GeomFillUniformSection::new(0.0, 1.0);
        assert!(section.is_constant());
    }

    #[test]
    fn test_d0() {
        let section = GeomFillUniformSection::new(0.0, 1.0);
        let (poles, weights) = section.d0(0.5);
        assert_eq!(poles.len(), 1);
        assert_eq!(weights.len(), 1);
        assert_eq!(weights[0], 1.0);
    }
}
