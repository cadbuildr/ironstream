// FILE: bin_drivers.rs
// occt: BinDrivers

/// Drivers for reading/writing binary format
#[derive(Default, Clone, Debug)]
pub struct BinDrivers;

impl BinDrivers {
    /// Factory method to create drivers
    pub fn factory() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory() {
        let _drivers = BinDrivers::factory();
    }
}
