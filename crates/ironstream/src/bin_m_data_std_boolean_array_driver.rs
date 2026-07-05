// FILE: bin_m_data_std_boolean_array_driver.rs
// occt: BinMDataStd_BooleanArrayDriver

pub struct BinMDataStdBooleanArrayDriver {
    message_driver: Option<String>,
}

impl BinMDataStdBooleanArrayDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdBooleanArrayDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdBooleanArrayDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
