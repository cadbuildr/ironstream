// FILE: interface_line_buffer.rs
// occt: Interface_LineBuffer

/// Simple Management of a Line Buffer
pub struct InterfaceLineBuffer {
    myline: Vec<char>,
    mymax: usize,
    myinit: usize,
    mykeep: usize,
    myget: usize,
    mylen: usize,
    myfriz: i32,
    mykept: char,
}

impl InterfaceLineBuffer {
    /// Creates a LineBuffer with an absolute maximum size
    pub fn new(size: usize) -> Self {
        InterfaceLineBuffer {
            myline: vec![' '; size],
            mymax: size,
            myinit: 0,
            mykeep: 0,
            myget: 0,
            mylen: 0,
            myfriz: 0,
            mykept: ' ',
        }
    }

    /// Changes Maximum allowed size of Buffer
    pub fn set_max(&mut self, max: usize) {
        if max == 0 {
            self.mymax = self.myline.len();
        } else {
            self.mymax = max;
        }
    }

    /// Sets an Initial reservation for Blank characters
    pub fn set_initial(&mut self, initial: usize) {
        self.myinit = initial;
    }

    /// Sets a Keep Status at current Length
    pub fn set_keep(&mut self) {
        self.mykeep = self.mylen;
    }

    /// Returns True if there is room enough to add more characters
    pub fn can_get(&mut self, more: usize) -> bool {
        self.myget = more;
        self.mylen + more + self.myinit <= self.mymax
    }

    /// Returns the Content of the LineBuffer
    pub fn content(&self) -> &[char] {
        &self.myline[..self.mylen]
    }

    /// Returns the Length of the LineBuffer
    pub fn length(&self) -> usize {
        self.mylen + self.myinit
    }

    /// Clears completely the LineBuffer
    pub fn clear(&mut self) {
        self.mylen = 0;
        self.mykeep = 0;
        self.myget = 0;
        self.myfriz = 0;
    }

    /// Inhibits effect of SetInitial
    pub fn freeze_initial(&mut self) {
        self.myfriz = 1;
    }

    /// Fills a String with the Content and clears the LineBuffer
    pub fn moved(&mut self) -> String {
        let result: String = self.myline[..self.mylen].iter().collect();
        self.clear();
        result
    }

    /// Adds a text as a CString
    pub fn add(&mut self, text: &str) {
        for ch in text.chars() {
            if self.mylen < self.myline.len() {
                self.myline[self.mylen] = ch;
                self.mylen += 1;
            }
        }
    }

    /// Adds a single character
    pub fn add_char(&mut self, text: char) {
        if self.mylen < self.myline.len() {
            self.myline[self.mylen] = text;
            self.mylen += 1;
        }
    }

    fn prepare(&mut self) {
        if self.myfriz == 0 && self.myinit > 0 && self.mylen > 0 {
            // Insert initial blanks if required
        }
    }

    fn keep(&mut self) {
        if self.mykeep > 0 {
            // Keep characters from SetKeep
            let kept_len = self.mylen - self.mykeep;
            for i in 0..kept_len {
                self.myline[i] = self.myline[self.mykeep + i];
            }
            self.mylen = kept_len;
        } else {
            self.clear();
        }
    }
}

impl Default for InterfaceLineBuffer {
    fn default() -> Self {
        Self::new(256)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let buf = InterfaceLineBuffer::new(100);
        assert_eq!(buf.length(), 0);
    }

    #[test]
    fn test_add() {
        let mut buf = InterfaceLineBuffer::new(100);
        buf.add("test");
        assert_eq!(buf.length(), 4);
    }

    #[test]
    fn test_add_char() {
        let mut buf = InterfaceLineBuffer::new(100);
        buf.add_char('a');
        assert_eq!(buf.length(), 1);
    }

    #[test]
    fn test_can_get() {
        let mut buf = InterfaceLineBuffer::new(100);
        assert!(buf.can_get(50));
    }

    #[test]
    fn test_clear() {
        let mut buf = InterfaceLineBuffer::new(100);
        buf.add("test");
        buf.clear();
        assert_eq!(buf.length(), 0);
    }

    #[test]
    fn test_moved() {
        let mut buf = InterfaceLineBuffer::new(100);
        buf.add("hello");
        let result = buf.moved();
        assert_eq!(result, "hello");
        assert_eq!(buf.length(), 0);
    }

    #[test]
    fn test_set_max() {
        let mut buf = InterfaceLineBuffer::new(100);
        buf.set_max(50);
        // Max size should be updated
    }
}
