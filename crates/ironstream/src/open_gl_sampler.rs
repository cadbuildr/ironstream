// FILE: open_gl_sampler.rs
// occt: OpenGl_Sampler

/// OpenGL texture sampler object.
#[derive(Debug, Clone)]
pub struct OpenGlSampler {
    filter: u32,
}

impl OpenGlSampler {
    pub fn new() -> Self {
        OpenGlSampler { filter: 0 }
    }

    pub fn filter(&self) -> u32 {
        self.filter
    }

    pub fn set_filter(&mut self, f: u32) {
        self.filter = f;
    }
}

impl Default for OpenGlSampler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampler() {
        let mut s = OpenGlSampler::new();
        s.set_filter(1);
        assert_eq!(s.filter(), 1);
    }
}
