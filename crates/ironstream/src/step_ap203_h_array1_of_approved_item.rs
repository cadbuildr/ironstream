// FILE: step_ap203_h_array1_of_approved_item.rs
// occt: StepAP203_HArray1OfApprovedItem

/// Handle (reference-counted) version of the array.
/// Corresponds to opencascade::handle<NCollection_Array1<StepAP203_ApprovedItem>>
pub struct StepAp203HArray1OfApprovedItem {
    data: Vec<Option<String>>,
    lower: usize,
    upper: usize,
}

impl StepAp203HArray1OfApprovedItem {
    pub fn new(lower: usize, upper: usize) -> Self {
        assert!(lower > 0 && upper >= lower);
        StepAp203HArray1OfApprovedItem {
            data: vec![None; upper + 1],
            lower,
            upper,
        }
    }

    pub fn value(&self, index: usize) -> Option<&String> {
        if index >= self.lower && index <= self.upper {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    pub fn set_value(&mut self, index: usize, value: String) -> bool {
        if index >= self.lower && index <= self.upper {
            self.data[index] = Some(value);
            true
        } else {
            false
        }
    }

    pub fn lower(&self) -> usize {
        self.lower
    }

    pub fn upper(&self) -> usize {
        self.upper
    }

    pub fn len(&self) -> usize {
        if self.upper >= self.lower {
            self.upper - self.lower + 1
        } else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.upper < self.lower
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_array() {
        let mut arr = StepAp203HArray1OfApprovedItem::new(1, 3);
        assert_eq!(arr.len(), 3);
        arr.set_value(1, "h1".to_string());
        assert_eq!(arr.value(1), Some(&"h1".to_string()));
    }
}
