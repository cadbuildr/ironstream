// FILE: bin_m_data_std_integer_array_driver.rs
// occt: BinMDataStd_IntegerArrayDriver

pub struct BinMDataStdIntegerArrayDriver {
    message_driver: Option<String>,
}

impl BinMDataStdIntegerArrayDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdIntegerArrayDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdIntegerArrayDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
