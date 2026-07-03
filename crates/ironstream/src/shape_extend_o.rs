// FILE: shape_extend_o.rs
// occt: ShapeExtend

/// Shape extension utilities.
pub struct ShapeExtend;

impl ShapeExtend {
    /// Initialize ShapeExtend.
    pub fn init() {
        // Implementation stub
    }

    /// Encode status to bit flag.
    pub fn encode_status(status: u32) -> i32 {
        1 << status as i32
    }

    /// Decode status from bit flag.
    pub fn decode_status(flag: i32, status: u32) -> bool {
        (flag & (1 << status as i32)) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        ShapeExtend::init();
    }

    #[test]
    fn test_encode_status() {
        let encoded = ShapeExtend::encode_status(0);
        assert_eq!(encoded, 1);
    }

    #[test]
    fn test_decode_status() {
        let flag = ShapeExtend::encode_status(2);
        assert!(ShapeExtend::decode_status(flag, 2));
        assert!(!ShapeExtend::decode_status(flag, 1));
    }
}
