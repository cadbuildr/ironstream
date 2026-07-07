// FILE: open_gl_arb_ins.rs
// occt: OpenGl_ArbIns

/// Instanced rendering extension support.
pub struct OpenGlArbIns;

impl OpenGlArbIns {
    pub fn draw_arrays_instanced() {}
    pub fn draw_elements_instanced() {}
    pub fn vertex_attrib_divisor() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instanced_drawing() {
        OpenGlArbIns::draw_arrays_instanced();
        OpenGlArbIns::draw_elements_instanced();
    }
}
