// FILE: step_basic_person.rs
// occt: StepBasic_Person

/// Represents a STEP Person entity with ID, optional LastName, FirstName, MiddleNames, PrefixTitles, and SuffixTitles.
#[derive(Clone, Debug)]
pub struct StepBasicPerson {
    id: String,
    last_name: Option<String>,
    has_last_name: bool,
    first_name: Option<String>,
    has_first_name: bool,
    middle_names: Vec<String>,
    has_middle_names: bool,
    prefix_titles: Vec<String>,
    has_prefix_titles: bool,
    suffix_titles: Vec<String>,
    has_suffix_titles: bool,
}

impl StepBasicPerson {
    /// Create a new empty StepBasicPerson.
    pub fn new() -> Self {
        StepBasicPerson {
            id: String::new(),
            last_name: None,
            has_last_name: false,
            first_name: None,
            has_first_name: false,
            middle_names: Vec::new(),
            has_middle_names: false,
            prefix_titles: Vec::new(),
            has_prefix_titles: false,
            suffix_titles: Vec::new(),
            has_suffix_titles: false,
        }
    }

    /// Initialize all fields.
    #[allow(clippy::too_many_arguments)]
    pub fn init(
        &mut self,
        id: String,
        has_last_name: bool,
        last_name: Option<String>,
        has_first_name: bool,
        first_name: Option<String>,
        has_middle_names: bool,
        middle_names: Vec<String>,
        has_prefix_titles: bool,
        prefix_titles: Vec<String>,
        has_suffix_titles: bool,
        suffix_titles: Vec<String>,
    ) {
        self.id = id;
        self.has_last_name = has_last_name;
        self.last_name = if has_last_name { last_name } else { None };
        self.has_first_name = has_first_name;
        self.first_name = if has_first_name { first_name } else { None };
        self.has_middle_names = has_middle_names;
        self.middle_names = if has_middle_names { middle_names } else { Vec::new() };
        self.has_prefix_titles = has_prefix_titles;
        self.prefix_titles = if has_prefix_titles { prefix_titles } else { Vec::new() };
        self.has_suffix_titles = has_suffix_titles;
        self.suffix_titles = if has_suffix_titles { suffix_titles } else { Vec::new() };
    }

    /// Returns the ID field.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Set the ID field.
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    /// Returns the LastName field.
    pub fn last_name(&self) -> Option<&str> {
        self.last_name.as_deref()
    }

    /// Set the LastName field.
    pub fn set_last_name(&mut self, last_name: String) {
        self.last_name = Some(last_name);
        self.has_last_name = true;
    }

    /// Unset the LastName field.
    pub fn unset_last_name(&mut self) {
        self.last_name = None;
        self.has_last_name = false;
    }

    /// Returns whether LastName is defined.
    pub fn has_last_name(&self) -> bool {
        self.has_last_name
    }

    /// Returns the FirstName field.
    pub fn first_name(&self) -> Option<&str> {
        self.first_name.as_deref()
    }

