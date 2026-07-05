// FILE: qa_bugs.rs
// occt: QABugs

//! QA test package for reproducing known bugs and verifying fixes.

/// QA bug test manager
#[derive(Debug)]
pub struct QABugsManager {
    tests: Vec<BugTest>,
}

/// Individual bug test
#[derive(Debug, Clone)]
pub struct BugTest {
    id: String,
    description: String,
    fixed: bool,
}

impl BugTest {
    pub fn new(id: String, description: String) -> Self {
        Self {
            id,
            description,
            fixed: false,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_fixed(&self) -> bool {
        self.fixed
    }

    pub fn set_fixed(&mut self, fixed: bool) {
        self.fixed = fixed;
    }
}

impl QABugsManager {
    pub fn new() -> Self {
        Self { tests: Vec::new() }
    }

    pub fn add_test(&mut self, test: BugTest) {
        self.tests.push(test);
    }

    pub fn num_tests(&self) -> usize {
        self.tests.len()
    }

    pub fn get_test(&self, id: &str) -> Option<&BugTest> {
        self.tests.iter().find(|t| t.id == id)
    }

    pub fn run_tests(&self) -> usize {
        self.tests.iter().filter(|t| t.fixed).count()
    }
}

impl Default for QABugsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_test() {
        let mut manager = QABugsManager::new();
        let test = BugTest::new("bug1".to_string(), "Test description".to_string());
        manager.add_test(test);

        assert_eq!(manager.num_tests(), 1);
    }

    #[test]
    fn test_get_test() {
        let mut manager = QABugsManager::new();
        let test = BugTest::new("bug1".to_string(), "Test".to_string());
        manager.add_test(test);

        assert!(manager.get_test("bug1").is_some());
        assert!(manager.get_test("bug2").is_none());
    }

    #[test]
    fn test_run_tests() {
        let mut manager = QABugsManager::new();
        let mut test1 = BugTest::new("bug1".to_string(), "Test1".to_string());
        test1.set_fixed(true);

        let test2 = BugTest::new("bug2".to_string(), "Test2".to_string());

        manager.add_test(test1);
        manager.add_test(test2);

        assert_eq!(manager.run_tests(), 1);
    }
}
