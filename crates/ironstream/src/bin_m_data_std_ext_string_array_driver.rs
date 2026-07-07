// FILE: bin_m_data_std_ext_string_array_driver.rs
// occt: BinMDataStd_ExtStringArrayDriver

pub struct BinMDataStdExtStringArrayDriver {
    message_driver: Option<String>,
}

impl BinMDataStdExtStringArrayDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdExtStringArrayDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdExtStringArrayDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
