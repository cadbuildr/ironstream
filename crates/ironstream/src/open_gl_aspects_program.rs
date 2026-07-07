// FILE: open_gl_aspects_program.rs
// occt: OpenGl_AspectsProgram

/// OpenGl_AspectsProgram manages shader program aspects.
pub struct OpenGlAspectsProgram {
    program_id: u32,
}

impl OpenGlAspectsProgram {
    pub fn new(program_id: u32) -> Self {
        OpenGlAspectsProgram { program_id }
    }

    pub fn program_id(&self) -> u32 {
        self.program_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_creation() {
        let program = OpenGlAspectsProgram::new(1);
        assert_eq!(program.program_id(), 1);
    }
}
