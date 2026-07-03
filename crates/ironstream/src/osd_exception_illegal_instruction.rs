// FILE: osd_exception_illegal_instruction.rs
// occt: OSD_Exception_ILLEGAL_INSTRUCTION

/// Illegal instruction exception.
#[derive(Debug, Clone)]
pub struct IllegalInstructionException;

impl std::fmt::Display for IllegalInstructionException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Illegal Instruction")
    }
}

impl std::error::Error for IllegalInstructionException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_illegal_instruction() {
        let exc = IllegalInstructionException;
        let s = format!("{}", exc);
        assert_eq!(s, "Illegal Instruction");
    }
}
