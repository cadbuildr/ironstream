// FILE: vrml_data_in_buffer.rs
// occt: VrmlData_InBuffer

#[derive(Clone, Debug)]
pub struct VrmlDataInBuffer {
    data: Vec<u8>,
    position: usize,
}

impl VrmlDataInBuffer {
    pub fn new() -> Self {
        VrmlDataInBuffer {
            data: Vec::new(),
            position: 0,
        }
    }

    pub fn append(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn read(&mut self) -> Option<u8> {
        if self.position < self.data.len() {
            let byte = self.data[self.position];
            self.position += 1;
            Some(byte)
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

impl Default for VrmlDataInBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let buf = VrmlDataInBuffer::new();
        assert_eq!(buf.size(), 0);
    }

    #[test]
    fn test_append_read() {
        let mut buf = VrmlDataInBuffer::new();
        buf.append(42);
        assert_eq!(buf.read(), Some(42));
        assert_eq!(buf.read(), None);
    }
}
