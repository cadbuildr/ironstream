// FILE: math_pso_particles_pool.rs
// occt: math_PSOParticlesPool

/// Pool of particles for Particle Swarm Optimization.
pub struct PSOParticlesPool {
    positions: Vec<Vec<f64>>,
    velocities: Vec<Vec<f64>>,
    best_positions: Vec<Vec<f64>>,
    fitness_values: Vec<f64>,
}

impl PSOParticlesPool {
    /// Create particle pool.
    pub fn new(nb_particles: usize, nb_dimensions: usize) -> Self {
        PSOParticlesPool {
            positions: vec![vec![0.0; nb_dimensions]; nb_particles],
            velocities: vec![vec![0.0; nb_dimensions]; nb_particles],
            best_positions: vec![vec![0.0; nb_dimensions]; nb_particles],
            fitness_values: vec![f64::INFINITY; nb_particles],
        }
    }

    pub fn nb_particles(&self) -> usize {
        self.positions.len()
    }

    pub fn nb_dimensions(&self) -> usize {
        if self.positions.is_empty() {
            0
        } else {
            self.positions[0].len()
        }
    }

    pub fn get_position(&self, i: usize) -> Option<&Vec<f64>> {
        self.positions.get(i)
    }

    pub fn get_fitness(&self, i: usize) -> Option<f64> {
        self.fitness_values.get(i).copied()
    }

    pub fn set_fitness(&mut self, i: usize, val: f64) {
        if i < self.fitness_values.len() {
            self.fitness_values[i] = val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pso_particles_pool() {
        let pool = PSOParticlesPool::new(10, 3);
        assert_eq!(pool.nb_particles(), 10);
        assert_eq!(pool.nb_dimensions(), 3);
    }

    #[test]
    fn test_pso_fitness() {
        let mut pool = PSOParticlesPool::new(5, 2);
        pool.set_fitness(0, 1.5);
        assert_eq!(pool.get_fitness(0), Some(1.5));
    }
}
