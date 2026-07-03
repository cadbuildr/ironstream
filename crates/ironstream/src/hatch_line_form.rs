// FILE: hatch_line_form.rs
// occt: Hatch_LineForm

/// Form of a trimmed line in hatch operations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HatchLineForm {
    /// X-axis line (horizontal)
    XLine = 0,
    /// Y-axis line (vertical)
    YLine = 1,
    /// Any arbitrary line
    AnyLine = 2,
}

impl HatchLineForm {
    /// Convert from integer representation
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(HatchLineForm::XLine),
            1 => Some(HatchLineForm::YLine),
            2 => Some(HatchLineForm::AnyLine),
            _ => None,
        }
    }

    /// Convert to integer representation
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_form_values() {
        assert_eq!(HatchLineForm::XLine as i32, 0);
        assert_eq!(HatchLineForm::YLine as i32, 1);
        assert_eq!(HatchLineForm::AnyLine as i32, 2);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(HatchLineForm::from_i32(0), Some(HatchLineForm::XLine));
        assert_eq!(HatchLineForm::from_i32(1), Some(HatchLineForm::YLine));
        assert_eq!(HatchLineForm::from_i32(2), Some(HatchLineForm::AnyLine));
        assert_eq!(HatchLineForm::from_i32(3), None);
    }

    #[test]
    fn test_as_i32() {
        assert_eq!(HatchLineForm::XLine.as_i32(), 0);
        assert_eq!(HatchLineForm::YLine.as_i32(), 1);
        assert_eq!(HatchLineForm::AnyLine.as_i32(), 2);
    }

    #[test]
    fn test_clone_copy() {
        let form = HatchLineForm::AnyLine;
        let form2 = form;
        assert_eq!(form, form2);
    }
}
