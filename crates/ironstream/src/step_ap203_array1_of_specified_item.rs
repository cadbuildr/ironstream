// FILE: step_ap203_array1_of_specified_item.rs
// occt: StepAP203_Array1OfSpecifiedItem

pub struct StepAp203Array1OfSpecifiedItem {
    data: Vec<Option<String>>,
    lower: usize,
    upper: usize,
}

impl StepAp203Array1OfSpecifiedItem {
    pub fn new(lower: usize, upper: usize) -> Self {
        assert!(lower > 0 && upper >= lower);
        StepAp203Array1OfSpecifiedItem {
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
    fn test_basic() {
        let arr = StepAp203Array1OfSpecifiedItem::new(1, 5);
        assert_eq!(arr.len(), 5);
    }
}
