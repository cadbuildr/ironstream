// FILE: open_gl_arb_tex_bindless.rs
// occt: OpenGl_ArbTexBindless

/// Bindless texture extension support.
pub struct OpenGlArbTexBindless;

impl OpenGlArbTexBindless {
    pub fn get_texture_handle_arb() {}
    pub fn get_texture_sampler_handle_arb() {}
    pub fn make_texture_handle_resident_arb() {}
    pub fn make_texture_handle_non_resident_arb() {}
    pub fn is_texture_handle_resident_arb() {}
    pub fn get_image_handle_arb() {}
    pub fn make_image_handle_resident_arb() {}
    pub fn make_image_handle_non_resident_arb() {}
    pub fn is_image_handle_resident_arb() {}
    pub fn uniform_handle_ui64_arb() {}
    pub fn uniform_handle_ui64v_arb() {}
    pub fn program_uniform_handle_ui64_arb() {}
    pub fn program_uniform_handle_ui64v_arb() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bindless_texture() {
        OpenGlArbTexBindless::get_texture_handle_arb();
    }
}
