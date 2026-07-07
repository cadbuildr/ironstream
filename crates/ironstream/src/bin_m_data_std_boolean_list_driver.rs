// FILE: bin_m_data_std_boolean_list_driver.rs
// occt: BinMDataStd_BooleanListDriver

pub struct BinMDataStdBooleanListDriver {
    message_driver: Option<String>,
}

impl BinMDataStdBooleanListDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdBooleanListDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdBooleanListDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
