// FILE: step_data_select_arr_real.rs
// occt: StepData_SelectArrReal

use crate::step_data_select_named::StepDataSelectNamed;

const KIND_ARRREAL: i32 = 100;

//! A SelectArrReal is a SelectNamed for array of reals
pub struct StepDataSelectArrReal {
    base: StepDataSelectNamed,
    arr: Vec<f64>,
}

impl StepDataSelectArrReal {
    //! Creates a SelectArrReal
    pub fn new() -> Self {
        StepDataSelectArrReal {
            base: StepDataSelectNamed::new(),
            arr: Vec::new(),
        }
    }

    //! Returns the kind
    pub fn kind(&self) -> i32 {
        KIND_ARRREAL
    }

    //! Returns the array of reals
    pub fn arr_real(&self) -> &[f64] {
        &self.arr
    }

    //! Sets the array of reals
    pub fn set_arr_real(&mut self, arr: Vec<f64>) {
        self.arr = arr;
    }

    //! Returns the base SelectNamed
    pub fn base(&self) -> &StepDataSelectNamed {
        &self.base
    }

    //! Returns mutable base
    pub fn base_mut(&mut self) -> &mut StepDataSelectNamed {
        &mut self.base
    }
}

impl Default for StepDataSelectArrReal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_arr_real_new() {
        let sel = StepDataSelectArrReal::new();
        assert_eq!(sel.kind(), KIND_ARRREAL);
        assert_eq!(sel.arr_real().len(), 0);
    }

    #[test]
    fn test_set_arr_real() {
        let mut sel = StepDataSelectArrReal::new();
        let arr = vec![1.0, 2.0, 3.14];
        sel.set_arr_real(arr);
        assert_eq!(sel.arr_real().len(), 3);
        assert!((sel.arr_real()[2] - 3.14).abs() < 1e-10);
    }
}
