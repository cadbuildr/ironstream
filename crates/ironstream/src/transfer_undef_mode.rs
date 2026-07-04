// FILE: transfer_undef_mode.rs
// occt: Transfer_UndefMode

/// Enumeration defining how undefined/unset values should be handled in transfer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransferUndefMode {
    /// Ignore undefined values
    IgnoreUndefMode = 0,
    /// Keep undefined values as-is
    KeepUndefMode = 1,
    /// Use a default value for undefined
    DefaultMode = 2,
    /// Fail on undefined values
    FailMode = 3,
}

impl Default for TransferUndefMode {
    fn default() -> Self {
        Self::IgnoreUndefMode
    }
}

impl TransferUndefMode {
    /// Returns the name of the mode.
    pub fn name(self) -> &'static str {
        match self {
            Self::IgnoreUndefMode => "IgnoreUndefMode",
            Self::KeepUndefMode => "KeepUndefMode",
            Self::DefaultMode => "DefaultMode",
            Self::FailMode => "FailMode",
        }
    }

    /// Parses a string to an undef mode.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "IgnoreUndefMode" => Some(Self::IgnoreUndefMode),
            "KeepUndefMode" => Some(Self::KeepUndefMode),
            "DefaultMode" => Some(Self::DefaultMode),
            "FailMode" => Some(Self::FailMode),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(TransferUndefMode::default(), TransferUndefMode::IgnoreUndefMode);
    }

    #[test]
    fn test_names() {
        assert_eq!(TransferUndefMode::IgnoreUndefMode.name(), "IgnoreUndefMode");
        assert_eq!(TransferUndefMode::KeepUndefMode.name(), "KeepUndefMode");
        assert_eq!(TransferUndefMode::DefaultMode.name(), "DefaultMode");
        assert_eq!(TransferUndefMode::FailMode.name(), "FailMode");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            TransferUndefMode::from_str("IgnoreUndefMode"),
            Some(TransferUndefMode::IgnoreUndefMode)
        );
        assert_eq!(
            TransferUndefMode::from_str("KeepUndefMode"),
            Some(TransferUndefMode::KeepUndefMode)
        );
        assert_eq!(TransferUndefMode::from_str("InvalidMode"), None);
    }
}
