use crate::PngResult;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::slice::Iter;
use std::str::FromStr;

/// ChunkType represents the chunk type as detailed out in the PNG spec
#[derive(PartialEq, Debug)]
pub struct ChunkType {
    chunk_type_bytes: [u8; 4],
}

impl ChunkType {
    /// Returns the byte array representation of this chunk type
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type_bytes
    }

    /// Returns whether the current chunk type is valid
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    /// Returns if the current chunk type is critical
    fn is_critical(&self) -> bool {
        self.chunk_type_bytes[0] & 32_u8 == 0
    }

    /// Return if the current chunk type is public
    fn is_public(&self) -> bool {
        self.chunk_type_bytes[1] & 32_u8 == 0
    }

    /// Returns if the current chunk type has its reserved bit set
    fn is_reserved_bit_valid(&self) -> bool {
        self.chunk_type_bytes[2] & 32_u8 == 0
    }

    /// Returns if the current chunk type is safe to copy
    fn is_safe_to_copy(&self) -> bool {
        // Don't make the mistake of checking for == 1 here given that we are setting the bit
        // at the 5th position so it would be 32 and not 1 -- noob mistake, I know!
        self.chunk_type_bytes[3] & 32_u8 != 0
    }

    /// Returns the iterator over this chunk type
    pub fn iter(&self) -> Iter<'_, u8> {
        self.chunk_type_bytes.iter()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::PngError;

    /// Attempt to parse chunk type out of the given byte array
    fn try_from(value: [u8; 4]) -> PngResult<Self> {
        let byte_check = |b: &u8| *b < 65 || *b > 122 || (*b > 90 && *b < 97);
        if value.iter().any(byte_check) {
            Err("Invalid chunk payload".into())
        } else {
            Ok(ChunkType {
                chunk_type_bytes: value,
            })
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for b in self.chunk_type_bytes {
            write!(f, "{}", char::from(b))?;
        }
        Ok(())
    }
}

impl FromStr for ChunkType {
    type Err = crate::PngError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ChunkType::try_from(<[u8; 4]>::try_from(s.as_bytes()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;
    use std::{assert_eq, format};

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
        assert_eq!(chunk.err().unwrap().to_string(), "Invalid chunk payload");
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
