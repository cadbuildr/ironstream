// FILE: bin_m_data_std_ext_string_list_driver.rs
// occt: BinMDataStd_ExtStringListDriver

pub struct BinMDataStdExtStringListDriver {
    message_driver: Option<String>,
}

impl BinMDataStdExtStringListDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdExtStringListDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdExtStringListDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
