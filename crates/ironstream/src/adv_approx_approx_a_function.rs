// FILE: adv_approx_approx_a_function.rs
// occt: AdvApprox_ApproxAFunction

pub struct AdvApproxApproxAFunction {
    degree: usize,
    max_degree: usize,
}

impl AdvApproxApproxAFunction {
    pub fn new(degree: usize, max_degree: usize) -> Self {
        AdvApproxApproxAFunction { degree, max_degree }
    }

    pub fn get_degree(&self) -> usize {
        self.degree
    }

    pub fn get_max_degree(&self) -> usize {
        self.max_degree
    }

    pub fn approximate(&self) -> bool {
        self.degree <= self.max_degree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_a_function_new() {
        let approx = AdvApproxApproxAFunction::new(3, 5);
        assert_eq!(approx.get_degree(), 3);
        assert_eq!(approx.get_max_degree(), 5);
    }

    #[test]
    fn test_approx_a_function_approximate() {
        let approx = AdvApproxApproxAFunction::new(3, 5);
        assert!(approx.approximate());

        let approx2 = AdvApproxApproxAFunction::new(7, 5);
        assert!(!approx2.approximate());
    }
}
