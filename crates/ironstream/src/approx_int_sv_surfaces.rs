// FILE: approx_int_sv_surfaces.rs
// occt: ApproxInt_SvSurfaces

/// Surface-surface approximation with singularities and vertices
#[derive(Clone, Debug)]
pub struct ApproxIntSvSurfaces {
    nb_surfaces: usize,
    is_approximated: bool,
}

impl ApproxIntSvSurfaces {
    pub fn new() -> Self {
        ApproxIntSvSurfaces { nb_surfaces: 0, is_approximated: false }
    }

    pub fn add_surface(&mut self) {
        self.nb_surfaces += 1;
    }

    pub fn nb_surfaces(&self) -> usize {
        self.nb_surfaces
    }

    pub fn approximate(&mut self) {
        self.is_approximated = true;
    }

    pub fn is_approximated(&self) -> bool {
        self.is_approximated
    }
}

impl Default for ApproxIntSvSurfaces {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sv = ApproxIntSvSurfaces::new();
        assert_eq!(sv.nb_surfaces(), 0);
        assert!(!sv.is_approximated());
    }

    #[test]
    fn test_add_surface() {
        let mut sv = ApproxIntSvSurfaces::new();
        sv.add_surface();
        assert_eq!(sv.nb_surfaces(), 1);
    }

    #[test]
    fn test_approximate() {
        let mut sv = ApproxIntSvSurfaces::new();
        sv.approximate();
        assert!(sv.is_approximated());
    }
}
