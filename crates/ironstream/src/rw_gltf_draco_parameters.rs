// FILE: rw_gltf_draco_parameters.rs
// occt: RWGltf_DracoParameters

//! Draco compression parameters for glTF export.

/// Draco compression parameters
#[derive(Debug, Clone)]
pub struct DracoParameters {
    compression_level: u32,
    quantization_bits_position: u32,
    quantization_bits_normal: u32,
    quantization_bits_texcoord: u32,
}

impl DracoParameters {
    pub fn new() -> Self {
        Self {
            compression_level: 7,
            quantization_bits_position: 14,
            quantization_bits_normal: 10,
            quantization_bits_texcoord: 12,
        }
    }

    pub fn compression_level(&self) -> u32 {
        self.compression_level
    }

    pub fn set_compression_level(&mut self, level: u32) {
        self.compression_level = level.min(10);
    }

    pub fn quantization_bits_position(&self) -> u32 {
        self.quantization_bits_position
    }

    pub fn set_quantization_bits_position(&mut self, bits: u32) {
        self.quantization_bits_position = bits;
    }

    pub fn quantization_bits_normal(&self) -> u32 {
        self.quantization_bits_normal
    }

    pub fn set_quantization_bits_normal(&mut self, bits: u32) {
        self.quantization_bits_normal = bits;
    }

    pub fn quantization_bits_texcoord(&self) -> u32 {
        self.quantization_bits_texcoord
    }

    pub fn set_quantization_bits_texcoord(&mut self, bits: u32) {
        self.quantization_bits_texcoord = bits;
    }
}

impl Default for DracoParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let params = DracoParameters::new();
        assert_eq!(params.compression_level(), 7);
    }

    #[test]
    fn test_set_compression_level() {
        let mut params = DracoParameters::new();
        params.set_compression_level(5);
        assert_eq!(params.compression_level(), 5);
    }

    #[test]
    fn test_compression_level_clamped() {
        let mut params = DracoParameters::new();
        params.set_compression_level(15);
        assert_eq!(params.compression_level(), 10);
    }

    #[test]
    fn test_quantization_bits() {
        let mut params = DracoParameters::new();
        assert_eq!(params.quantization_bits_position(), 14);
        params.set_quantization_bits_position(12);
        assert_eq!(params.quantization_bits_position(), 12);
    }
}
