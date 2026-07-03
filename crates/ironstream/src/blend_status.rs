// FILE: blend_status.rs
// occt: Blend_Status

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendStatus {
    StepTooLarge = 0,
    StepTooSmall = 1,
    Backward = 2,
    SamePoints = 3,
    OnRst1 = 4,
    OnRst2 = 5,
    OnRst12 = 6,
    Ok = 7,
}

impl BlendStatus {
    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(BlendStatus::StepTooLarge),
            1 => Some(BlendStatus::StepTooSmall),
            2 => Some(BlendStatus::Backward),
            3 => Some(BlendStatus::SamePoints),
            4 => Some(BlendStatus::OnRst1),
            5 => Some(BlendStatus::OnRst2),
            6 => Some(BlendStatus::OnRst12),
            7 => Some(BlendStatus::Ok),
            _ => None,
        }
    }

    pub fn to_i32(self) -> i32 {
        self as i32
    }

    pub fn is_ok(self) -> bool {
        self == BlendStatus::Ok
    }

    pub fn is_error(self) -> bool {
        self != BlendStatus::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statuses() {
        assert_eq!(BlendStatus::StepTooLarge.to_i32(), 0);
        assert_eq!(BlendStatus::Ok.to_i32(), 7);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(BlendStatus::from_i32(0), Some(BlendStatus::StepTooLarge));
        assert_eq!(BlendStatus::from_i32(7), Some(BlendStatus::Ok));
        assert_eq!(BlendStatus::from_i32(99), None);
    }

    #[test]
    fn test_is_ok() {
        assert!(BlendStatus::Ok.is_ok());
        assert!(!BlendStatus::StepTooLarge.is_ok());
    }

    #[test]
    fn test_is_error() {
        assert!(BlendStatus::Backward.is_error());
        assert!(!BlendStatus::Ok.is_error());
    }
}
