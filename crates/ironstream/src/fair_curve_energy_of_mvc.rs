// FILE: fair_curve_energy_of_mvc.rs
// occt: FairCurve_EnergyOfMVC

/// Energy functional for minimum variation curve (MVC)
#[derive(Clone, Debug)]
pub struct FairCurveEnergyOfMvc {
    energy: f64,
}

impl FairCurveEnergyOfMvc {
    pub fn new() -> Self {
        FairCurveEnergyOfMvc { energy: 0.0 }
    }

    pub fn set_energy(&mut self, energy: f64) {
        self.energy = energy;
    }

    pub fn energy(&self) -> f64 {
        self.energy
    }
}

impl Default for FairCurveEnergyOfMvc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let energy = FairCurveEnergyOfMvc::new();
        assert_eq!(energy.energy(), 0.0);
    }

    #[test]
    fn test_set_energy() {
        let mut energy = FairCurveEnergyOfMvc::new();
        energy.set_energy(6.0);
        assert_eq!(energy.energy(), 6.0);
    }
}
