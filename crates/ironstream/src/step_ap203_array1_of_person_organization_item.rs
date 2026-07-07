// FILE: step_ap203_array1_of_person_organization_item.rs
// occt: StepAP203_Array1OfPersonOrganizationItem

pub struct StepAp203Array1OfPersonOrganizationItem {
    data: Vec<Option<String>>,
    lower: usize,
    upper: usize,
}

impl StepAp203Array1OfPersonOrganizationItem {
    pub fn new(lower: usize, upper: usize) -> Self {
        // OCCT NCollection_Array1 accepts an arbitrary lower bound (0 included);
        // only upper >= lower is required for a non-empty array.
        assert!(upper >= lower);
        StepAp203Array1OfPersonOrganizationItem {
            data: vec![None; upper - lower + 1],
            lower,
            upper,
        }
    }

    pub fn value(&self, index: usize) -> Option<&String> {
        if index >= self.lower && index <= self.upper {
            self.data[index - self.lower].as_ref()
        } else {
            None
        }
    }

    pub fn set_value(&mut self, index: usize, value: String) -> bool {
        if index >= self.lower && index <= self.upper {
            self.data[index - self.lower] = Some(value);
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
    fn test_basic() {
        let arr = StepAp203Array1OfPersonOrganizationItem::new(0, 10);
        assert!(!arr.is_empty());
    }
}
