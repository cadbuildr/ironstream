// FILE: open_gl_shader_program_dump_level.rs
// occt: OpenGl_ShaderProgramDumpLevel

/// Debug dump level for shader programs.
#[derive(Debug, Clone, Copy)]
pub enum OpenGlShaderProgramDumpLevel {
    Short = 0,
    Full = 1,
}

impl OpenGlShaderProgramDumpLevel {
    pub fn is_full(&self) -> bool {
        matches!(self, OpenGlShaderProgramDumpLevel::Full)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dump_level() {
        let short = OpenGlShaderProgramDumpLevel::Short;
        let full = OpenGlShaderProgramDumpLevel::Full;
        assert!(!short.is_full());
        assert!(full.is_full());
    }
}
