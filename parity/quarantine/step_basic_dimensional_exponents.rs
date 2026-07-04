// FILE: step_basic_dimensional_exponents.rs
// occt: StepBasic_DimensionalExponents

pub struct StepBasic_DimensionalExponents {
    length_exponent: f64,
    mass_exponent: f64,
    time_exponent: f64,
    electric_current_exponent: f64,
    thermodynamic_temperature_exponent: f64,
    amount_of_substance_exponent: f64,
    luminous_intensity_exponent: f64,
}

impl StepBasic_DimensionalExponents {
    pub fn new() -> Self {
        StepBasic_DimensionalExponents {
            length_exponent: 0.0,
            mass_exponent: 0.0,
            time_exponent: 0.0,
            electric_current_exponent: 0.0,
            thermodynamic_temperature_exponent: 0.0,
            amount_of_substance_exponent: 0.0,
            luminous_intensity_exponent: 0.0,
        }
    }

    pub fn init(
        &mut self,
        length: f64,
        mass: f64,
        time: f64,
        electric_current: f64,
        thermodynamic_temperature: f64,
        amount_of_substance: f64,
        luminous_intensity: f64,
    ) {
        self.length_exponent = length;
        self.mass_exponent = mass;
        self.time_exponent = time;
        self.electric_current_exponent = electric_current;
        self.thermodynamic_temperature_exponent = thermodynamic_temperature;
        self.amount_of_substance_exponent = amount_of_substance;
        self.luminous_intensity_exponent = luminous_intensity;
    }

    pub fn set_length_exponent(&mut self, val: f64) { self.length_exponent = val; }
    pub fn length_exponent(&self) -> f64 { self.length_exponent }

    pub fn set_mass_exponent(&mut self, val: f64) { self.mass_exponent = val; }
    pub fn mass_exponent(&self) -> f64 { self.mass_exponent }

    pub fn set_time_exponent(&mut self, val: f64) { self.time_exponent = val; }
    pub fn time_exponent(&self) -> f64 { self.time_exponent }

    pub fn set_electric_current_exponent(&mut self, val: f64) { self.electric_current_exponent = val; }
    pub fn electric_current_exponent(&self) -> f64 { self.electric_current_exponent }

    pub fn set_thermodynamic_temperature_exponent(&mut self, val: f64) { self.thermodynamic_temperature_exponent = val; }
    pub fn thermodynamic_temperature_exponent(&self) -> f64 { self.thermodynamic_temperature_exponent }

    pub fn set_amount_of_substance_exponent(&mut self, val: f64) { self.amount_of_substance_exponent = val; }
    pub fn amount_of_substance_exponent(&self) -> f64 { self.amount_of_substance_exponent }

    pub fn set_luminous_intensity_exponent(&mut self, val: f64) { self.luminous_intensity_exponent = val; }
    pub fn luminous_intensity_exponent(&self) -> f64 { self.luminous_intensity_exponent }
}

impl Default for StepBasic_DimensionalExponents {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let de = StepBasic_DimensionalExponents::new();
        assert_eq!(de.length_exponent(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut de = StepBasic_DimensionalExponents::new();
        de.init(1.0, 1.0, -2.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(de.length_exponent(), 1.0);
        assert_eq!(de.mass_exponent(), 1.0);
        assert_eq!(de.time_exponent(), -2.0);
    }
}
