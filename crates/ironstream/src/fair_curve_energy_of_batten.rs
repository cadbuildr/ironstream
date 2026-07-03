// FILE: fair_curve_energy_of_batten.rs
// occt: FairCurve_EnergyOfBatten

/// Energy functional for batten curves
#[derive(Clone, Debug)]
pub struct FairCurveEnergyOfBatten {
    energy: f64,
}

impl FairCurveEnergyOfBatten {
    pub fn new() -> Self {
        FairCurveEnergyOfBatten { energy: 0.0 }
    }

    pub fn set_energy(&mut self, energy: f64) {
        self.energy = energy;
    }

    pub fn energy(&self) -> f64 {
        self.energy
    }
}

impl Default for FairCurveEnergyOfBatten {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let energy = FairCurveEnergyOfBatten::new();
        assert_eq!(energy.energy(), 0.0);
    }

    #[test]
    fn test_set_energy() {
        let mut energy = FairCurveEnergyOfBatten::new();
        energy.set_energy(8.0);
        assert_eq!(energy.energy(), 8.0);
    }
}
