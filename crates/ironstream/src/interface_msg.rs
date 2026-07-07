// FILE: interface_msg.rs
// occt: Interface_MSG

use std::collections::HashMap;

static mut MESSAGE_DICT: Option<HashMap<String, String>> = None;

/// This class manages a list of translated messages (messagery)
pub struct InterfaceMsg {
    thekey: String,
    theval: String,
}

impl InterfaceMsg {
    /// Creates a MSG to translate a message by key
    pub fn new(key: &str) -> Self {
        let theval = Self::translated(key);
        InterfaceMsg {
            thekey: key.to_string(),
            theval,
        }
    }

    /// Translates a message with one integer variable
    pub fn with_int(key: &str, i1: i32) -> Self {
        let template = Self::translated(key);
        let theval = format!("{}", i1); // TODO: use sprintf-like formatting
        InterfaceMsg {
            thekey: key.to_string(),
            theval,
        }
    }

    /// Translates a message with two integer variables
    pub fn with_two_ints(key: &str, i1: i32, i2: i32) -> Self {
        let _template = Self::translated(key);
        let theval = format!("{} {}", i1, i2); // TODO: proper formatting
        InterfaceMsg {
            thekey: key.to_string(),
            theval,
        }
    }

    /// Translates a message with one real variable
    pub fn with_real(key: &str, r1: f64, _intervals: i32) -> Self {
        let _template = Self::translated(key);
        let theval = format!("{}", r1);
        InterfaceMsg {
            thekey: key.to_string(),
            theval,
        }
    }

    /// Translates a message with one string variable
    pub fn with_string(key: &str, str_val: &str) -> Self {
        let _template = Self::translated(key);
        InterfaceMsg {
            thekey: key.to_string(),
            theval: str_val.to_string(),
        }
    }

    /// Translates a message with integer and string variables
    pub fn with_int_and_string(key: &str, ival: i32, str_val: &str) -> Self {
        let _template = Self::translated(key);
        let theval = format!("{} {}", ival, str_val);
        InterfaceMsg {
            thekey: key.to_string(),
            theval,
        }
    }

    /// Returns the translated message
    pub fn value(&self) -> &str {
        &self.theval
    }

    /// Returns item for a key, or the key itself if not found
    pub fn translated(key: &str) -> String {
        key.to_string() // TODO: actual dictionary lookup
    }

    /// Records a message in the dictionary
    pub fn record(key: &str, item: &str) {
        // TODO: Implement recording to dictionary
        let _key = key;
        let _item = item;
    }

    /// Fills the dictionary with messages from a stream
    pub fn read_stream(_data: &str) -> i32 {
        0 // TODO: Implement file reading
    }

    /// Reads messages from a file
    pub fn read_file(_file: &str) -> i32 {
        0 // TODO: Implement file reading
    }

    /// Returns an "intervalled" value from a starting real:
    /// the terms of the interval are the powers of ten multiplied by
    /// values depending on <order>, the returned value is the nearest
    /// term which is lower (upper=false) or greater (upper=true) than val.
    /// Faithful port of Interface_MSG::Intervalled.
    pub fn intervalled(val: f64, order: i32, upper: bool) -> f64 {
        let vl = if val > 0.0 { val } else { -val };
        let mut bl = 1.0_f64;
        let mut bu = 1.0_f64;
        if vl >= 1.0 {
            bu = 10.0;
            for _ in 0..200 {
                if vl < bu {
                    break;
                }
                bl = bu;
                bu *= 10.0;
            }
        } else {
            bl = 0.1;
            for _ in 0..200 {
                if vl >= bl {
                    break;
                }
                bu = bl;
                bl /= 10.0;
            }
            let _ = bu;
            if vl == 0.0 {
                return 0.0;
            }
        }

        let mut rst = vl / bl;
        if order <= 1 {
            rst = if upper { 10.0 } else { 1.0 };
        } else if order == 2 {
            if rst <= 3.0 {
                rst = if upper { 3.0 } else { 1.0 };
            } else {
                rst = if upper { 10.0 } else { 3.0 };
            }
        } else if order == 3 {
            if rst <= 2.0 {
                rst = if upper { 2.0 } else { 1.0 };
            } else if rst <= 5.0 {
                rst = if upper { 5.0 } else { 2.0 };
            } else {
                rst = if upper { 10.0 } else { 5.0 };
            }
        } else if order == 4 {
            if rst <= 2.0 {
                rst = if upper { 2.0 } else { 1.0 };
            } else if rst <= 3.0 {
                rst = if upper { 3.0 } else { 2.0 };
            } else if rst <= 6.0 {
                rst = if upper { 6.0 } else { 3.0 };
            } else {
                rst = if upper { 10.0 } else { 6.0 };
            }
        } else if order <= 6 {
            if rst <= 1.5 {
                rst = if upper { 1.5 } else { 1.0 };
            } else if rst <= 2.0 {
                rst = if upper { 2.0 } else { 1.5 };
            } else if rst <= 3.0 {
                rst = if upper { 3.0 } else { 2.0 };
            } else if rst <= 5.0 {
                rst = if upper { 5.0 } else { 3.0 };
            } else if rst <= 7.0 {
                rst = if upper { 7.0 } else { 5.0 };
            } else {
                rst = if upper { 10.0 } else { 7.0 };
            }
        } else {
            // only makes sense up to 10 ...
            if rst <= 1.2 {
                rst = if upper { 1.2 } else { 1.0 };
            } else if rst <= 1.5 {
                rst = if upper { 1.5 } else { 1.2 };
            } else if rst <= 2.0 {
                rst = if upper { 2.0 } else { 1.5 };
            } else if rst <= 2.5 {
                rst = if upper { 2.5 } else { 2.0 };
            } else if rst <= 3.0 {
                rst = if upper { 3.0 } else { 2.5 };
            } else if rst <= 4.0 {
                rst = if upper { 4.0 } else { 3.0 };
            } else if rst <= 5.0 {
                rst = if upper { 5.0 } else { 4.0 };
            } else if rst <= 6.0 {
                rst = if upper { 6.0 } else { 5.0 };
            } else if rst <= 8.0 {
                rst = if upper { 8.0 } else { 6.0 };
            } else {
                rst = if upper { 10.0 } else { 8.0 };
            }
        }
        if val < 0.0 {
            -(bl * rst)
        } else {
            bl * rst
        }
    }
}

impl AsRef<str> for InterfaceMsg {
    fn as_ref(&self) -> &str {
        &self.theval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let msg = InterfaceMsg::new("test_key");
        assert!(!msg.value().is_empty());
    }

    #[test]
    fn test_with_int() {
        let msg = InterfaceMsg::with_int("test_key", 42);
        assert_eq!(msg.value(), "42");
    }

    #[test]
    fn test_with_string() {
        let msg = InterfaceMsg::with_string("test_key", "hello");
        assert_eq!(msg.value(), "hello");
    }

    #[test]
    fn test_with_real() {
        let msg = InterfaceMsg::with_real("test_key", 3.14, -1);
        assert!(msg.value().contains("3.14"));
    }

    #[test]
    fn test_intervalled() {
        let result = InterfaceMsg::intervalled(5.0, 1, false);
        assert!(result > 0.0);
    }

    #[test]
    fn test_as_ref() {
        let msg = InterfaceMsg::new("key");
        let s: &str = msg.as_ref();
        assert!(!s.is_empty());
    }
}
