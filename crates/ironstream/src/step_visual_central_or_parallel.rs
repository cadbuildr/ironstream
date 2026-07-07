// FILE: step_visual_central_or_parallel.rs
// occt: StepVisual_CentralOrParallel

/// Represents a union for central or parallel projection
#[derive(Debug, Clone)]
pub enum StepVisual_CentralOrParallel {
    Central(String),
    Parallel(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_central() {
        let cop = StepVisual_CentralOrParallel::Central("central1".to_string());
        match cop {
            StepVisual_CentralOrParallel::Central(ref c) => assert_eq!(c, "central1"),
            _ => panic!("Expected Central"),
        }
    }

    #[test]
    fn test_parallel() {
        let cop = StepVisual_CentralOrParallel::Parallel("parallel1".to_string());
        match cop {
            StepVisual_CentralOrParallel::Parallel(ref p) => assert_eq!(p, "parallel1"),
            _ => panic!("Expected Parallel"),
        }
    }
}
