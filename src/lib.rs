#![forbid(unsafe_code)]

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Error)]
pub enum BlenderError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("invalid Blender file header")]
    InvalidHeader,
}

impl PartialEq for BlenderError {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other),
            (Self::InvalidHeader, Self::InvalidHeader)
        )
    }
}

/// Metadata extracted from a Blender `.blend` file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlendMetadata {
    /// '_' for 32-bit, '-' for 64-bit pointer size.
    pub pointer_size: char,
    /// 'v' little-endian, 'V' big-endian.
    pub endian: char,
    /// (major, minor, subminor) version numbers.
    pub version: (u8, u8, u8),
}

impl BlendMetadata {
    /// Parses the first 12 bytes of a `.blend` file to extract metadata.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, BlenderError> {
        let mut f = File::open(path)?;
        let mut header = [0u8; 12];
        let n = f.read(&mut header)?;
        if n < 12 {
            return Err(BlenderError::InvalidHeader);
        }
        // Verify magic "BLENDER"
        if &header[0..7] != b"BLENDER" {
            return Err(BlenderError::InvalidHeader);
        }
        let pointer_size = header[7] as char;
        let endian = header[8] as char;
        let major = header[9] - b'0';
        let minor = header[10] - b'0';
        let sub = header[11] - b'0';
        Ok(Self {
            pointer_size,
            endian,
            version: (major, minor, sub),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn make_blend_bytes(ptr: char, endian: char, ver: (u8, u8, u8)) -> Vec<u8> {
        let mut header = Vec::new();
        header.extend_from_slice(b"BLENDER");
        header.push(ptr as u8);
        header.push(endian as u8);
        header.push(b'0' + ver.0);
        header.push(b'0' + ver.1);
        header.push(b'0' + ver.2);
        header
    }

    #[test]
    fn test_valid_header_aaa() {
        // Arrange
        let dir = std::env::temp_dir();
        let path = dir.join("test_valid.blend");
        let bytes = make_blend_bytes('_', 'v', (2, 9, 0));
        std::fs::write(&path, &bytes).unwrap();

        // Act
        let meta = BlendMetadata::from_file(&path).expect("parse");

        // Assert
        assert_eq!(meta.pointer_size, '_');
        assert_eq!(meta.endian, 'v');
        assert_eq!(meta.version, (2, 9, 0));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_invalid_magic_aaa() {
        // Arrange
        let dir = std::env::temp_dir();
        let path = dir.join("test_invalid.blend");
        std::fs::write(&path, b"NOTBLEND____").unwrap();

        // Act
        let err = BlendMetadata::from_file(&path).unwrap_err();

        // Assert
        assert_eq!(err, BlenderError::InvalidHeader);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_short_file_aaa() {
        // Arrange
        let dir = std::env::temp_dir();
        let path = dir.join("test_short.blend");
        std::fs::write(&path, b"BLEND").unwrap();

        // Act
        let err = BlendMetadata::from_file(&path).unwrap_err();

        // Assert
        assert_eq!(err, BlenderError::InvalidHeader);
        let _ = std::fs::remove_file(path);
    }
}
