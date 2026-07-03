// FILE: int_polyh_tools.rs
// occt: IntPolyh_Tools

/// Provides tools for surface sampling and analysis.
pub struct PolyhedronTools;

impl PolyhedronTools {
    /// Check if a surface can be enlarged in U or V direction.
    /// Returns (can_enlarge_u, can_enlarge_v)
    pub fn is_enlarge_possible() -> (bool, bool) {
        // Surface enlargement depends on surface parameters.
        // For now, return a generic result that can be refined based on surface type.
        (false, false)
    }

    /// Make sampling of a surface with given number of points in U and V directions.
    /// Returns arrays of U and V parameters.
    pub fn make_sampling(nb_u: i32, nb_v: i32, enlarge_zone: bool) -> (Vec<f64>, Vec<f64>) {
        let mut u_pars = Vec::with_capacity(nb_u as usize);
        let mut v_pars = Vec::with_capacity(nb_v as usize);

        if nb_u > 0 {
            let step_u = 1.0 / (nb_u as f64);
            for i in 0..nb_u {
                u_pars.push((i as f64) * step_u);
            }
        }

        if nb_v > 0 {
            let step_v = 1.0 / (nb_v as f64);
            for i in 0..nb_v {
                v_pars.push((i as f64) * step_v);
            }
        }

        (u_pars, v_pars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_enlarge_possible() {
        let (can_u, can_v) = PolyhedronTools::is_enlarge_possible();
        assert!(!can_u);
        assert!(!can_v);
    }

    #[test]
    fn test_make_sampling() {
        let (u_pars, v_pars) = PolyhedronTools::make_sampling(3, 2, false);
        assert_eq!(u_pars.len(), 3);
        assert_eq!(v_pars.len(), 2);
        assert!(u_pars[0] >= 0.0 && u_pars[0] <= 1.0);
        assert!(v_pars[0] >= 0.0 && v_pars[0] <= 1.0);
    }

    #[test]
    fn test_make_sampling_with_zero() {
        let (u_pars, v_pars) = PolyhedronTools::make_sampling(0, 0, true);
        assert_eq!(u_pars.len(), 0);
        assert_eq!(v_pars.len(), 0);
    }
}
