// FILE: bin_m_data_std_expression_driver.rs
// occt: BinMDataStd_ExpressionDriver

pub struct BinMDataStdExpressionDriver {
    message_driver: Option<String>,
}

impl BinMDataStdExpressionDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdExpressionDriver { message_driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinMDataStdExpressionDriver::new(None);
        assert_eq!(driver.message_driver, None);
    }
}
