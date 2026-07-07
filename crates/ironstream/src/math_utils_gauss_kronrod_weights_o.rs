// FILE: math_utils_gauss_kronrod_weights_o.rs
// occt: MathUtils_GaussKronrodWeights

/// Gauss-Kronrod quadrature weights.
pub struct GaussKronrodWeights {
    order: usize,
}

impl GaussKronrodWeights {
    pub fn new(order: usize) -> Self {
        Self { order }
    }

    pub fn order(&self) -> usize {
        self.order
    }
}
