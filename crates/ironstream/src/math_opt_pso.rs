// FILE: math_opt_pso.rs
// occt: MathOpt_PSO

use std::f64;

/// Initialization mode for PSO particles.
// occt-ref: PSOInitMode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSOInitMode {
    RandomOnly,      // All particles randomly initialized (default)
    SeededOnly,      // Initialize from seed particles only
    SeededPlusRandom, // Seed particles + remaining random
}

/// Boundary handling mode for particles leaving the search space.
// occt-ref: PSOBoundaryMode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSOBoundaryMode {
    Clamp,   // Clamp to bound, reverse velocity with -0.5 damping (default)
    Reflect, // Reflect coordinate, damp velocity by -0.5
    Wrap,    // Periodic wrap into bounds
}

/// Inertia weight schedule.
// occt-ref: PSOInertiaSchedule
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSOInertiaSchedule {
    Constant,   // Fixed omega (default)
    LinearDecay, // Linear decay from Omega to OmegaMin over iterations
}

/// Seed particle for PSO initialization.
// occt-ref: PSOSeedParticle
#[derive(Clone, Debug)]
pub struct PSOSeedParticle {
    pub position: Vec<f64>,
    pub value: Option<f64>,
    pub velocity: Option<Vec<f64>>,
}

impl PSOSeedParticle {
    pub fn new(position: Vec<f64>) -> Self {
        Self {
            position,
            value: None,
            velocity: None,
        }
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_velocity(mut self, velocity: Vec<f64>) -> Self {
        self.velocity = Some(velocity);
        self
    }
}

/// Statistics collected during PSO execution.
// occt-ref: PSOStats
#[derive(Clone, Debug)]
pub struct PSOStats {
    pub nb_function_evals: usize,
    pub nb_iterations: usize,
    pub nb_boundary_corrections: usize,
    pub nb_stagnation_events: usize,
    pub nb_restarts: usize,
    pub initial_best: f64,
    pub final_best: f64,
}

impl PSOStats {
    pub fn new() -> Self {
        Self {
            nb_function_evals: 0,
            nb_iterations: 0,
            nb_boundary_corrections: 0,
            nb_stagnation_events: 0,
            nb_restarts: 0,
            initial_best: f64::MAX,
            final_best: f64::MAX,
        }
    }
}

impl Default for PSOStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for Particle Swarm Optimization.
// occt-ref: PSOConfig
#[derive(Clone, Debug)]
pub struct PSOConfig {
    pub nb_particles: usize,
    pub omega: f64,
    pub phi_personal: f64,
    pub phi_global: f64,
    pub velocity_clamp: f64,
    pub seed: u32,
    pub init_mode: PSOInitMode,
    pub boundary_mode: PSOBoundaryMode,
    pub inertia_schedule: PSOInertiaSchedule,
    pub omega_min: f64,
    pub min_iterations: usize,
    pub target_value: Option<f64>,
    pub no_improve_tol: f64,
    pub no_improve_iters: usize,
    pub restart_fraction: f64,
    pub max_restarts: usize,
    pub polish_budget_per_dim: usize,
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl PSOConfig {
    pub fn new() -> Self {
        Self {
            nb_particles: 40,
            omega: 0.7,
            phi_personal: 1.5,
            phi_global: 1.5,
            velocity_clamp: 0.5,
            seed: 6,
            init_mode: PSOInitMode::RandomOnly,
            boundary_mode: PSOBoundaryMode::Clamp,
            inertia_schedule: PSOInertiaSchedule::Constant,
            omega_min: 0.4,
            min_iterations: 0,
            target_value: None,
            no_improve_tol: 0.0,
            no_improve_iters: 10,
            restart_fraction: 0.0,
            max_restarts: 0,
            polish_budget_per_dim: 50,
            max_iterations: 100,
            tolerance: 1.0e-8,
        }
    }

    pub fn with_particles(mut self, n: usize) -> Self {
        self.nb_particles = n;
        self
    }

    pub fn with_iterations(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }

    pub fn with_tolerance(mut self, tol: f64) -> Self {
        self.tolerance = tol;
        self
    }
}

impl Default for PSOConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Result type for PSO optimization
// occt-ref: VectorResult
#[derive(Clone, Debug)]
pub struct VectorResult {
    pub solution: Vec<f64>,
    pub value: f64,
    pub nb_iterations: usize,
    pub is_done: bool,
}

impl VectorResult {
    pub fn new(dim: usize) -> Self {
        Self {
            solution: vec![0.0; dim],
            value: f64::MAX,
            nb_iterations: 0,
            is_done: false,
        }
    }
}

struct Particle {
    position: Vec<f64>,
    velocity: Vec<f64>,
    best_position: Vec<f64>,
    best_value: f64,
    current_value: f64,
}

impl Particle {
    fn new(dim: usize) -> Self {
        Self {
            position: vec![0.0; dim],
            velocity: vec![0.0; dim],
            best_position: vec![0.0; dim],
            best_value: f64::MAX,
            current_value: f64::MAX,
        }
    }
}

/// Simple linear congruential generator for reproducibility
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u32) -> Self {
        Self {
            state: seed as u64,
        }
    }

