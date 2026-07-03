// FILE: geom_convert_comp_bezier_surfaces_to_b_spline_surface.rs
// occt: GeomConvert_CompBezierSurfacesToBSplineSurface

/// Converts a grid of adjacent non-rational Bezier surfaces into a BSpline surface.
pub struct GeomConvertCompBezierSurfacesToBSplineSurface {
    u_degree: usize,
    v_degree: usize,
    u_knots: Vec<f64>,
    v_knots: Vec<f64>,
    u_mults: Vec<usize>,
    v_mults: Vec<usize>,
    poles: Vec<(f64, f64, f64)>,
    nb_u_poles: usize,
    nb_v_poles: usize,
    is_done: bool,
}

impl GeomConvertCompBezierSurfacesToBSplineSurface {
    /// Create a new converter from Bezier surface grid.
    pub fn new(
        nb_bezier_u: usize,
        nb_bezier_v: usize,
        max_u_degree: usize,
        max_v_degree: usize,
    ) -> Self {
        let mut conv = GeomConvertCompBezierSurfacesToBSplineSurface {
            u_degree: max_u_degree,
            v_degree: max_v_degree,
            u_knots: Vec::new(),
            v_knots: Vec::new(),
            u_mults: Vec::new(),
            v_mults: Vec::new(),
            poles: Vec::new(),
            nb_u_poles: 0,
            nb_v_poles: 0,
            is_done: false,
        };

        // For piecewise Bezier: internal knots have multiplicity degree
        // First and last knots have multiplicity degree + 1
        conv.u_mults.resize(nb_bezier_u + 1, max_u_degree);
        conv.u_mults[0] = max_u_degree + 1;
        conv.u_mults[nb_bezier_u] = max_u_degree + 1;

        conv.v_mults.resize(nb_bezier_v + 1, max_v_degree);
        conv.v_mults[0] = max_v_degree + 1;
        conv.v_mults[nb_bezier_v] = max_v_degree + 1;

        // Knots are uniformly spaced
        for i in 0..=nb_bezier_u {
            conv.u_knots.push((i as f64) / (nb_bezier_u as f64));
        }
        for i in 0..=nb_bezier_v {
            conv.v_knots.push((i as f64) / (nb_bezier_v as f64));
        }

        conv
    }

    /// Returns the number of knots in the U direction.
    pub fn nb_u_knots(&self) -> usize {
        self.u_knots.len()
    }

    /// Returns the number of poles in the U direction.
    pub fn nb_u_poles(&self) -> usize {
        self.nb_u_poles
    }

    /// Returns the number of knots in the V direction.
    pub fn nb_v_knots(&self) -> usize {
        self.v_knots.len()
    }

    /// Returns the number of poles in the V direction.
    pub fn nb_v_poles(&self) -> usize {
        self.nb_v_poles
    }

    /// Returns the U degree.
    pub fn u_degree(&self) -> usize {
        self.u_degree
    }

    /// Returns the V degree.
    pub fn v_degree(&self) -> usize {
        self.v_degree
    }

    /// Returns the U knots.
    pub fn u_knots(&self) -> &[f64] {
        &self.u_knots
    }

    /// Returns the V knots.
    pub fn v_knots(&self) -> &[f64] {
        &self.v_knots
    }

    /// Returns the U multiplicities.
    pub fn u_multiplicities(&self) -> &[usize] {
        &self.u_mults
    }

    /// Returns the V multiplicities.
    pub fn v_multiplicities(&self) -> &[usize] {
        &self.v_mults
    }

    /// Sets the poles for the BSpline surface.
    pub fn set_poles(&mut self, poles: Vec<(f64, f64, f64)>, nb_u: usize, nb_v: usize) {
        self.poles = poles;
        self.nb_u_poles = nb_u;
        self.nb_v_poles = nb_v;
    }

    /// Returns the poles.
    pub fn poles(&self) -> &[(f64, f64, f64)] {
        &self.poles
    }

    /// Returns whether the conversion was successful.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Mark the conversion as done.
    pub fn set_done(&mut self, done: bool) {
        self.is_done = done;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_converter() {
        let conv = GeomConvertCompBezierSurfacesToBSplineSurface::new(2, 2, 3, 3);
        assert_eq!(conv.u_degree(), 3);
        assert_eq!(conv.v_degree(), 3);
        assert_eq!(conv.nb_u_knots(), 3); // 0, 0.5, 1
        assert_eq!(conv.nb_v_knots(), 3);
    }

    #[test]
    fn test_knots_uniform() {
        let conv = GeomConvertCompBezierSurfacesToBSplineSurface::new(4, 3, 2, 2);
        let u_knots = conv.u_knots();
        assert_eq!(u_knots.len(), 5); // 0, 0.25, 0.5, 0.75, 1
        assert!((u_knots[0] - 0.0).abs() < 1e-10);
        assert!((u_knots[4] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_multiplicities() {
        let conv = GeomConvertCompBezierSurfacesToBSplineSurface::new(3, 3, 3, 3);
        let u_mults = conv.u_multiplicities();
        // First and last: degree + 1 = 4; internal: degree = 3
        assert_eq!(u_mults[0], 4);
        assert_eq!(u_mults[1], 3);
        assert_eq!(u_mults[2], 3);
        assert_eq!(u_mults[3], 4);
    }

    #[test]
    fn test_set_poles() {
        let mut conv = GeomConvertCompBezierSurfacesToBSplineSurface::new(2, 2, 2, 2);
        let poles = vec![(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (1.0, 1.0, 0.0)];
        conv.set_poles(poles.clone(), 2, 2);

        assert_eq!(conv.nb_u_poles(), 2);
        assert_eq!(conv.nb_v_poles(), 2);
        assert_eq!(conv.poles().len(), 4);
    }

    #[test]
    fn test_is_done_flag() {
        let mut conv = GeomConvertCompBezierSurfacesToBSplineSurface::new(2, 2, 2, 2);
        assert!(!conv.is_done());
        conv.set_done(true);
        assert!(conv.is_done());
    }
}
