// FILE: bin_m_data_std_generic_ext_string_driver.rs
// occt: BinMDataStd_GenericExtStringDriver

pub struct BinMDataStdGenericExtStringDriver {
    message_driver: Option<String>,
}

impl BinMDataStdGenericExtStringDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdGenericExtStringDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdGenericExtStringDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
