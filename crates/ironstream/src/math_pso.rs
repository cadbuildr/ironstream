// FILE: math_pso.rs

/// Result returned by [`MathPSO::perform`].
pub struct PSOResult {
    pub best_position: Vec<f64>,
    pub best_value: f64,
    pub iterations: u32,
    pub converged: bool,
}

/// Configuration for the PSO algorithm.
pub struct PSOParams {
    /// Number of particles in the swarm.
    pub nb_particles: u32,
    /// Maximum number of iterations.
    pub max_iter: u32,
    /// Inertia weight `w`.
    pub inertia: f64,
    /// Cognitive acceleration coefficient `c1`.
    pub cognitive: f64,
    /// Social acceleration coefficient `c2`.
    pub social: f64,
    /// Convergence tolerance: stop when the change in global best drops below this.
    pub tolerance: f64,
}

impl PSOParams {
    pub fn default() -> Self {
        Self {
            nb_particles: 32,
            max_iter: 100,
            inertia: 0.729,
            cognitive: 1.494,
            social: 1.494,
            tolerance: 1e-8,
        }
    }
}

// occt: math_PSO
pub struct MathPSO {
    params: PSOParams,
    dimension: usize,
    lower_bounds: Vec<f64>,
    upper_bounds: Vec<f64>,
}

impl MathPSO {
    pub fn new(
        dimension: usize,
        lower: Vec<f64>,
        upper: Vec<f64>,
        params: PSOParams,
    ) -> Self {
        Self {
            params,
            dimension,
            lower_bounds: lower,
            upper_bounds: upper,
        }
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn params(&self) -> &PSOParams {
        &self.params
    }

    /// Run PSO and return the best result found.
    ///
    /// Positions are seeded using a deterministic LCG so runs are reproducible
    /// without an external `rand` crate.  Each particle gets independent
    /// per-dimension initial positions spread across `[lower, upper]`.
    /// Velocities start at zero.  The classic PSO velocity update is applied
    /// with pseudo-random r1, r2 drawn from a per-(iter, particle, dim) LCG.
    pub fn perform<F: Fn(&[f64]) -> f64>(&self, f: F) -> PSOResult {
        let n = self.params.nb_particles as usize;
        let dim = self.dimension;
        let lo = &self.lower_bounds;
        let hi = &self.upper_bounds;

        // Xorshift64 PRNG — high-quality, zero-dep, reproducible.
        let mut rng: u64 = 0x9e37_79b9_7f4a_7c15;
        let mut next_f64 = move || -> f64 {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            // Map to (0, 1).
            (rng >> 11) as f64 / ((1u64 << 53) as f64)
        };

        // Particle positions, velocities, personal bests.
        let mut pos: Vec<Vec<f64>> = Vec::with_capacity(n);
        let mut vel: Vec<Vec<f64>> = Vec::with_capacity(n);
        let mut pbest_pos: Vec<Vec<f64>> = Vec::with_capacity(n);
        let mut pbest_val: Vec<f64> = Vec::with_capacity(n);

        for _ in 0..n {
            let p: Vec<f64> = (0..dim)
                .map(|d| lo[d] + next_f64() * (hi[d] - lo[d]))
                .collect();
            let v: Vec<f64> = vec![0.0; dim];
            let fval = f(&p);
            pbest_val.push(fval);
            pbest_pos.push(p.clone());
            pos.push(p);
            vel.push(v);
        }

        // Global best.
        let mut gbest_idx = 0usize;
        for k in 1..n {
            if pbest_val[k] < pbest_val[gbest_idx] {
                gbest_idx = k;
            }
        }
        let mut gbest_pos = pbest_pos[gbest_idx].clone();
        let mut gbest_val = pbest_val[gbest_idx];

        let w = self.params.inertia;
        let c1 = self.params.cognitive;
        let c2 = self.params.social;

        let mut converged = false;
        let mut final_iter = self.params.max_iter;

        for iter in 0..self.params.max_iter {
            let prev_best = gbest_val;

            for k in 0..n {
                for d in 0..dim {
                    let r1 = next_f64();
                    let r2 = next_f64();

                    let cognitive_term = c1 * r1 * (pbest_pos[k][d] - pos[k][d]);
                    let social_term = c2 * r2 * (gbest_pos[d] - pos[k][d]);
                    vel[k][d] = w * vel[k][d] + cognitive_term + social_term;

                    pos[k][d] = (pos[k][d] + vel[k][d]).clamp(lo[d], hi[d]);
                }

                let fval = f(&pos[k]);
                if fval < pbest_val[k] {
                    pbest_val[k] = fval;
                    pbest_pos[k] = pos[k].clone();
                    if fval < gbest_val {
                        gbest_val = fval;
                        gbest_pos = pos[k].clone();
                    }
                }
            }

            if (prev_best - gbest_val).abs() < self.params.tolerance {
                converged = true;
                final_iter = iter + 1;
                break;
            }
        }

        if !converged {
            final_iter = self.params.max_iter;
        }

        PSOResult {
            best_position: gbest_pos,
            best_value: gbest_val,
            iterations: final_iter,
            converged,
        }
    }
}

/// Result stored by [`MathGlobOptMin`] after a `perform` call.
pub struct GlobOptMinResult {
    pub minimum: f64,
    pub position: Vec<f64>,
    pub is_done: bool,
}

// occt: math_GlobOptMin
pub struct MathGlobOptMin {
    dimension: usize,
    lower: Vec<f64>,
    upper: Vec<f64>,
    best: Option<GlobOptMinResult>,
}

impl MathGlobOptMin {
    pub fn new(dimension: usize, lower: Vec<f64>, upper: Vec<f64>) -> Self {
        Self {
            dimension,
            lower,
            upper,
            best: None,
        }
    }

