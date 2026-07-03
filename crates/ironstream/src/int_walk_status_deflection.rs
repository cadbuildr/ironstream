// FILE: int_walk_status_deflection.rs
// occt: IntWalk_StatusDeflection

/// Status of deflection during surface walking
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusDeflection {
    OK,
    TooLarge,
    SmallDeflection,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let _s1 = StatusDeflection::OK;
        let _s2 = StatusDeflection::TooLarge;
        let _s3 = StatusDeflection::SmallDeflection;
        assert_ne!(StatusDeflection::OK, StatusDeflection::TooLarge);
    }
}
