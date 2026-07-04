// FILE: transfer_b_rep_transfer_result_info.rs
// occt: TransferBRep_TransferResultInfo

/// Data structure storing transfer operation result statistics.
/// Tracks counts of results with warnings/failures and no results with warnings/failures.
#[derive(Clone, Debug)]
pub struct TransferBRepTransferResultInfo {
    /// Count of successful transfers with result
    result: u32,
    /// Count of transfers with result and warnings
    result_warning: u32,
    /// Count of transfers with result and failures
    result_fail: u32,
    /// Count of transfers with result and both warnings and failures
    result_warning_fail: u32,
    /// Count of transfers with no result
    no_result: u32,
    /// Count of transfers with no result but warnings
    no_result_warning: u32,
    /// Count of transfers with no result but failures
    no_result_fail: u32,
    /// Count of transfers with no result and both warnings and failures
    no_result_warning_fail: u32,
}

impl TransferBRepTransferResultInfo {
    /// Creates a new transfer result info with all counters at zero.
    pub fn new() -> Self {
        Self {
            result: 0,
            result_warning: 0,
            result_fail: 0,
            result_warning_fail: 0,
            no_result: 0,
            no_result_warning: 0,
            no_result_fail: 0,
            no_result_warning_fail: 0,
        }
    }

    /// Clears all counters.
    pub fn clear(&mut self) {
        self.result = 0;
        self.result_warning = 0;
        self.result_fail = 0;
        self.result_warning_fail = 0;
        self.no_result = 0;
        self.no_result_warning = 0;
        self.no_result_fail = 0;
        self.no_result_warning_fail = 0;
    }

    /// Gets the result counter.
    pub fn result(&self) -> u32 {
        self.result
    }

    /// Sets the result counter.
    pub fn set_result(&mut self, count: u32) {
        self.result = count;
    }

    /// Gets the result with warning counter.
    pub fn result_warning(&self) -> u32 {
        self.result_warning
    }

    /// Sets the result with warning counter.
    pub fn set_result_warning(&mut self, count: u32) {
        self.result_warning = count;
    }

    /// Gets the result with fail counter.
    pub fn result_fail(&self) -> u32 {
        self.result_fail
    }

    /// Sets the result with fail counter.
    pub fn set_result_fail(&mut self, count: u32) {
        self.result_fail = count;
    }

    /// Gets the result with warning and fail counter.
    pub fn result_warning_fail(&self) -> u32 {
        self.result_warning_fail
    }

    /// Sets the result with warning and fail counter.
    pub fn set_result_warning_fail(&mut self, count: u32) {
        self.result_warning_fail = count;
    }

    /// Gets the no result counter.
    pub fn no_result(&self) -> u32 {
        self.no_result
    }

    /// Sets the no result counter.
    pub fn set_no_result(&mut self, count: u32) {
        self.no_result = count;
    }

    /// Gets the no result with warning counter.
    pub fn no_result_warning(&self) -> u32 {
        self.no_result_warning
    }

    /// Sets the no result with warning counter.
    pub fn set_no_result_warning(&mut self, count: u32) {
        self.no_result_warning = count;
    }

    /// Gets the no result with fail counter.
    pub fn no_result_fail(&self) -> u32 {
        self.no_result_fail
    }

    /// Sets the no result with fail counter.
    pub fn set_no_result_fail(&mut self, count: u32) {
        self.no_result_fail = count;
    }

    /// Gets the no result with warning and fail counter.
    pub fn no_result_warning_fail(&self) -> u32 {
        self.no_result_warning_fail
    }

    /// Sets the no result with warning and fail counter.
    pub fn set_no_result_warning_fail(&mut self, count: u32) {
        self.no_result_warning_fail = count;
    }

    /// Returns the total count of all transfers.
    pub fn total(&self) -> u32 {
        self.result
            + self.result_warning
            + self.result_fail
            + self.result_warning_fail
            + self.no_result
            + self.no_result_warning
            + self.no_result_fail
            + self.no_result_warning_fail
    }
}

impl Default for TransferBRepTransferResultInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let info = TransferBRepTransferResultInfo::new();
        assert_eq!(info.result(), 0);
        assert_eq!(info.result_warning(), 0);
        assert_eq!(info.total(), 0);
    }

    #[test]
    fn test_setters() {
        let mut info = TransferBRepTransferResultInfo::new();
        info.set_result(5);
        info.set_result_warning(3);
        info.set_result_fail(2);

        assert_eq!(info.result(), 5);
        assert_eq!(info.result_warning(), 3);
        assert_eq!(info.result_fail(), 2);
    }

    #[test]
    fn test_clear() {
        let mut info = TransferBRepTransferResultInfo::new();
        info.set_result(10);
        info.set_no_result(5);
        assert_eq!(info.total(), 15);

        info.clear();
        assert_eq!(info.total(), 0);
        assert_eq!(info.result(), 0);
        assert_eq!(info.no_result(), 0);
    }

    #[test]
    fn test_total() {
        let mut info = TransferBRepTransferResultInfo::new();
        info.set_result(5);
        info.set_result_warning(3);
        info.set_no_result(2);
        info.set_no_result_fail(1);

        assert_eq!(info.total(), 11);
    }

    #[test]
    fn test_all_counters() {
        let mut info = TransferBRepTransferResultInfo::new();
        info.set_result(1);
        info.set_result_warning(2);
        info.set_result_fail(3);
        info.set_result_warning_fail(4);
        info.set_no_result(5);
        info.set_no_result_warning(6);
        info.set_no_result_fail(7);
        info.set_no_result_warning_fail(8);

        assert_eq!(info.result(), 1);
        assert_eq!(info.result_warning(), 2);
        assert_eq!(info.result_fail(), 3);
        assert_eq!(info.result_warning_fail(), 4);
        assert_eq!(info.no_result(), 5);
        assert_eq!(info.no_result_warning(), 6);
        assert_eq!(info.no_result_fail(), 7);
        assert_eq!(info.no_result_warning_fail(), 8);
        assert_eq!(info.total(), 36);
    }
}
