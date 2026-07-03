// FILE: geom_tools_curve2d_set.rs
// occt: GeomTools_Curve2dSet

/// A set of 2D curves for geometric tools.
pub struct GeomToolsCurve2dSet {
    curves: Vec<Vec<(f64, f64)>>,
}

impl GeomToolsCurve2dSet {
    /// Create a new 2D curve set.
    pub fn new() -> Self {
        GeomToolsCurve2dSet { curves: Vec::new() }
    }

    /// Add a 2D curve.
    pub fn add_curve(&mut self, curve: Vec<(f64, f64)>) {
        self.curves.push(curve);
    }

    /// Get the number of curves.
    pub fn nb_curves(&self) -> usize {
        self.curves.len()
    }

    /// Get a curve by index.
    pub fn curve(&self, index: usize) -> Option<&[(f64, f64)]> {
        if index < self.curves.len() {
            Some(&self.curves[index])
        } else {
            None
        }
    }

    /// Clear all curves.
    pub fn clear(&mut self) {
        self.curves.clear();
    }
}

impl Default for GeomToolsCurve2dSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_set() {
        let set = GeomToolsCurve2dSet::new();
        assert_eq!(set.nb_curves(), 0);
    }

    #[test]
    fn test_add_curve() {
        let mut set = GeomToolsCurve2dSet::new();
        let curve = vec![(0.0, 0.0), (1.0, 1.0)];
        set.add_curve(curve);
        assert_eq!(set.nb_curves(), 1);
    }

    #[test]
    fn test_get_curve() {
        let mut set = GeomToolsCurve2dSet::new();
        let curve = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)];
        set.add_curve(curve.clone());
        let retrieved = set.curve(0).unwrap();
        assert_eq!(retrieved.len(), 3);
        assert_eq!(retrieved[0], (0.0, 0.0));
    }

    #[test]
    fn test_clear() {
        let mut set = GeomToolsCurve2dSet::new();
        set.add_curve(vec![(0.0, 0.0)]);
        set.add_curve(vec![(1.0, 1.0)]);
        assert_eq!(set.nb_curves(), 2);
        set.clear();
        assert_eq!(set.nb_curves(), 0);
    }
}
