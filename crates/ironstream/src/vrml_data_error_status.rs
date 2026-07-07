// FILE: vrml_data_error_status.rs
// occt: VrmlData_ErrorStatus

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VrmlDataErrorStatus {
    Ok = 0,
    MemoryError = 1,
    ReadError = 2,
    WriteError = 3,
}

impl VrmlDataErrorStatus {
    pub fn is_ok(self) -> bool {
        self == VrmlDataErrorStatus::Ok
    }

    pub fn is_error(self) -> bool {
        self != VrmlDataErrorStatus::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let status = VrmlDataErrorStatus::Ok;
        assert!(status.is_ok());
        assert!(!status.is_error());
    }

    #[test]
    fn test_error() {
        let status = VrmlDataErrorStatus::ReadError;
        assert!(!status.is_ok());
        assert!(status.is_error());
    }
}