    fn next_real(&mut self) -> f64 {
        // LCG parameters
        const A: u64 = 1103515245;
        const C: u64 = 12345;
        const M: u64 = 2u64.pow(31);

        self.state = (A.wrapping_mul(self.state).wrapping_add(C)) % M;
        (self.state as f64) / (M as f64)
    }
}

/// Particle Swarm Optimization for global minimization.
///
/// PSO is a stochastic, population-based optimization algorithm inspired
/// by social behavior of bird flocking or fish schooling.
pub fn pso<F>(
    func: &F,
    lower_bounds: &[f64],
    upper_bounds: &[f64],
    config: &PSOConfig,
    seeds: Option<&[PSOSeedParticle]>,
) -> VectorResult
where
    F: Fn(&[f64], &mut f64) -> bool + ?Sized,
{
    let n = lower_bounds.len();
    let mut result = VectorResult::new(n);

    // Validate input
    if upper_bounds.len() != n || config.nb_particles == 0 {
        return result;
    }

    for i in 0..n {
        if lower_bounds[i] >= upper_bounds[i] {
            return result;
        }
    }

    let mut stats = PSOStats::new();
    let mut rng = SimpleRng::new(config.seed);

    // Compute velocity limits
    let mut vel_max = vec![0.0; n];
    let mut range = vec![0.0; n];
    for i in 0..n {
        range[i] = upper_bounds[i] - lower_bounds[i];
        vel_max[i] = config.velocity_clamp * range[i];
    }

    // Initialize particles
    let mut swarm: Vec<Particle> = (0..config.nb_particles)
        .map(|_| Particle::new(n))
        .collect();

    let mut global_best = vec![0.0; n];
    let mut global_best_value = f64::MAX;

    // Seed initialization
    let nb_seeds = seeds.map(|s| s.len()).unwrap_or(0);
    let mut seeded = 0;

    if nb_seeds > 0
        && (config.init_mode == PSOInitMode::SeededPlusRandom
            || config.init_mode == PSOInitMode::SeededOnly)
    {
        let seeds = seeds.unwrap();
        let nb_direct = (nb_seeds).min(config.nb_particles);

        for seed_idx in 0..nb_direct {
            let seed = &seeds[seed_idx];
            let particle = &mut swarm[seed_idx];

            // Clamp seed position
            for i in 0..n {
                particle.position[i] =
                    seed.position[i].clamp(lower_bounds[i], upper_bounds[i]);
            }

            // Set velocity
            if let Some(vel) = &seed.velocity {
                for i in 0..n {
                    particle.velocity[i] = vel[i].clamp(-vel_max[i], vel_max[i]);
                }
            } else {
                for i in 0..n {
                    particle.velocity[i] = (2.0 * rng.next_real() - 1.0) * vel_max[i];
                }
            }

            // Evaluate
            if !func(&particle.position, &mut particle.current_value) {
                particle.current_value = f64::MAX;
            }
            stats.nb_function_evals += 1;

            particle.best_position = particle.position.clone();
            particle.best_value = particle.current_value;

            if particle.best_value < global_best_value {
                global_best_value = particle.best_value;
                global_best = particle.best_position.clone();
            }
        }
        seeded = nb_direct;
    }

    // Fill remaining particles randomly
    for part_idx in seeded..config.nb_particles {
        let particle = &mut swarm[part_idx];
        for i in 0..n {
            particle.position[i] = lower_bounds[i] + rng.next_real() * range[i];
            particle.velocity[i] = (2.0 * rng.next_real() - 1.0) * vel_max[i];
        }

        if !func(&particle.position, &mut particle.current_value) {
            particle.current_value = f64::MAX;
        }
        stats.nb_function_evals += 1;

        particle.best_position = particle.position.clone();
        particle.best_value = particle.current_value;

        if particle.best_value < global_best_value {
            global_best_value = particle.best_value;
            global_best = particle.best_position.clone();
        }
    }

    stats.initial_best = global_best_value;

    // Main PSO loop
    let mut prev_best = global_best_value;
    let mut stagnation_count = 0;
    let stag_tol = if config.no_improve_tol > 0.0 {
        config.no_improve_tol
    } else {
        config.tolerance
    };

    for iter in 0..config.max_iterations {
        result.nb_iterations = iter + 1;

        // Compute inertia weight
        let omega = match config.inertia_schedule {
            PSOInertiaSchedule::Constant => config.omega,
            PSOInertiaSchedule::LinearDecay => {
                config.omega
                    - (config.omega - config.omega_min) * (iter as f64)
                        / (config.max_iterations as f64)
            }
        };

        // Update particles
        for particle in &mut swarm {
            // Update velocity
            for i in 0..n {
                let r1 = rng.next_real();
                let r2 = rng.next_real();

                let v_new = omega * particle.velocity[i]
                    + config.phi_personal * r1 * (particle.best_position[i] - particle.position[i])
                    + config.phi_global * r2 * (global_best[i] - particle.position[i]);

                particle.velocity[i] = v_new.clamp(-vel_max[i], vel_max[i]);
            }

            // Update position with boundary handling
            for i in 0..n {
                let mut x_new = particle.position[i] + particle.velocity[i];

                if x_new < lower_bounds[i] || x_new > upper_bounds[i] {
                    stats.nb_boundary_corrections += 1;

                    match config.boundary_mode {
                        PSOBoundaryMode::Clamp => {
                            x_new = x_new.clamp(lower_bounds[i], upper_bounds[i]);
                            particle.velocity[i] *= -0.5;
                        }
                        PSOBoundaryMode::Reflect => {
                            if x_new < lower_bounds[i] {
                                x_new = 2.0 * lower_bounds[i] - x_new;
                            } else {
                                x_new = 2.0 * upper_bounds[i] - x_new;
                            }
                            x_new = x_new.clamp(lower_bounds[i], upper_bounds[i]);
                            particle.velocity[i] *= -0.5;
                        }
                        PSOBoundaryMode::Wrap => {
                            x_new -= lower_bounds[i];
                            x_new = x_new.rem_euclid(range[i]);
                            x_new += lower_bounds[i];
                        }
                    }
                }

                particle.position[i] = x_new;
            }

            // Evaluate fitness
            if !func(&particle.position, &mut particle.current_value) {
                particle.current_value = f64::MAX;
            }
            stats.nb_function_evals += 1;

            // Update personal best
            if particle.current_value < particle.best_value {
                particle.best_value = particle.current_value;
                particle.best_position = particle.position.clone();
            }

            // Update global best
            if particle.best_value < global_best_value {
                global_best_value = particle.best_value;
                global_best = particle.best_position.clone();
            }
        }

        // Check target value early stop
        if let Some(target) = config.target_value {
            if global_best_value <= target {
                break;
            }
        }

        // Check for convergence after minimum iterations
        if iter >= config.min_iterations {
            if (global_best_value - prev_best).abs()
                < stag_tol * (1.0 + global_best_value.abs())
            {
                stagnation_count += 1;
                if stagnation_count >= config.no_improve_iters {
                    stats.nb_stagnation_events += 1;

                    // Restart logic
                    if config.restart_fraction > 0.0
                        && (config.max_restarts == 0 || stats.nb_restarts < config.max_restarts)
                    {
                        stats.nb_restarts += 1;
                        stagnation_count = 0;

                        // Reinitialize worst particles
                        let nb_restart = ((config.restart_fraction * config.nb_particles as f64)
                            .ceil() as usize)
                            .min(config.nb_particles - 1);

                        // Find worst particles and reinitialize
                        for part_idx in 0..nb_restart.min(swarm.len()) {
                            let particle = &mut swarm[part_idx];
                            for i in 0..n {
                                particle.position[i] = lower_bounds[i] + rng.next_real() * range[i];
                                particle.velocity[i] =
                                    (2.0 * rng.next_real() - 1.0) * vel_max[i];
                            }

                            if !func(&particle.position, &mut particle.current_value) {
                                particle.current_value = f64::MAX;
                            }
                            stats.nb_function_evals += 1;

                            particle.best_position = particle.position.clone();
                            particle.best_value = particle.current_value;

                            if particle.best_value < global_best_value {
                                global_best_value = particle.best_value;
                                global_best = particle.best_position.clone();
                            }
                        }
                    } else {
                        break;
                    }
                }
            } else {
                stagnation_count = 0;
            }
        }

        prev_best = global_best_value;
    }

    stats.nb_iterations = result.nb_iterations;
    stats.final_best = global_best_value;

    result.solution = global_best;
    result.value = global_best_value;
    result.is_done = true;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sphere_func(x: &[f64], f: &mut f64) -> bool {
        *f = x.iter().map(|xi| xi * xi).sum();
        true
    }

    #[test]
    fn test_sphere_2d() {
        let lower = vec![-5.0, -5.0];
        let upper = vec![5.0, 5.0];
        let config = PSOConfig::new()
            .with_particles(50)
            .with_iterations(200)
            .with_tolerance(1.0e-6);

        let result = pso(&sphere_func, &lower, &upper, &config, None);

        assert!(result.is_done);
        assert!(result.value < 1.0e-6);
    }

    #[test]
    fn test_config_default() {
        let config = PSOConfig::default();
        assert_eq!(config.nb_particles, 40);
        assert_eq!(config.max_iterations, 100);
        assert_eq!(config.omega, 0.7);
    }

    #[test]
    fn test_seed_particle() {
        let pos = vec![1.0, 2.0];
        let seed = PSOSeedParticle::new(pos)
            .with_value(5.0)
            .with_velocity(vec![0.1, 0.2]);

        assert_eq!(seed.value, Some(5.0));
        assert!(seed.velocity.is_some());
    }

    #[test]
    fn test_pso_stats() {
        let mut stats = PSOStats::new();
        stats.nb_function_evals = 100;
        assert_eq!(stats.nb_function_evals, 100);
    }

    #[test]
    fn test_invalid_bounds() {
        let lower = vec![5.0, 5.0];
        let upper = vec![0.0, 0.0]; // Invalid: lower > upper
        let config = PSOConfig::default();

        let result = pso(&sphere_func, &lower, &upper, &config, None);
        assert!(!result.is_done);
    }
}
