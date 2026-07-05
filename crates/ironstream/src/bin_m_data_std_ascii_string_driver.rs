// FILE: bin_m_data_std_ascii_string_driver.rs
// occt: BinMDataStd_AsciiStringDriver

pub struct BinMDataStdAsciiStringDriver {
    message_driver: Option<String>,
}

impl BinMDataStdAsciiStringDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdAsciiStringDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdAsciiStringDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
