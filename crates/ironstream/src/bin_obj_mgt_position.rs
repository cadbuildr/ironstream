// FILE: bin_obj_mgt_position.rs
// occt: BinObjMgt_Position

use std::io::{Read, Seek, Write};

/// Stores and manipulates position in the stream.
/// Tracks the position where an object's size should be written,
/// and can compute and store the size difference.
pub struct BinObjMgtPosition {
    position: u64,
    size: u64,
}

impl BinObjMgtPosition {
    /// Creates position using the current stream position.
    pub fn new<W: Seek + ?Sized>(stream: &mut W) -> std::io::Result<Self> {
        let pos = stream.stream_position()?;
        Ok(BinObjMgtPosition {
            position: pos,
            size: 0,
        })
    }

    /// Stores the difference between the current position and the stored one.
    pub fn store_size<W: Seek + ?Sized>(&mut self, stream: &mut W) -> std::io::Result<()> {
        let current_pos = stream.stream_position()?;
        self.size = current_pos.saturating_sub(self.position);
        Ok(())
    }

    /// Writes stored size at the stored position. Changes the current stream position.
    /// If dummy is true, writes a zero size.
    pub fn write_size<W: Seek + Write + ?Sized>(
        &self,
        stream: &mut W,
        dummy: bool,
    ) -> std::io::Result<()> {
        if !dummy {
            stream.seek(std::io::SeekFrom::Start(self.position))?;
        }
        let size_to_write = if dummy { 0u64 } else { self.size };
        let bytes = size_to_write.to_le_bytes();
        stream.write_all(&bytes)?;
        Ok(())
    }

    /// Returns the stored position
    pub fn position(&self) -> u64 {
        self.position
    }

    /// Returns the stored size
    pub fn size(&self) -> u64 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_position_creation() -> std::io::Result<()> {
        let mut data = vec![0u8; 100];
        let mut cursor = Cursor::new(&mut data);
        cursor.seek(std::io::SeekFrom::Start(10))?;
        let pos = BinObjMgtPosition::new(&mut cursor)?;
        assert_eq!(pos.position(), 10);
        assert_eq!(pos.size(), 0);
        Ok(())
    }

    #[test]
    fn test_store_size() -> std::io::Result<()> {
        let mut data = vec![0u8; 100];
        let mut cursor = Cursor::new(&mut data);
        cursor.seek(std::io::SeekFrom::Start(10))?;
        let mut pos = BinObjMgtPosition::new(&mut cursor)?;

        cursor.seek(std::io::SeekFrom::Start(20))?;
        pos.store_size(&mut cursor)?;

        assert_eq!(pos.position(), 10);
        assert_eq!(pos.size(), 10);
        Ok(())
    }

    #[test]
    fn test_write_size() -> std::io::Result<()> {
        let mut data = vec![0u8; 100];
        let mut cursor = Cursor::new(&mut data);
        cursor.seek(std::io::SeekFrom::Start(10))?;
        let mut pos = BinObjMgtPosition::new(&mut cursor)?;

        cursor.seek(std::io::SeekFrom::Start(20))?;
        pos.store_size(&mut cursor)?;

        pos.write_size(&mut cursor, false)?;

        // OCCT WriteSize seeks back to the stored position and writes the
        // 8-byte size there, so the stream ends up right after it.
        let current_pos = cursor.stream_position()?;
        assert_eq!(current_pos, 18);

        cursor.seek(std::io::SeekFrom::Start(10))?;
        let mut buf = [0u8; 8];
        cursor.read_exact(&mut buf)?;
        let stored_size = u64::from_le_bytes(buf);
        assert_eq!(stored_size, 10);
        Ok(())
    }

    #[test]
    fn test_write_size_dummy() -> std::io::Result<()> {
        let mut data = vec![0u8; 100];
        let mut cursor = Cursor::new(&mut data);
        cursor.seek(std::io::SeekFrom::Start(10))?;
        let mut pos = BinObjMgtPosition::new(&mut cursor)?;

        cursor.seek(std::io::SeekFrom::Start(20))?;
        pos.store_size(&mut cursor)?;

        let initial_pos = cursor.stream_position()?;
        pos.write_size(&mut cursor, true)?;
        let final_pos = cursor.stream_position()?;

        // With dummy=true, OCCT does not seek: a zero placeholder is written
        // at the current position and the stream advances by 8 bytes.
        assert_eq!(final_pos, initial_pos + 8);

        cursor.seek(std::io::SeekFrom::Start(initial_pos))?;
        let mut buf = [0u8; 8];
        cursor.read_exact(&mut buf)?;
        let stored_size = u64::from_le_bytes(buf);
        assert_eq!(stored_size, 0);
        Ok(())
    }
}
