// FILE: iges_data_file_recognizer.rs
// occt: IGESData_FileRecognizer

//! Abstract base class for recognizing and creating IGES entities from file data.
//! Supports a chain of recognizers that are tried in sequence.

use std::cell::RefCell;
use std::rc::Rc;

/// Recognizer for IGES entities based on type/form information
#[derive(Clone, Debug)]
pub struct FileRecognizer {
    result: Rc<RefCell<Option<String>>>,
    next: Rc<RefCell<Option<Box<FileRecognizer>>>>,
}

impl FileRecognizer {
    /// Creates a new FileRecognizer with no result
    pub fn new() -> Self {
        FileRecognizer {
            result: Rc::new(RefCell::new(None)),
            next: Rc::new(RefCell::new(None)),
        }
    }

    /// Evaluates if recognition has a result, returns it if yes
    /// Tries this recognizer first, then added recognizers in order
    pub fn evaluate(&self, type_num: i32, form_num: i32) -> Option<String> {
        // Try self evaluation
        self.eval(type_num, form_num);

        // Check if result was set
        if let Some(res) = self.result.borrow().as_ref() {
            return Some(res.clone());
        }

        // Try next recognizer in chain
        if let Some(next_rec) = self.next.borrow().as_ref() {
            return next_rec.evaluate(type_num, form_num);
        }

        None
    }

    /// Returns result of last recognition
    pub fn result(&self) -> Option<String> {
        self.result.borrow().as_ref().cloned()
    }

    /// Adds a new Recognizer to the chain
    /// Added recognizers are tried in order after Eval returns no result
    pub fn add(&self, recognizer: FileRecognizer) {
        let mut next_opt = self.next.borrow_mut();

        if next_opt.is_none() {
            *next_opt = Some(Box::new(recognizer));
        } else {
            // Chain to the next recognizer
            if let Some(next) = next_opt.as_ref() {
                // Unwrap is safe because we checked is_some above
                drop(next_opt);
                let next_borrowed = self.next.borrow().as_ref().unwrap().as_ref();
                let next_owned = FileRecognizer {
                    result: Rc::clone(&next_borrowed.result),
                    next: Rc::clone(&next_borrowed.next),
                };
                next_owned.add(recognizer);
            }
        }
    }

    /// Records the result of recognition
    pub fn set_ok(&self, result: String) {
        *self.result.borrow_mut() = Some(result);
    }

    /// Records that recognition gives no result
    pub fn set_ko(&self) {
        *self.result.borrow_mut() = None;
    }

    /// Evaluation method to be overridden by specific recognizers
    /// Subclasses should call set_ok or set_ko
    pub fn eval(&self, type_num: i32, form_num: i32) {
        // Default implementation: no recognition
        self.set_ko();
    }

    /// Clears the result for a new evaluation
    pub fn clear(&self) {
        *self.result.borrow_mut() = None;
    }
}

impl Default for FileRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fr = FileRecognizer::new();
        assert_eq!(fr.result(), None);
    }

    #[test]
    fn test_set_ok() {
        let fr = FileRecognizer::new();
        fr.set_ok("entity1".to_string());
        assert_eq!(fr.result(), Some("entity1".to_string()));
    }

    #[test]
    fn test_set_ko() {
        let fr = FileRecognizer::new();
        fr.set_ok("entity1".to_string());
        assert_eq!(fr.result(), Some("entity1".to_string()));
        fr.set_ko();
        assert_eq!(fr.result(), None);
    }

    #[test]
    fn test_evaluate_no_result() {
        let fr = FileRecognizer::new();
        let res = fr.evaluate(1, 0);
        assert_eq!(res, None);
    }

    #[test]
    fn test_evaluate_with_result() {
        let fr = FileRecognizer::new();
        fr.set_ok("test_entity".to_string());
        let res = fr.evaluate(1, 0);
        assert_eq!(res, Some("test_entity".to_string()));
    }

    #[test]
    fn test_clear() {
        let fr = FileRecognizer::new();
        fr.set_ok("entity".to_string());
        assert_eq!(fr.result(), Some("entity".to_string()));
        fr.clear();
        assert_eq!(fr.result(), None);
    }

    #[test]
    fn test_add_recognizer() {
        let fr1 = FileRecognizer::new();
        let fr2 = FileRecognizer::new();
        fr2.set_ok("second_entity".to_string());

        fr1.add(fr2);
        // After adding, both recognizers exist in the chain
        assert_eq!(fr1.result(), None); // fr1 still has no result
    }

    #[test]
    fn test_default() {
        let fr = FileRecognizer::default();
        assert_eq!(fr.result(), None);
    }
}
