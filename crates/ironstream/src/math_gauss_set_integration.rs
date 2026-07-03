// FILE: math_gauss_set_integration.rs
// occt: math_GaussSetIntegration

/// Gauss integration for function sets.
pub struct GaussSetIntegration {
    order: usize,
    nb_dimensions: usize,
}

impl GaussSetIntegration {
    pub fn new(order: usize, nb_dims: usize) -> Self {
        GaussSetIntegration {
            order,
            nb_dimensions: nb_dims,
        }
    }

    pub fn get_order(&self) -> usize {
        self.order
    }

    pub fn nb_dimensions(&self) -> usize {
        self.nb_dimensions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss_set_integration() {
        let int = GaussSetIntegration::new(3, 2);
        assert_eq!(int.get_order(), 3);
        assert_eq!(int.nb_dimensions(), 2);
    }
}
