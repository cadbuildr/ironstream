// FILE: osd_signal_mode.rs
// occt: OSD_SignalMode

/// Mode of operation for signal handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsdSignalMode {
    /// Do not set or remove signal handlers
    AsIs = 0,
    /// Set OCCT signal handlers
    Set = 1,
    /// Set OCCT signal handler but only if no handler is set, for each particular signal type
    SetUnhandled = 2,
    /// Unset signal handler to system default
    Unset = 3,
}

impl OsdSignalMode {
    /// Creates OsdSignalMode from integer value
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            0 => Some(OsdSignalMode::AsIs),
            1 => Some(OsdSignalMode::Set),
            2 => Some(OsdSignalMode::SetUnhandled),
            3 => Some(OsdSignalMode::Unset),
            _ => None,
        }
    }

    /// Gets numeric value of mode
    pub fn as_int(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_signal_mode_asis() {
        let mode = OsdSignalMode::AsIs;
        assert_eq!(mode.as_int(), 0);
    }

    #[test]
    fn test_osd_signal_mode_set() {
        let mode = OsdSignalMode::Set;
        assert_eq!(mode.as_int(), 1);
    }

    #[test]
    fn test_osd_signal_mode_set_unhandled() {
        let mode = OsdSignalMode::SetUnhandled;
        assert_eq!(mode.as_int(), 2);
    }

    #[test]
    fn test_osd_signal_mode_unset() {
        let mode = OsdSignalMode::Unset;
        assert_eq!(mode.as_int(), 3);
    }

    #[test]
    fn test_osd_signal_mode_from_int() {
        assert_eq!(OsdSignalMode::from_int(0), Some(OsdSignalMode::AsIs));
        assert_eq!(OsdSignalMode::from_int(1), Some(OsdSignalMode::Set));
        assert_eq!(OsdSignalMode::from_int(2), Some(OsdSignalMode::SetUnhandled));
        assert_eq!(OsdSignalMode::from_int(3), Some(OsdSignalMode::Unset));
        assert_eq!(OsdSignalMode::from_int(4), None);
    }
}