    /// Grid search with 20 points per dimension (capped at dim ≤ 3) followed
    /// by one PSO refinement pass.
    pub fn perform<F: Fn(&[f64]) -> f64>(&mut self, f: F) {
        const GRID_PTS: usize = 20;
        let dim = self.dimension;
        let effective_dim = dim.min(3);

        let mut best_pos = self.lower.clone();
        let mut best_val = f64::INFINITY;

        // Grid search over up to 3 dimensions; remaining dims stay at lower bound.
        let total = GRID_PTS.pow(effective_dim as u32);
        for idx in 0..total {
            let mut pos = self.lower.clone();
            let mut rem = idx;
            for d in 0..effective_dim {
                let gi = rem % GRID_PTS;
                rem /= GRID_PTS;
                let t = gi as f64 / (GRID_PTS - 1) as f64;
                pos[d] = self.lower[d] + t * (self.upper[d] - self.lower[d]);
            }
            let v = f(&pos);
            if v < best_val {
                best_val = v;
                best_pos = pos;
            }
        }

        // PSO refinement starting from the grid best.
        let mut params = PSOParams::default();
        params.nb_particles = 64;
        params.max_iter = 200;
        params.tolerance = 1e-10;
        let pso = MathPSO::new(dim, self.lower.clone(), self.upper.clone(), params);
        let res = pso.perform(&f);
        if res.best_value < best_val {
            best_val = res.best_value;
            best_pos = res.best_position;
        }

        self.best = Some(GlobOptMinResult {
            minimum: best_val,
            position: best_pos,
            is_done: true,
        });
    }

    pub fn is_done(&self) -> bool {
        self.best.as_ref().map_or(false, |r| r.is_done)
    }

    pub fn result(&self) -> Option<&GlobOptMinResult> {
        self.best.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pso_params_default_values() {
        let p = PSOParams::default();
        assert_eq!(p.nb_particles, 32);
        assert_eq!(p.max_iter, 100);
        assert!((p.inertia - 0.729).abs() < 1e-12);
        assert!((p.cognitive - 1.494).abs() < 1e-12);
        assert!((p.social - 1.494).abs() < 1e-12);
        assert!((p.tolerance - 1e-8).abs() < 1e-20);
    }

    #[test]
    fn math_pso_dimension_accessor() {
        let pso = MathPSO::new(3, vec![0.0; 3], vec![1.0; 3], PSOParams::default());
        assert_eq!(pso.dimension(), 3);
    }

    #[test]
    fn math_pso_params_accessor() {
        let p = PSOParams::default();
        let pso = MathPSO::new(1, vec![0.0], vec![1.0], p);
        assert_eq!(pso.params().nb_particles, 32);
    }

    #[test]
    fn pso_1d_parabola_finds_zero() {
        let pso = MathPSO::new(
            1,
            vec![-2.0],
            vec![2.0],
            PSOParams::default(),
        );
        let res = pso.perform(|x| x[0] * x[0]);
        assert!(res.best_value < 0.1, "best_value={}", res.best_value);
        assert!(res.best_position[0].abs() < 0.4, "pos={}", res.best_position[0]);
    }

    #[test]
    fn pso_2d_bowl_finds_minimum() {
        let mut p = PSOParams::default();
        p.nb_particles = 64;
        p.max_iter = 300;
        let pso = MathPSO::new(2, vec![-5.0, -5.0], vec![5.0, 5.0], p);
        let res = pso.perform(|x| (x[0] - 1.0).powi(2) + (x[1] - 2.0).powi(2));
        assert!(res.best_value < 0.1, "best_value={}", res.best_value);
        assert!((res.best_position[0] - 1.0).abs() < 0.5, "x={}", res.best_position[0]);
        assert!((res.best_position[1] - 2.0).abs() < 0.5, "y={}", res.best_position[1]);
    }

    #[test]
    fn pso_result_fields_accessible() {
        let pso = MathPSO::new(1, vec![-1.0], vec![1.0], PSOParams::default());
        let res = pso.perform(|x| x[0] * x[0]);
        let _ = res.best_position;
        let _ = res.best_value;
        let _ = res.iterations;
        let _ = res.converged;
    }

    #[test]
    fn glob_opt_min_1d_not_done_before_perform() {
        let gom = MathGlobOptMin::new(1, vec![-3.0], vec![3.0]);
        assert!(!gom.is_done());
        assert!(gom.result().is_none());
    }

    #[test]
    fn glob_opt_min_1d_x_squared() {
        let mut gom = MathGlobOptMin::new(1, vec![-3.0], vec![3.0]);
        gom.perform(|x| x[0] * x[0]);
        assert!(gom.is_done());
        let r = gom.result().unwrap();
        assert!(r.minimum < 0.01, "minimum={}", r.minimum);
        assert!(r.position[0].abs() < 0.2, "pos={}", r.position[0]);
    }

    #[test]
    fn glob_opt_min_result_is_done_field() {
        let mut gom = MathGlobOptMin::new(1, vec![0.0], vec![1.0]);
        gom.perform(|x| (x[0] - 0.5).powi(2));
        let r = gom.result().unwrap();
        assert!(r.is_done);
    }
}
