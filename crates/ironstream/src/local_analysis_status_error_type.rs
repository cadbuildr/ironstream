// FILE: local_analysis_status_error_type.rs
// occt: LocalAnalysis_StatusErrorType

/// Status error types for local analysis
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusErrorType {
    OK,
    NotDone,
    InvalidData,
    ParameterOutOfRange,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_types() {
        let _s1 = StatusErrorType::OK;
        let _s2 = StatusErrorType::NotDone;
        let _s3 = StatusErrorType::InvalidData;
        let _s4 = StatusErrorType::ParameterOutOfRange;
    }
}
