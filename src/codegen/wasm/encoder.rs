/// WebAssembly binary format encoder
///
/// This module provides utilities for encoding data in the WebAssembly binary format,
/// including LEB128 encoding for integers and section encoding.

/// Encoder for building WebAssembly binary data
pub struct WasmEncoder {
    data: Vec<u8>,
}

impl WasmEncoder {
    /// Create a new encoder
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Write a single byte
    pub fn write_byte(&mut self, byte: u8) {
        self.data.push(byte);
    }

    /// Write multiple bytes
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    /// Write an unsigned LEB128 integer
    pub fn write_unsigned(&mut self, mut value: u32) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80; // Set continuation bit
            }
            self.write_byte(byte);
            if value == 0 {
                break;
            }
        }
    }

    /// Write a signed LEB128 integer
    pub fn write_signed(&mut self, mut value: i32) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            let sign_bit = (byte & 0x40) != 0;

            if (value == 0 && !sign_bit) || (value == -1 && sign_bit) {
                self.write_byte(byte);
                break;
            } else {
                byte |= 0x80; // Set continuation bit
                self.write_byte(byte);
            }
        }
    }

    /// Write a 32-bit float
    pub fn write_f32(&mut self, value: f32) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Write a 64-bit float
    pub fn write_f64(&mut self, value: f64) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Write a string (length-prefixed UTF-8)
    pub fn write_string(&mut self, s: &str) {
        let bytes = s.as_bytes();
        self.write_unsigned(bytes.len() as u32);
        self.write_bytes(bytes);
    }

    /// Write a section with the given ID and content
    pub fn write_section<F>(&mut self, section_id: u8, writer: F)
    where
        F: FnOnce(&mut WasmEncoder),
    {
        self.write_byte(section_id);

        // Encode section contents into a temporary encoder
        let mut section_encoder = WasmEncoder::new();
        writer(&mut section_encoder);
        let section_data = section_encoder.finish();

        // Write section size then contents
        self.write_unsigned(section_data.len() as u32);
        self.write_bytes(&section_data);
    }

    /// Get the current size
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Finish encoding and return the data
    pub fn finish(self) -> Vec<u8> {
        self.data
    }
}

impl Default for WasmEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Encode an unsigned integer as LEB128 (standalone function)
pub fn encode_unsigned(value: u32) -> Vec<u8> {
    let mut result = Vec::new();
    let mut value = value;

    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        result.push(byte);
        if value == 0 {
            break;
        }
    }

    result
}

/// Encode a signed integer as LEB128 (standalone function)
pub fn encode_signed(value: i32) -> Vec<u8> {
    let mut result = Vec::new();
    let mut value = value;

    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        let sign_bit = (byte & 0x40) != 0;

        if (value == 0 && !sign_bit) || (value == -1 && sign_bit) {
            result.push(byte);
            break;
        } else {
            byte |= 0x80;
            result.push(byte);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_unsigned() {
        assert_eq!(encode_unsigned(0), vec![0x00]);
        assert_eq!(encode_unsigned(1), vec![0x01]);
        assert_eq!(encode_unsigned(127), vec![0x7F]);
        assert_eq!(encode_unsigned(128), vec![0x80, 0x01]);
        assert_eq!(encode_unsigned(624485), vec![0xE5, 0x8E, 0x26]);
    }

    #[test]
    fn test_encode_signed() {
        assert_eq!(encode_signed(0), vec![0x00]);
        assert_eq!(encode_signed(1), vec![0x01]);
        assert_eq!(encode_signed(-1), vec![0x7F]);
        assert_eq!(encode_signed(127), vec![0xFF, 0x00]);
        assert_eq!(encode_signed(-127), vec![0x81, 0x7F]);
    }

    #[test]
    fn test_encoder_bytes() {
        let mut enc = WasmEncoder::new();
        enc.write_byte(0x00);
        enc.write_bytes(&[0x61, 0x73, 0x6D]);
        assert_eq!(enc.finish(), vec![0x00, 0x61, 0x73, 0x6D]);
    }

    #[test]
    fn test_encoder_string() {
        let mut enc = WasmEncoder::new();
        enc.write_string("hello");
        assert_eq!(enc.finish(), vec![0x05, b'h', b'e', b'l', b'l', b'o']);
    }

    #[test]
    fn test_encoder_floats() {
        let mut enc = WasmEncoder::new();
        enc.write_f64(3.14159);
        let result = enc.finish();
        assert_eq!(result.len(), 8);
        assert_eq!(f64::from_le_bytes(result.try_into().unwrap()), 3.14159);
    }
}
