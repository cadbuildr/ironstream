// FILE: bin_m_data_std_integer_list_driver.rs
// occt: BinMDataStd_IntegerListDriver

pub struct BinMDataStdIntegerListDriver {
    message_driver: Option<String>,
}

impl BinMDataStdIntegerListDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdIntegerListDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdIntegerListDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
