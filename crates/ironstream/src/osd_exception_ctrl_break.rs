// FILE: osd_exception_ctrl_break.rs
// occt: OSD_Exception_CTRL_BREAK

/// Ctrl+C break exception.
#[derive(Debug, Clone)]
pub struct CtrlBreakException;

impl std::fmt::Display for CtrlBreakException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Ctrl+Break")
    }
}

impl std::error::Error for CtrlBreakException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctrl_break() {
        let exc = CtrlBreakException;
        let s = format!("{}", exc);
        assert_eq!(s, "Ctrl+Break");
    }
}
