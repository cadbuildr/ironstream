// FILE: open_gl_halton_sampler.rs
// occt: OpenGl_HaltonSampler

/// Halton sequence sampler for uniform quasi-random sampling.
#[derive(Debug)]
pub struct OpenGlHaltonSampler {
    index: u32,
}

impl OpenGlHaltonSampler {
    /// Creates a new Halton sampler.
    pub fn new() -> Self {
        OpenGlHaltonSampler { index: 0 }
    }

    /// Gets the next sample index.
    pub fn next(&mut self) -> u32 {
        let val = self.index;
        self.index += 1;
        val
    }

    /// Resets the sampler.
    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl Default for OpenGlHaltonSampler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halton_sampler_creation() {
        let sampler = OpenGlHaltonSampler::new();
        assert_eq!(sampler.index, 0);
    }

    #[test]
    fn test_halton_sampler_next() {
        let mut sampler = OpenGlHaltonSampler::new();
        assert_eq!(sampler.next(), 0);
        assert_eq!(sampler.next(), 1);
        assert_eq!(sampler.next(), 2);
    }

    #[test]
    fn test_halton_sampler_reset() {
        let mut sampler = OpenGlHaltonSampler::new();
        sampler.next();
        sampler.next();
        sampler.reset();
        assert_eq!(sampler.next(), 0);
    }
}
