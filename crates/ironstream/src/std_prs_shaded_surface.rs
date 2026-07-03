// FILE: std_prs_shaded_surface.rs
// occt: StdPrs_ShadedSurface

/// Shaded surface presentation.
#[derive(Clone, Debug)]
pub struct ShadedSurface {
    pub surface_id: u32,
    pub triangle_count: usize,
    pub is_computed: bool,
}

impl ShadedSurface {
    pub fn new(surface_id: u32) -> Self {
        Self {
            surface_id,
            triangle_count: 0,
            is_computed: false,
        }
    }

    pub fn add_triangle(&mut self) {
        self.triangle_count += 1;
    }

    pub fn nb_triangles(&self) -> usize {
        self.triangle_count
    }

    pub fn compute(&mut self) {
        self.is_computed = true;
    }

    pub fn is_computed(&self) -> bool {
        self.is_computed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_surface() {
        let surf = ShadedSurface::new(1);
        assert_eq!(surf.surface_id, 1);
        assert_eq!(surf.nb_triangles(), 0);
    }

    #[test]
    fn add_triangles() {
        let mut surf = ShadedSurface::new(1);
        for _ in 0..100 {
            surf.add_triangle();
        }
        assert_eq!(surf.nb_triangles(), 100);
    }

    #[test]
    fn compute() {
        let mut surf = ShadedSurface::new(1);
        surf.add_triangle();
        surf.compute();
        assert!(surf.is_computed());
    }
}
