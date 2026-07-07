// FILE: bin_m_data_xtd.rs
// occt: BinMDataXtd

/// Binary drivers for extended data attributes
#[derive(Default, Clone, Debug)]
pub struct BinMDataXtd;

impl BinMDataXtd {
    /// Factory method for creating drivers
    pub fn factory() -> Self {
        Self
    }

    /// Returns the version number
    pub fn version() -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory() {
        let _drivers = BinMDataXtd::factory();
    }

    #[test]
    fn test_version() {
        assert_eq!(BinMDataXtd::version(), 1);
    }
}
