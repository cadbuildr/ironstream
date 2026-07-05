// FILE: hatch_gen_domains.rs
// occt: HatchGen_Domains

//! Deprecated: Use Vec<Domain> directly.
//! Domains for hatch generation.

#[derive(Clone, Debug)]
pub struct Domain {
    pub domain_id: usize,
    pub start: f64,
    pub end: f64,
}

impl Domain {
    pub fn new(domain_id: usize, start: f64, end: f64) -> Self {
        Domain { domain_id, start, end }
    }

    pub fn length(&self) -> f64 {
        self.end - self.start
    }

    pub fn contains(&self, value: f64) -> bool {
        value >= self.start && value <= self.end
    }
}

pub type HatchGenDomains = Vec<Domain>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_creation() {
        let domain = Domain::new(1, 0.0, 1.0);
        assert_eq!(domain.domain_id, 1);
        assert_eq!(domain.start, 0.0);
        assert_eq!(domain.end, 1.0);
    }

    #[test]
    fn test_domain_length() {
        let domain = Domain::new(1, 0.0, 5.0);
        assert_eq!(domain.length(), 5.0);
    }

    #[test]
    fn test_domain_contains() {
        let domain = Domain::new(1, 0.0, 10.0);
        assert!(domain.contains(5.0));
        assert!(!domain.contains(15.0));
        assert!(domain.contains(0.0));
        assert!(domain.contains(10.0));
    }

    #[test]
    fn test_domains_vector() {
        let mut domains: HatchGenDomains = Vec::new();
        domains.push(Domain::new(1, 0.0, 5.0));
        domains.push(Domain::new(2, 5.0, 10.0));

        assert_eq!(domains.len(), 2);
        assert_eq!(domains[0].length(), 5.0);
        assert_eq!(domains[1].length(), 5.0);
    }

    #[test]
    fn test_domains_iteration() {
        let domains = vec![
            Domain::new(1, 0.0, 1.0),
            Domain::new(2, 1.0, 2.0),
        ];

        let total_length: f64 = domains.iter().map(|d| d.length()).sum();
        assert_eq!(total_length, 2.0);
    }
}
