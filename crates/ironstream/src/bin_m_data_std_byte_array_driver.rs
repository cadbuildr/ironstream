// FILE: bin_m_data_std_byte_array_driver.rs
// occt: BinMDataStd_ByteArrayDriver

pub struct BinMDataStdByteArrayDriver {
    message_driver: Option<String>,
}

impl BinMDataStdByteArrayDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdByteArrayDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdByteArrayDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
