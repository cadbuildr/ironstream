// FILE: adv_app2_var_patch_o.rs
// occt: AdvApp2Var_Patch

//! Patch representation for advanced surface approximation.

#[derive(Clone, Debug)]
pub struct Patch {
    pub u_min: f64,
    pub u_max: f64,
    pub v_min: f64,
    pub v_max: f64,
    pub coefficients: Vec<Vec<f64>>,
}

impl Patch {
    pub fn new(u_min: f64, u_max: f64, v_min: f64, v_max: f64) -> Self {
        Patch {
            u_min,
            u_max,
            v_min,
            v_max,
            coefficients: vec![],
        }
    }

    pub fn parameter_area(&self) -> f64 {
        (self.u_max - self.u_min) * (self.v_max - self.v_min)
    }

    pub fn evaluate(&self, u: f64, v: f64) -> Option<f64> {
        if u < self.u_min || u > self.u_max || v < self.v_min || v > self.v_max {
            return None;
        }

        let mut result = 0.0;
        let nu = self.coefficients.len();
        let nv = if nu > 0 { self.coefficients[0].len() } else { 0 };

        for i in 0..nu {
            for j in 0..nv {
                result += self.coefficients[i][j] * (u.powi(i as i32)) * (v.powi(j as i32));
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_create() {
        let patch = Patch::new(0.0, 1.0, 0.0, 1.0);
        assert_eq!(patch.parameter_area(), 1.0);
    }

    #[test]
    fn test_patch_evaluate() {
        let mut patch = Patch::new(0.0, 1.0, 0.0, 1.0);
        patch.coefficients = vec![vec![1.0]];
        let result = patch.evaluate(0.5, 0.5);
        assert_eq!(result, Some(1.0));
    }
}
