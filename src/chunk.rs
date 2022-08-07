use crate::chunk_type::ChunkType;
use crate::{PngError, PngResult};
use crc::Crc;
use std::fmt::{Display, Formatter};

/// Chunk represents a PNG chunk as detailed out in the PNG spec
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    checksum: u32,
}

impl Chunk {
    /// The number of bytes taken up by the Chunk length field
    const LENGTH_BYTES_LEN: usize = 4;

    /// The number of bytes taken up by the chunk type field
    const CHUNK_TYPE_BYTES_LEN: usize = 4;

    /// Create a new `Chunk` from the given chunk type and payload.
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let mut digest = crc.digest();
        digest.update(&chunk_type.bytes());
        digest.update(&data);
        Chunk {
            length: data.len() as u32,
            chunk_type,
            data,
            checksum: digest.finalize(),
        }
    }

    /// The overall size of this chunk including chunk type, crc, data and length field
    pub fn overall_length(&self) -> u32 {
        self.length + 12 // 4 * 3 -- 4 bytes each for chunk type, crc, length field
    }

    /// The length of the data/payload held inside this chunk
    fn length(&self) -> u32 {
        self.length
    }

    /// Returns the type for this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// Returns the payload for this chunk
    fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the CRC or checksum for this chunk
    fn crc(&self) -> u32 {
        self.checksum
    }

    /// Returns the data represented as `String` for this chunk
    pub fn data_as_string(&self) -> PngResult<String> {
        String::from_utf8(self.data.clone()).map_err(PngError::from)
    }

    /// Returns the byte representation for this chunk
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.iter())
            .chain(self.data.iter())
            .chain(self.checksum.to_be_bytes().iter())
            .cloned()
            .collect::<Vec<_>>()
    }
}

fn read_4_bytes(slice: &[u8], start: usize, end: usize) -> [u8; 4] {
    // If we can't convert a 4 element slice to 4 element array, we panic };
    assert_eq!(end - start, 4);
    (&slice[start..end]).to_vec().try_into().unwrap()
}

impl TryFrom<&[u8]> for Chunk {
    type Error = PngError;

    /// Attempt to perform conversion from a byte slice to Chunk
    fn try_from(value: &[u8]) -> PngResult<Chunk> {
        // Read the length and chunk type bytes back to back
        let length = u32::from_be_bytes(read_4_bytes(value, 0, Chunk::LENGTH_BYTES_LEN));
        let chunk_type = ChunkType::try_from(read_4_bytes(
            value,
            Chunk::CHUNK_TYPE_BYTES_LEN,
            Chunk::CHUNK_TYPE_BYTES_LEN + Chunk::LENGTH_BYTES_LEN,
        ))?;

        let start = Chunk::CHUNK_TYPE_BYTES_LEN + Chunk::LENGTH_BYTES_LEN;
        let (data_start, data_end) = (start, start + length as usize);
        let data = (&value[data_start..data_end]).to_vec();
        let checksum = u32::from_be_bytes(read_4_bytes(value, data_end, data_end + 4));
        let chunk = Chunk::new(chunk_type, data);

        if chunk.checksum != checksum {
            return Err("Incoming check does not match computed checksum".into());
        }
        if chunk.length != length {
            return Err("Incoming length does not match computed length".into());
        }
        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8(self.data.clone()).unwrap();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;
    use std::{assert_eq, format};

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
