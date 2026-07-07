// FILE: open_gl_arb_tbo.rs
// occt: OpenGl_ArbTBO

/// Texture buffer object extension support.
pub struct OpenGlArbTbo;

impl OpenGlArbTbo {
    pub fn tex_buffer() {}
    pub fn tex_buffer_range() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tex_buffer() {
        OpenGlArbTbo::tex_buffer();
        OpenGlArbTbo::tex_buffer_range();
    }
}
