// FILE: bin_m_data_std_int_packed_map_driver.rs
// occt: BinMDataStd_IntPackedMapDriver

pub struct BinMDataStdIntPackedMapDriver {
    message_driver: Option<String>,
}

impl BinMDataStdIntPackedMapDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdIntPackedMapDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdIntPackedMapDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
