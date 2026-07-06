// FILE: step_data_factors.rs
// occt: StepData_Factors

// Class for using units variables
pub struct StepDataFactors {
    length_factor: f64,
    plane_angle_factor: f64,
    solid_angle_factor: f64,
    fact_rd: f64,
    fact_dr: f64,
    cascade_unit: f64,
}

impl StepDataFactors {
    // Constructor
    pub fn new() -> Self {
        StepDataFactors {
            length_factor: 1.0,
            plane_angle_factor: 1.0,
            solid_angle_factor: 1.0,
            fact_rd: 1.0,
            fact_dr: 1.0,
            cascade_unit: 1.0,
        }
    }

    // Initializes the 3 factors for the conversion of units
    pub fn initialize_factors(
        &mut self,
        length_factor: f64,
        plane_angle_factor: f64,
        solid_angle_factor: f64,
    ) {
        self.length_factor = length_factor;
        self.plane_angle_factor = plane_angle_factor;
        self.solid_angle_factor = solid_angle_factor;
        self.fact_rd = 1.0 / plane_angle_factor;
        self.fact_dr = plane_angle_factor;
    }

    // Sets length unit for current transfer process
    pub fn set_cascade_unit(&mut self, unit: f64) {
        self.cascade_unit = unit;
    }

    // Returns length unit for current transfer process (mm by default)
    pub fn cascade_unit(&self) -> f64 {
        self.cascade_unit
    }

    // Returns transient length factor for scaling of shapes
    pub fn length_factor(&self) -> f64 {
        self.length_factor
    }

    // Returns transient plane angle factor for conversion of angles
    pub fn plane_angle_factor(&self) -> f64 {
        self.plane_angle_factor
    }

    // Returns transient solid angle factor for conversion of angles
    pub fn solid_angle_factor(&self) -> f64 {
        self.solid_angle_factor
    }

    // Returns transient factor radian degree for conversion of angles
    pub fn factor_radian_degree(&self) -> f64 {
        self.fact_rd
    }

    // Returns transient factor degree radian for conversion of angles
    pub fn factor_degree_radian(&self) -> f64 {
        self.fact_dr
    }
}

impl Default for StepDataFactors {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factors_new() {
        let factors = StepDataFactors::new();
        assert_eq!(factors.length_factor(), 1.0);
        assert_eq!(factors.plane_angle_factor(), 1.0);
        assert_eq!(factors.solid_angle_factor(), 1.0);
    }

    #[test]
    fn test_initialize_factors() {
        let mut factors = StepDataFactors::new();
        factors.initialize_factors(2.0, 3.0, 4.0);
        assert_eq!(factors.length_factor(), 2.0);
        assert_eq!(factors.plane_angle_factor(), 3.0);
        assert_eq!(factors.solid_angle_factor(), 4.0);
        assert!((factors.factor_radian_degree() - 1.0 / 3.0).abs() < 1e-10);
        assert!((factors.factor_degree_radian() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_cascade_unit() {
        let mut factors = StepDataFactors::new();
        factors.set_cascade_unit(10.0);
        assert_eq!(factors.cascade_unit(), 10.0);
    }
}
