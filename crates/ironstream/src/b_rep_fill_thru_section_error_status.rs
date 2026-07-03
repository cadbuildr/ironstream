// FILE: b_rep_fill_thru_section_error_status.rs
// occt: BRepFill_ThruSectionErrorStatus

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BRepFillThruSectionErrorStatus {
    Done = 0,
    NotDone = 1,
    NotSameTopology = 2,
    ProfilesInconsistent = 3,
    WrongUsage = 4,
    Null3DCurve = 5,
    Failed = 6,
}

impl Default for BRepFillThruSectionErrorStatus {
    fn default() -> Self {
        BRepFillThruSectionErrorStatus::NotDone
    }
}

impl BRepFillThruSectionErrorStatus {
    pub fn is_done(&self) -> bool {
        matches!(self, BRepFillThruSectionErrorStatus::Done)
    }

    pub fn is_error(&self) -> bool {
        !self.is_done()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_values() {
        assert_eq!(BRepFillThruSectionErrorStatus::Done as u32, 0);
        assert_eq!(BRepFillThruSectionErrorStatus::NotDone as u32, 1);
        assert_eq!(BRepFillThruSectionErrorStatus::Failed as u32, 6);
    }

    #[test]
    fn test_is_done() {
        assert!(BRepFillThruSectionErrorStatus::Done.is_done());
        assert!(!BRepFillThruSectionErrorStatus::NotDone.is_done());
        assert!(!BRepFillThruSectionErrorStatus::Failed.is_done());
    }

    #[test]
    fn test_is_error() {
        assert!(!BRepFillThruSectionErrorStatus::Done.is_error());
        assert!(BRepFillThruSectionErrorStatus::NotDone.is_error());
        assert!(BRepFillThruSectionErrorStatus::Failed.is_error());
    }

    #[test]
    fn test_default_status() {
        assert_eq!(BRepFillThruSectionErrorStatus::default(), BRepFillThruSectionErrorStatus::NotDone);
    }
}
