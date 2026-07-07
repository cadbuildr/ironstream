// FILE: xml_obj_mgt.rs
// occt: XmlObjMgt

/// XmlObjMgt provides services to manage storage grain of data and persistent extern references.
/// Contains various utility functions for XML object management.
pub struct XmlObjMgt;

impl XmlObjMgt {
    /// Define the name of XML attribute 'ID'
    pub fn id_string() -> &'static str {
        "ID"
    }

    /// Get tag entry string from DOMString representation
    pub fn get_tag_entry_string(_target: &str) -> Option<String> {
        // Implementation stub: converts XPath expression to TagEntry string
        None
    }

    /// Set tag entry string from TagEntry string representation
    pub fn set_tag_entry_string(_source: &mut String, _tag_entry: &str) {
        // Implementation stub: converts TagEntry string to XPath expression
    }

    /// Find child element by object ID
    pub fn find_child_element(_source: &str, _obj_id: i32) -> Option<String> {
        None
    }

    /// Find child by reference name
    pub fn find_child_by_ref(_source: &str, _ref_name: &str) -> Option<String> {
        None
    }

    /// Find child by name
    pub fn find_child_by_name(_source: &str, _name: &str) -> Option<String> {
        None
    }

    /// Get integer from string
    pub fn get_integer(string: &str) -> Option<(i32, &str)> {
        let trimmed = string.trim_start();
        let mut end = 0;
        let mut chars = trimmed.chars();

        // Handle optional sign
        if let Some(first) = chars.next() {
            if first == '-' || first == '+' {
                end = 1;
            }
        }

        for (i, ch) in trimmed.chars().enumerate().skip(end) {
            if !ch.is_ascii_digit() {
                if i == end {
                    return None; // No digits found
                }
                let num_str = &trimmed[..i];
                let value = num_str.parse::<i32>().ok()?;
                return Some((value, &trimmed[i..]));
            }
        }

        let value = trimmed.parse::<i32>().ok()?;
        Some((value, ""))
    }

    /// Get real (double) from string
    pub fn get_real(string: &str) -> Option<(f64, &str)> {
        let trimmed = string.trim_start();
        let mut chars = trimmed.chars().peekable();
        let mut end = 0;

        // Handle optional sign
        if let Some(&ch) = chars.peek() {
            if ch == '-' || ch == '+' {
                chars.next();
                end = 1;
            }
        }

        // Read digits before decimal point
        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_digit() {
                chars.next();
                end += 1;
            } else {
                break;
            }
        }

        // Handle decimal point
        if let Some(&'.') = chars.peek() {
            chars.next();
            end += 1;
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_digit() {
                    chars.next();
                    end += 1;
                } else {
                    break;
                }
            }
        }

        // Handle exponent
        if let Some(&ch) = chars.peek() {
            if ch == 'e' || ch == 'E' {
                chars.next();
                end += 1;
                if let Some(&sign) = chars.peek() {
                    if sign == '+' || sign == '-' {
                        chars.next();
                        end += 1;
                    }
                }
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() {
                        chars.next();
                        end += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        if end == 0 || (end == 1 && (trimmed.starts_with('-') || trimmed.starts_with('+'))) {
            return None; // No valid number
        }

        let num_str = &trimmed[..end];
        let value = num_str.parse::<f64>().ok()?;
        Some((value, &trimmed[end..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_string() {
        assert_eq!(XmlObjMgt::id_string(), "ID");
    }

    #[test]
    fn test_get_integer() {
        assert_eq!(XmlObjMgt::get_integer("123 rest"), Some((123, " rest")));
        assert_eq!(XmlObjMgt::get_integer("-42abc"), Some((-42, "abc")));
        assert_eq!(XmlObjMgt::get_integer("+100"), Some((100, "")));
        assert_eq!(XmlObjMgt::get_integer("abc"), None);
        assert_eq!(XmlObjMgt::get_integer(""), None);
    }

    #[test]
    fn test_get_real() {
        assert_eq!(XmlObjMgt::get_real("3.14 rest"), Some((3.14, " rest")));
        assert_eq!(XmlObjMgt::get_real("-42.0e2"), Some((-42.0e2, "")));
        assert_eq!(XmlObjMgt::get_real("1.5E-3x"), Some((1.5E-3, "x")));
        assert_eq!(XmlObjMgt::get_real("abc"), None);
        // OCCT XmlObjMgt::GetReal delegates to strtod, which accepts ".5" as 0.5
        assert_eq!(XmlObjMgt::get_real(".5"), Some((0.5, "")));
    }
}
