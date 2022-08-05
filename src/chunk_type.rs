use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::net::AddrParseError;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
struct ChunkType {
    chunk_type_bytes: [u8; 4],
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.chunk_type_bytes
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    fn is_critical(&self) -> bool {
        (self.chunk_type_bytes[0] & 32_u8 == 0)
    }

    fn is_public(&self) -> bool {
        (self.chunk_type_bytes[1] & 32_u8 == 0)
    }

    fn is_reserved_bit_valid(&self) -> bool {
        (self.chunk_type_bytes[2] & 32_u8 == 0)
    }

    fn is_safe_to_copy(&self) -> bool {
        (self.chunk_type_bytes[3] & 32_u8 == 1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BadChunkType {
    message: String,
}

impl BadChunkType {
    fn new() -> Self {
        BadChunkType {
            message: "Bad chunk type".to_string(),
        }
    }
}

impl fmt::Display for BadChunkType {
    #[allow(deprecated, deprecated_in_future)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

impl Error for BadChunkType {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { chunk_type_bytes: value })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ChunkType::try_from(<[u8; 4]>::try_from(s.as_bytes()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::io::Read;
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