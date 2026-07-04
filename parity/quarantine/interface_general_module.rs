// FILE: interface_general_module.rs
// occt: Interface_GeneralModule

/// This class defines general services which must be provided
/// for each type of Entity (i.e. of Transient Object processed
/// by an Interface)
pub trait InterfaceGeneralModule {
    /// Specific filling of the list of Entities shared by an Entity
    fn fill_shared(&self, cn: i32, ent: &std::sync::Arc<dyn std::any::Any>) -> Vec<std::sync::Arc<dyn std::any::Any>> {
        self.fill_shared_case(cn, ent)
    }

    /// Specific filling of the list of Entities shared by an Entity
    fn fill_shared_case(&self, cn: i32, ent: &std::sync::Arc<dyn std::any::Any>) -> Vec<std::sync::Arc<dyn std::any::Any>>;

    /// List the Implied References of <ent>
    fn list_implied(&self, cn: i32, ent: &std::sync::Arc<dyn std::any::Any>) -> Vec<std::sync::Arc<dyn std::any::Any>> {
        self.list_implied_case(cn, ent)
    }

    /// List the Implied References of <ent>
    fn list_implied_case(&self, _cn: i32, _ent: &std::sync::Arc<dyn std::any::Any>) -> Vec<std::sync::Arc<dyn std::any::Any>> {
        Vec::new()
    }

    /// Specific Checking of an Entity
    fn check_case(&self, cn: i32, ent: &std::sync::Arc<dyn std::any::Any>) -> bool;

    /// Specific answer to the question "is Copy properly implemented"
    fn can_copy(&self, _cn: i32, _ent: &std::sync::Arc<dyn std::any::Any>) -> bool {
        false
    }

    /// Dispatches an entity
    fn dispatch(&self, _cn: i32, _entfrom: &std::sync::Arc<dyn std::any::Any>) -> (bool, Option<std::sync::Arc<dyn std::any::Any>>) {
        (false, None)
    }

    /// Creates a new void entity
    fn new_void(&self, cn: i32) -> std::sync::Arc<dyn std::any::Any>;

    /// Specific Copy ("Deep") from <entfrom> to <entto>
    fn copy_case(&self, cn: i32, entfrom: &std::sync::Arc<dyn std::any::Any>, entto: &mut std::sync::Arc<dyn std::any::Any>);

    /// Specific operator (create+copy)
    fn new_copied_case(&self, _cn: i32, _entfrom: &std::sync::Arc<dyn std::any::Any>) -> Option<std::sync::Arc<dyn std::any::Any>> {
        None
    }

    /// Specific Copying of Implied References
    fn renew_implied_case(&self, _cn: i32, _entfrom: &std::sync::Arc<dyn std::any::Any>, _entto: &mut std::sync::Arc<dyn std::any::Any>) {
        // Default does nothing
    }

    /// Prepares an entity to be deleted
    fn when_delete_case(&self, _cn: i32, _ent: &std::sync::Arc<dyn std::any::Any>, _dispatched: bool) {
        // Default does nothing
    }

    /// Returns a category number which characterizes an entity
    fn category_number(&self, _cn: i32, _ent: &std::sync::Arc<dyn std::any::Any>) -> i32 {
        0
    }

    /// Determines if an entity brings a Name
    fn name(&self, _cn: i32, _ent: &std::sync::Arc<dyn std::any::Any>) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestModule;

    impl InterfaceGeneralModule for TestModule {
        fn fill_shared_case(&self, _cn: i32, _ent: &std::sync::Arc<dyn std::any::Any>) -> Vec<std::sync::Arc<dyn std::any::Any>> {
            Vec::new()
        }

        fn check_case(&self, _cn: i32, _ent: &std::sync::Arc<dyn std::any::Any>) -> bool {
            true
        }

        fn new_void(&self, _cn: i32) -> std::sync::Arc<dyn std::any::Any> {
            std::sync::Arc::new(())
        }

        fn copy_case(&self, _cn: i32, _entfrom: &std::sync::Arc<dyn std::any::Any>, _entto: &mut std::sync::Arc<dyn std::any::Any>) {
        }
    }

    #[test]
    fn test_module_trait() {
        let module = TestModule;
        let ent = std::sync::Arc::new(());
        assert!(module.check_case(0, &ent));
    }
}
