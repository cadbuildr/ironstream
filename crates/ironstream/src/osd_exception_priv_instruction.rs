// FILE: osd_exception_priv_instruction.rs
// occt: OSD_Exception_PRIV_INSTRUCTION

/// Privileged instruction exception.
#[derive(Debug, Clone)]
pub struct PrivInstructionException;

impl std::fmt::Display for PrivInstructionException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Privileged Instruction")
    }
}

impl std::error::Error for PrivInstructionException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priv_instruction() {
        let exc = PrivInstructionException;
        let s = format!("{}", exc);
        assert_eq!(s, "Privileged Instruction");
    }
}
