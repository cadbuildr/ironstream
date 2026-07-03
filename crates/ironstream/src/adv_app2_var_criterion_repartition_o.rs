// FILE: adv_app2_var_criterion_repartition_o.rs
// occt: AdvApp2Var_CriterionRepartition

//! Criterion for repartitioning in advanced approximation.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepartitionStrategy {
    UniformDivision,
    ErrorBased,
    Hybrid,
}

#[derive(Clone, Debug)]
pub struct CriterionRepartition {
    pub strategy: RepartitionStrategy,
    pub target_error: f64,
    pub max_segments: usize,
}

impl CriterionRepartition {
    pub fn new(strategy: RepartitionStrategy, target_error: f64, max_segments: usize) -> Self {
        CriterionRepartition {
            strategy,
            target_error,
            max_segments,
        }
    }

    pub fn compute_repartition(&self, total_error: f64, current_segments: usize) -> usize {
        match self.strategy {
            RepartitionStrategy::UniformDivision => (current_segments * 2).min(self.max_segments),
            RepartitionStrategy::ErrorBased => {
                let ratio = (total_error / self.target_error).sqrt();
                ((current_segments as f64) * ratio).ceil() as usize
            }
            RepartitionStrategy::Hybrid => {
                let uniform_next = current_segments * 2;
                let error_based = ((current_segments as f64) * (total_error / self.target_error).sqrt()).ceil() as usize;
                uniform_next.max(error_based).min(self.max_segments)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criterion_create() {
        let crit = CriterionRepartition::new(RepartitionStrategy::UniformDivision, 1e-6, 100);
        assert_eq!(crit.max_segments, 100);
    }

    #[test]
    fn test_uniform_repartition() {
        let crit = CriterionRepartition::new(RepartitionStrategy::UniformDivision, 1e-6, 100);
        let next = crit.compute_repartition(1e-3, 10);
        assert_eq!(next, 20);
    }

    #[test]
    fn test_error_based_repartition() {
        let crit = CriterionRepartition::new(RepartitionStrategy::ErrorBased, 1e-6, 100);
        let next = crit.compute_repartition(1e-3, 10);
        assert!(next > 10);
    }
}
