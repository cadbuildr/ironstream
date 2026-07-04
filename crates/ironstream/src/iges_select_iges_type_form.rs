// FILE: iges_select_iges_type_form.rs
// occt: IGESSelect_IGESTypeForm

pub struct IGESSelectIGESTypeForm;

impl IGESSelectIGESTypeForm {
    pub fn new() -> Self {
        IGESSelectIGESTypeForm
    }
}

impl Default for IGESSelectIGESTypeForm {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectIGESTypeForm::new();
    }
}
