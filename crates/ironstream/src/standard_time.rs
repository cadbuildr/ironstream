// FILE: standard_time.rs
// occt: Standard_Time

//! Time utilities and type definitions.

use std::time::{SystemTime, UNIX_EPOCH};

/// Standard time value type (seconds since epoch).
pub type StandardTime = u64;

/// Check if two time values are equal.
pub fn is_equal(t1: StandardTime, t2: StandardTime) -> bool {
    t1 == t2
}

/// Get current time as seconds since UNIX epoch.
pub fn current_time() -> StandardTime {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Convert a duration in seconds to StandardTime.
pub const fn time_from_secs(secs: u64) -> StandardTime {
    secs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_equal_same() {
        assert!(is_equal(100, 100));
    }

    #[test]
    fn test_is_equal_different() {
        assert!(!is_equal(100, 200));
    }

    #[test]
    fn test_current_time() {
        let t = current_time();
        assert!(t > 0);
    }

    #[test]
    fn test_time_from_secs() {
        let t = time_from_secs(12345);
        assert_eq!(t, 12345);
    }

    #[test]
    fn test_time_comparison() {
        let t1 = time_from_secs(1000);
        let t2 = time_from_secs(2000);
        assert!(t1 < t2);
    }

    #[test]
    fn test_time_ordering() {
        let t1 = time_from_secs(100);
        let t2 = time_from_secs(200);
        let t3 = time_from_secs(300);

        assert!(t1 < t2);
        assert!(t2 < t3);
        assert!(t1 < t3);
    }
}
