// FILE: math_value_and_weight.rs
// occt: math_ValueAndWeight

/// Value and weight pair for quadrature and sampling.
#[derive(Debug, Clone)]
pub struct ValueAndWeight {
    pub value: f64,
    pub weight: f64,
}

impl ValueAndWeight {
    pub fn new(value: f64, weight: f64) -> Self {
        ValueAndWeight { value, weight }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn get_weight(&self) -> f64 {
        self.weight
    }

    pub fn set_value(&mut self, val: f64) {
        self.value = val;
    }

    pub fn set_weight(&mut self, w: f64) {
        self.weight = w;
    }

    /// Weighted value.
    pub fn weighted(&self) -> f64 {
        self.value * self.weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_and_weight() {
        let vw = ValueAndWeight::new(2.0, 3.0);
        assert_eq!(vw.get_value(), 2.0);
        assert_eq!(vw.get_weight(), 3.0);
        assert_eq!(vw.weighted(), 6.0);
    }
}
