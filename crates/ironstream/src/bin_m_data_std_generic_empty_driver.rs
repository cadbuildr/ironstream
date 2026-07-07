// FILE: bin_m_data_std_generic_empty_driver.rs
// occt: BinMDataStd_GenericEmptyDriver

pub struct BinMDataStdGenericEmptyDriver {
    message_driver: Option<String>,
}

impl BinMDataStdGenericEmptyDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdGenericEmptyDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdGenericEmptyDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
