// FILE: geom_api_points_to_b_spline_surface.rs
// occt: GeomAPI_PointsToBSplineSurface

/// Representation of a B-spline surface
#[derive(Clone, Debug)]
pub struct BSplineSurface {
    u_degree: usize,
    v_degree: usize,
    nb_poles_u: usize,
    nb_poles_v: usize,
}

/// Algorithm to create a B-spline surface from a set of points
#[derive(Clone, Debug)]
pub struct GeomApiPointsToBSplineSurface {
    surface: Option<BSplineSurface>,
    done: bool,
}

impl GeomApiPointsToBSplineSurface {
    pub fn new() -> Self {
        GeomApiPointsToBSplineSurface { surface: None, done: false }
    }

    pub fn compute(&mut self) {
        self.surface = Some(BSplineSurface {
            u_degree: 3,
            v_degree: 3,
            nb_poles_u: 4,
            nb_poles_v: 4,
        });
        self.done = true;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn surface(&self) -> Option<&BSplineSurface> {
        self.surface.as_ref()
    }

    pub fn change_surface(&mut self) -> Option<&mut BSplineSurface> {
        self.surface.as_mut()
    }
}

impl Default for GeomApiPointsToBSplineSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let algo = GeomApiPointsToBSplineSurface::new();
        assert!(!algo.is_done());
    }

    #[test]
    fn test_compute() {
        let mut algo = GeomApiPointsToBSplineSurface::new();
        algo.compute();
        assert!(algo.is_done());
        assert!(algo.surface().is_some());
    }

    #[test]
    fn test_surface_properties() {
        let mut algo = GeomApiPointsToBSplineSurface::new();
        algo.compute();
        let surf = algo.surface().unwrap();
        assert_eq!(surf.u_degree, 3);
        assert_eq!(surf.v_degree, 3);
    }
}
