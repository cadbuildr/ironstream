// FILE: bin_m_data_std_integer_driver.rs
// occt: BinMDataStd_IntegerDriver

pub struct BinMDataStdIntegerDriver {
    message_driver: Option<String>,
}

impl BinMDataStdIntegerDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdIntegerDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdIntegerDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
