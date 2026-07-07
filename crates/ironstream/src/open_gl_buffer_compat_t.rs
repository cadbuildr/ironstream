// FILE: open_gl_buffer_compat_t.rs
// occt: OpenGl_BufferCompatT

pub struct OpenGlBufferCompatT;

impl OpenGlBufferCompatT {
    pub fn bind() {}
    pub fn unbind() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compat() {
        OpenGlBufferCompatT::bind();
    }
}
