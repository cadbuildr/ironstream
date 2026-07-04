// FILE: step_ap203_cc_design_contract.rs
// occt: StepAP203_CcDesignContract

/// CC Design Contract for STEP AP203
pub struct StepAP203_CcDesignContract {
    contract_id: i32,
}

impl StepAP203_CcDesignContract {
    pub fn new() -> Self {
        StepAP203_CcDesignContract { contract_id: 0 }
    }

    pub fn set_contract_id(&mut self, id: i32) {
        self.contract_id = id;
    }

    pub fn get_contract_id(&self) -> i32 {
        self.contract_id
    }
}

impl Default for StepAP203_CcDesignContract {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let contract = StepAP203_CcDesignContract::new();
        assert_eq!(contract.get_contract_id(), 0);
    }
}
