// FILE: step_repr_make_from_usage_option.rs
// occt: StepRepr_MakeFromUsageOption

/// StepRepr_MakeFromUsageOption: Representation of STEP entity MakeFromUsageOption
/// Inherits from StepRepr_ProductDefinitionUsage
#[derive(Clone, Debug)]
pub struct StepReprMakeFromUsageOption {
    id: String,
    name: String,
    description: Option<String>,
    ranking: i32,
    ranking_rationale: String,
    quantity: String, // Simplified: storing identifier
}

impl StepReprMakeFromUsageOption {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprMakeFromUsageOption {
            id: String::new(),
            name: String::new(),
            description: None,
            ranking: 0,
            ranking_rationale: String::new(),
            quantity: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        id: String,
        name: String,
        has_description: bool,
        description: Option<String>,
        ranking: i32,
        ranking_rationale: String,
        quantity: String,
    ) {
        self.id = id;
        self.name = name;
        self.description = if has_description { description } else { None };
        self.ranking = ranking;
        self.ranking_rationale = ranking_rationale;
        self.quantity = quantity;
    }

    /// Returns field Ranking
    pub fn ranking(&self) -> i32 {
        self.ranking
    }

    /// Set field Ranking
    pub fn set_ranking(&mut self, ranking: i32) {
        self.ranking = ranking;
    }

    /// Returns field RankingRationale
    pub fn ranking_rationale(&self) -> &str {
        &self.ranking_rationale
    }

    /// Set field RankingRationale
    pub fn set_ranking_rationale(&mut self, rationale: String) {
        self.ranking_rationale = rationale;
    }

    /// Returns field Quantity
    pub fn quantity(&self) -> &str {
        &self.quantity
    }

    /// Set field Quantity
    pub fn set_quantity(&mut self, quantity: String) {
        self.quantity = quantity;
    }

    /// Get id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl Default for StepReprMakeFromUsageOption {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let mfuo = StepReprMakeFromUsageOption::new();
        assert_eq!(mfuo.id(), "");
        assert_eq!(mfuo.name(), "");
        assert_eq!(mfuo.ranking(), 0);
        assert_eq!(mfuo.ranking_rationale(), "");
        assert_eq!(mfuo.quantity(), "");
    }

    #[test]
    fn test_init() {
        let mut mfuo = StepReprMakeFromUsageOption::new();
        mfuo.init(
            "id1".to_string(),
            "name1".to_string(),
            true,
            Some("desc1".to_string()),
            5,
            "rationale".to_string(),
            "qty".to_string(),
        );
        assert_eq!(mfuo.id(), "id1");
        assert_eq!(mfuo.name(), "name1");
        assert_eq!(mfuo.ranking(), 5);
        assert_eq!(mfuo.ranking_rationale(), "rationale");
        assert_eq!(mfuo.quantity(), "qty");
    }

    #[test]
    fn test_set_ranking() {
        let mut mfuo = StepReprMakeFromUsageOption::new();
        mfuo.set_ranking(10);
        assert_eq!(mfuo.ranking(), 10);
    }

    #[test]
    fn test_set_ranking_rationale() {
        let mut mfuo = StepReprMakeFromUsageOption::new();
        mfuo.set_ranking_rationale("new_rationale".to_string());
        assert_eq!(mfuo.ranking_rationale(), "new_rationale");
    }

    #[test]
    fn test_set_quantity() {
        let mut mfuo = StepReprMakeFromUsageOption::new();
        mfuo.set_quantity("new_qty".to_string());
        assert_eq!(mfuo.quantity(), "new_qty");
    }
}
