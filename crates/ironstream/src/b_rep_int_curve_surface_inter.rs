// FILE: b_rep_int_curve_surface_inter.rs
// occt: BRepIntCurveSurface_Inter

/// Curve-surface intersection
pub struct Intersection {
    nb_points: i32,
}

impl Intersection {
    pub fn new() -> Self {
        Intersection { nb_points: 0 }
    }

    pub fn perform(&mut self) {
        self.nb_points = 0;
    }

    pub fn nb_points(&self) -> i32 {
        self.nb_points
    }
}

impl Default for Intersection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_creation() {
        let inter = Intersection::new();
        assert_eq!(inter.nb_points(), 0);
    }
}
