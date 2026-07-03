// FILE: standard_o_stream.rs
// occt: Standard_OStream

//! Standard_OStream is a type alias for std::io::Write in Rust.
//! It represents an output stream interface compatible with C++ std::ostream.

use std::io::{self, Write};

/// Represents an output stream (Rust equivalent of C++ std::ostream).
/// This is a type alias for any type implementing std::io::Write.
pub type StandardOStream = Box<dyn Write>;

/// Helper function to create a StandardOStream from a writer.
pub fn from_writer<W: Write + 'static>(writer: W) -> StandardOStream {
    Box::new(writer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ostream_from_vec() {
        let mut output = Vec::new();
        output.write_all(b"Hello").unwrap();
        assert_eq!(output, b"Hello");
    }

    #[test]
    fn test_ostream_from_writer() {
        let vec: Vec<u8> = Vec::new();
        let mut stream = from_writer(vec);
        stream.write_all(b"test").unwrap();
    }

    #[test]
    fn test_ostream_write() {
        let mut buf = Vec::new();
        let n = buf.write(b"data").unwrap();
        assert_eq!(n, 4);
        assert_eq!(&buf, b"data");
    }

    #[test]
    fn test_ostream_multiple_writes() {
        let mut buf = Vec::new();
        buf.write_all(b"hello").unwrap();
        buf.write_all(b" ").unwrap();
        buf.write_all(b"world").unwrap();
        assert_eq!(&buf, b"hello world");
    }
}
