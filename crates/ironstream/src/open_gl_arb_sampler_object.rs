// FILE: open_gl_arb_sampler_object.rs
// occt: OpenGl_ArbSamplerObject

/// Sampler object extension support.
pub struct OpenGlArbSamplerObject;

impl OpenGlArbSamplerObject {
    pub fn bind_sampler() {}
    pub fn delete_samplers() {}
    pub fn gen_samplers() {}
    pub fn is_sampler() {}
    pub fn sampler_parameteri() {}
    pub fn sampler_parameterf() {}
    pub fn get_sampler_parameteriv() {}
    pub fn get_sampler_parameterfv() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampler_operations() {
        OpenGlArbSamplerObject::gen_samplers();
        OpenGlArbSamplerObject::bind_sampler();
    }
}