    /// Set the FirstName field.
    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = Some(first_name);
        self.has_first_name = true;
    }

    /// Unset the FirstName field.
    pub fn unset_first_name(&mut self) {
        self.first_name = None;
        self.has_first_name = false;
    }

    /// Returns whether FirstName is defined.
    pub fn has_first_name(&self) -> bool {
        self.has_first_name
    }

    /// Returns the MiddleNames list.
    pub fn middle_names(&self) -> &[String] {
        &self.middle_names
    }

    /// Set the MiddleNames list.
    pub fn set_middle_names(&mut self, names: Vec<String>) {
        self.middle_names = names;
        self.has_middle_names = true;
    }

    /// Unset the MiddleNames.
    pub fn unset_middle_names(&mut self) {
        self.middle_names.clear();
        self.has_middle_names = false;
    }

    /// Returns whether MiddleNames is defined.
    pub fn has_middle_names(&self) -> bool {
        self.has_middle_names
    }

    /// Returns a specific middle name by index.
    pub fn middle_names_value(&self, index: usize) -> Option<&str> {
        self.middle_names.get(index).map(|s| s.as_str())
    }

    /// Returns the number of middle names.
    pub fn nb_middle_names(&self) -> usize {
        self.middle_names.len()
    }

    /// Returns the PrefixTitles list.
    pub fn prefix_titles(&self) -> &[String] {
        &self.prefix_titles
    }

    /// Set the PrefixTitles list.
    pub fn set_prefix_titles(&mut self, titles: Vec<String>) {
        self.prefix_titles = titles;
        self.has_prefix_titles = true;
    }

    /// Unset the PrefixTitles.
    pub fn unset_prefix_titles(&mut self) {
        self.prefix_titles.clear();
        self.has_prefix_titles = false;
    }

    /// Returns whether PrefixTitles is defined.
    pub fn has_prefix_titles(&self) -> bool {
        self.has_prefix_titles
    }

    /// Returns a specific prefix title by index.
    pub fn prefix_titles_value(&self, index: usize) -> Option<&str> {
        self.prefix_titles.get(index).map(|s| s.as_str())
    }

    /// Returns the number of prefix titles.
    pub fn nb_prefix_titles(&self) -> usize {
        self.prefix_titles.len()
    }

    /// Returns the SuffixTitles list.
    pub fn suffix_titles(&self) -> &[String] {
        &self.suffix_titles
    }

    /// Set the SuffixTitles list.
    pub fn set_suffix_titles(&mut self, titles: Vec<String>) {
        self.suffix_titles = titles;
        self.has_suffix_titles = true;
    }

    /// Unset the SuffixTitles.
    pub fn unset_suffix_titles(&mut self) {
        self.suffix_titles.clear();
        self.has_suffix_titles = false;
    }

    /// Returns whether SuffixTitles is defined.
    pub fn has_suffix_titles(&self) -> bool {
        self.has_suffix_titles
    }

    /// Returns a specific suffix title by index.
    pub fn suffix_titles_value(&self, index: usize) -> Option<&str> {
        self.suffix_titles.get(index).map(|s| s.as_str())
    }

    /// Returns the number of suffix titles.
    pub fn nb_suffix_titles(&self) -> usize {
        self.suffix_titles.len()
    }
}

impl Default for StepBasicPerson {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let person = StepBasicPerson::new();
        assert_eq!(person.id(), "");
        assert_eq!(person.last_name(), None);
        assert_eq!(person.nb_middle_names(), 0);
    }

    #[test]
    fn test_init_full() {
        let mut person = StepBasicPerson::new();
        person.init(
            "P-001".to_string(),
            true,
            Some("Smith".to_string()),
            true,
            Some("John".to_string()),
            true,
            vec!["Paul".to_string()],
            true,
            vec!["Dr.".to_string()],
            true,
            vec!["Jr.".to_string()],
        );

        assert_eq!(person.id(), "P-001");
        assert_eq!(person.last_name(), Some("Smith"));
        assert_eq!(person.first_name(), Some("John"));
        assert_eq!(person.nb_middle_names(), 1);
        assert_eq!(person.middle_names_value(0), Some("Paul"));
        assert_eq!(person.nb_prefix_titles(), 1);
        assert_eq!(person.prefix_titles_value(0), Some("Dr."));
        assert_eq!(person.nb_suffix_titles(), 1);
        assert_eq!(person.suffix_titles_value(0), Some("Jr."));
    }

    #[test]
    fn test_init_partial() {
        let mut person = StepBasicPerson::new();
        person.init(
            "P-002".to_string(),
            false,
            Some("ignored".to_string()),
            true,
            Some("Jane".to_string()),
            false,
            vec![],
            false,
            vec![],
            false,
            vec![],
        );

        assert_eq!(person.last_name(), None);
        assert!(!person.has_last_name());
        assert_eq!(person.first_name(), Some("Jane"));
        assert_eq!(person.nb_middle_names(), 0);
    }

    #[test]
    fn test_setters() {
        let mut person = StepBasicPerson::new();
        person.set_id("P-003".to_string());
        person.set_last_name("Doe".to_string());
        person.set_first_name("Bob".to_string());
        person.set_middle_names(vec!["Michael".to_string()]);
        person.set_prefix_titles(vec!["Prof.".to_string()]);
        person.set_suffix_titles(vec!["Sr.".to_string()]);

        assert_eq!(person.id(), "P-003");
        assert_eq!(person.last_name(), Some("Doe"));
        assert_eq!(person.first_name(), Some("Bob"));
        assert_eq!(person.nb_middle_names(), 1);
        assert_eq!(person.nb_prefix_titles(), 1);
        assert_eq!(person.nb_suffix_titles(), 1);
    }

    #[test]
    fn test_unset() {
        let mut person = StepBasicPerson::new();
        person.set_last_name("Smith".to_string());
        assert!(person.has_last_name());
        person.unset_last_name();
        assert!(!person.has_last_name());
        assert_eq!(person.last_name(), None);
    }
}
