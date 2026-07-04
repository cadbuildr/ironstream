// FILE: units_measurement.rs
// occt: Units_Measurement

/// Unit measurement with value and unit.
#[derive(Clone, Copy, Debug)]
pub struct Measurement {
    value: f64,
    unit: &'static str,
}

impl Measurement {
    pub fn new(value: f64, unit: &'static str) -> Self {
        Self { value, unit }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn unit(&self) -> &'static str {
        self.unit
    }

    pub fn to_mm(&self) -> f64 {
        match self.unit {
            "mm" => self.value,
            "cm" => self.value * 10.0,
            "m" => self.value * 1000.0,
            "inch" => self.value * 25.4,
            _ => self.value,
        }
    }
}
