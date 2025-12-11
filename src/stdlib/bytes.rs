//! Bytes and binary data handling module for A-lang
//! Provides low-level binary data manipulation, packing/unpacking, and encoding

use crate::interpreter::value::Value;
use std::collections::HashMap;

/// Byte order (endianness)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
    Native,
}

impl ByteOrder {
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "little" | "le" => Ok(ByteOrder::LittleEndian),
            "big" | "be" => Ok(ByteOrder::BigEndian),
            "native" => Ok(ByteOrder::Native),
            _ => Err(format!("Unknown byte order: {}", s)),
        }
    }

    pub fn is_little_endian(&self) -> bool {
        match self {
            ByteOrder::LittleEndian => true,
            ByteOrder::BigEndian => false,
            ByteOrder::Native => cfg!(target_endian = "little"),
        }
    }
}

/// Binary data buffer
#[derive(Debug, Clone)]
pub struct ByteBuffer {
    data: Vec<u8>,
    position: usize,
    byte_order: ByteOrder,
}

impl ByteBuffer {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            position: 0,
            byte_order: ByteOrder::Native,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            position: 0,
            byte_order: ByteOrder::Native,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            data: bytes,
            position: 0,
            byte_order: ByteOrder::Native,
        }
    }

    pub fn set_byte_order(&mut self, order: ByteOrder) {
        self.byte_order = order;
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn seek(&mut self, pos: usize) -> Result<(), String> {
        if pos > self.data.len() {
            return Err(format!(
                "Position {} out of bounds (buffer size: {})",
                pos,
                self.data.len()
            ));
        }
        self.position = pos;
        Ok(())
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.position)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.data
    }

    /// Write operations
    pub fn write_u8(&mut self, value: u8) {
        self.data.push(value);
    }

    pub fn write_i8(&mut self, value: i8) {
        self.data.push(value as u8);
    }

    pub fn write_u16(&mut self, value: u16) {
        let bytes = if self.byte_order.is_little_endian() {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.data.extend_from_slice(&bytes);
    }

    pub fn write_i16(&mut self, value: i16) {
        let bytes = if self.byte_order.is_little_endian() {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.data.extend_from_slice(&bytes);
    }

    pub fn write_u32(&mut self, value: u32) {
        let bytes = if self.byte_order.is_little_endian() {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.data.extend_from_slice(&bytes);
    }

    pub fn write_i32(&mut self, value: i32) {
        let bytes = if self.byte_order.is_little_endian() {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.data.extend_from_slice(&bytes);
    }

    pub fn write_u64(&mut self, value: u64) {
        let bytes = if self.byte_order.is_little_endian() {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.data.extend_from_slice(&bytes);
    }

    pub fn write_i64(&mut self, value: i64) {
        let bytes = if self.byte_order.is_little_endian() {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.data.extend_from_slice(&bytes);
    }

    pub fn write_f32(&mut self, value: f32) {
        let bytes = if self.byte_order.is_little_endian() {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.data.extend_from_slice(&bytes);
    }

    pub fn write_f64(&mut self, value: f64) {
        let bytes = if self.byte_order.is_little_endian() {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.data.extend_from_slice(&bytes);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn write_string(&mut self, s: &str) {
        self.data.extend_from_slice(s.as_bytes());
    }

    pub fn write_cstring(&mut self, s: &str) {
        self.data.extend_from_slice(s.as_bytes());
        self.data.push(0); // null terminator
    }

    /// Read operations
    pub fn read_u8(&mut self) -> Result<u8, String> {
        if self.position >= self.data.len() {
            return Err("Buffer underflow: cannot read u8".to_string());
        }
        let value = self.data[self.position];
        self.position += 1;
        Ok(value)
    }

    pub fn read_i8(&mut self) -> Result<i8, String> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_u16(&mut self) -> Result<u16, String> {
        if self.position + 2 > self.data.len() {
            return Err("Buffer underflow: cannot read u16".to_string());
        }
        let bytes = &self.data[self.position..self.position + 2];
        self.position += 2;

        let value = if self.byte_order.is_little_endian() {
            u16::from_le_bytes([bytes[0], bytes[1]])
        } else {
            u16::from_be_bytes([bytes[0], bytes[1]])
        };
        Ok(value)
    }

    pub fn read_i16(&mut self) -> Result<i16, String> {
        if self.position + 2 > self.data.len() {
            return Err("Buffer underflow: cannot read i16".to_string());
        }
        let bytes = &self.data[self.position..self.position + 2];
        self.position += 2;

        let value = if self.byte_order.is_little_endian() {
            i16::from_le_bytes([bytes[0], bytes[1]])
        } else {
            i16::from_be_bytes([bytes[0], bytes[1]])
        };
        Ok(value)
    }

    pub fn read_u32(&mut self) -> Result<u32, String> {
        if self.position + 4 > self.data.len() {
            return Err("Buffer underflow: cannot read u32".to_string());
        }
        let bytes = &self.data[self.position..self.position + 4];
        self.position += 4;

        let value = if self.byte_order.is_little_endian() {
            u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        } else {
            u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        };
        Ok(value)
    }

    pub fn read_i32(&mut self) -> Result<i32, String> {
        if self.position + 4 > self.data.len() {
            return Err("Buffer underflow: cannot read i32".to_string());
        }
        let bytes = &self.data[self.position..self.position + 4];
        self.position += 4;

        let value = if self.byte_order.is_little_endian() {
            i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        } else {
            i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        };
        Ok(value)
    }

    pub fn read_u64(&mut self) -> Result<u64, String> {
        if self.position + 8 > self.data.len() {
            return Err("Buffer underflow: cannot read u64".to_string());
        }
        let bytes = &self.data[self.position..self.position + 8];
        self.position += 8;

        let value = if self.byte_order.is_little_endian() {
            u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])
        } else {
            u64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])
        };
        Ok(value)
    }

    pub fn read_i64(&mut self) -> Result<i64, String> {
        if self.position + 8 > self.data.len() {
            return Err("Buffer underflow: cannot read i64".to_string());
        }
        let bytes = &self.data[self.position..self.position + 8];
        self.position += 8;

        let value = if self.byte_order.is_little_endian() {
            i64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])
        } else {
            i64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])
        };
        Ok(value)
    }

    pub fn read_f32(&mut self) -> Result<f32, String> {
        if self.position + 4 > self.data.len() {
            return Err("Buffer underflow: cannot read f32".to_string());
        }
        let bytes = &self.data[self.position..self.position + 4];
        self.position += 4;

        let value = if self.byte_order.is_little_endian() {
            f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        } else {
            f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        };
        Ok(value)
    }

    pub fn read_f64(&mut self) -> Result<f64, String> {
        if self.position + 8 > self.data.len() {
            return Err("Buffer underflow: cannot read f64".to_string());
        }
        let bytes = &self.data[self.position..self.position + 8];
        self.position += 8;

        let value = if self.byte_order.is_little_endian() {
            f64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])
        } else {
            f64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])
        };
        Ok(value)
    }

    pub fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>, String> {
        if self.position + count > self.data.len() {
            return Err(format!("Buffer underflow: cannot read {} bytes", count));
        }
        let bytes = self.data[self.position..self.position + count].to_vec();
        self.position += count;
        Ok(bytes)
    }

    pub fn read_string(&mut self, count: usize) -> Result<String, String> {
        let bytes = self.read_bytes(count)?;
        String::from_utf8(bytes).map_err(|e| format!("Invalid UTF-8 string: {}", e))
    }

    pub fn read_cstring(&mut self) -> Result<String, String> {
        let start = self.position;
        let mut end = start;

        while end < self.data.len() && self.data[end] != 0 {
            end += 1;
        }

        if end >= self.data.len() {
            return Err("Buffer underflow: no null terminator found".to_string());
        }

        let bytes = self.data[start..end].to_vec();
        self.position = end + 1; // skip null terminator

        String::from_utf8(bytes).map_err(|e| format!("Invalid UTF-8 string: {}", e))
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        self.data.clear();
        self.position = 0;
    }

    /// Reset position to start
    pub fn reset(&mut self) {
        self.position = 0;
    }
}

impl Default for ByteBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Binary encoding utilities
pub struct BinaryEncoder;

impl BinaryEncoder {
    /// Encode bytes to hexadecimal string
    pub fn to_hex(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }

    /// Decode hexadecimal string to bytes
    pub fn from_hex(hex: &str) -> Result<Vec<u8>, String> {
        if hex.len() % 2 != 0 {
            return Err("Hex string must have even length".to_string());
        }

        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for i in (0..hex.len()).step_by(2) {
            let byte_str = &hex[i..i + 2];
            let byte = u8::from_str_radix(byte_str, 16)
                .map_err(|e| format!("Invalid hex string: {}", e))?;
            bytes.push(byte);
        }

        Ok(bytes)
    }

    /// Encode bytes to base64
    pub fn to_base64(bytes: &[u8]) -> String {
        const BASE64_CHARS: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();

        let mut i = 0;
        while i < bytes.len() {
            let b1 = bytes[i];
            let b2 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
            let b3 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

            let n = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

            result.push(BASE64_CHARS[((n >> 18) & 0x3F) as usize] as char);
            result.push(BASE64_CHARS[((n >> 12) & 0x3F) as usize] as char);
            result.push(if i + 1 < bytes.len() {
                BASE64_CHARS[((n >> 6) & 0x3F) as usize] as char
            } else {
                '='
            });
            result.push(if i + 2 < bytes.len() {
                BASE64_CHARS[(n & 0x3F) as usize] as char
            } else {
                '='
            });

            i += 3;
        }

        result
    }

    /// Decode base64 to bytes (simplified implementation)
    pub fn from_base64(s: &str) -> Result<Vec<u8>, String> {
        let decode_char = |c: char| -> Result<u8, String> {
            match c {
                'A'..='Z' => Ok((c as u8) - b'A'),
                'a'..='z' => Ok((c as u8) - b'a' + 26),
                '0'..='9' => Ok((c as u8) - b'0' + 52),
                '+' => Ok(62),
                '/' => Ok(63),
                '=' => Ok(0),
                _ => Err(format!("Invalid base64 character: {}", c)),
            }
        };

        let chars: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();
        if chars.len() % 4 != 0 {
            return Err("Base64 string length must be multiple of 4".to_string());
        }

        let mut result = Vec::new();

        for chunk in chars.chunks(4) {
            let b1 = decode_char(chunk[0])?;
            let b2 = decode_char(chunk[1])?;
            let b3 = decode_char(chunk[2])?;
            let b4 = decode_char(chunk[3])?;

            let n = ((b1 as u32) << 18) | ((b2 as u32) << 12) | ((b3 as u32) << 6) | (b4 as u32);

            result.push((n >> 16) as u8);
            if chunk[2] != '=' {
                result.push((n >> 8) as u8);
            }
            if chunk[3] != '=' {
                result.push(n as u8);
            }
        }

        Ok(result)
    }
}

/// Bit manipulation utilities
pub struct BitOps;

impl BitOps {
    pub fn get_bit(value: u64, bit: u8) -> bool {
        if bit >= 64 {
            return false;
        }
        (value >> bit) & 1 == 1
    }

    pub fn set_bit(value: u64, bit: u8) -> u64 {
        if bit >= 64 {
            return value;
        }
        value | (1 << bit)
    }

    pub fn clear_bit(value: u64, bit: u8) -> u64 {
        if bit >= 64 {
            return value;
        }
        value & !(1 << bit)
    }

    pub fn toggle_bit(value: u64, bit: u8) -> u64 {
        if bit >= 64 {
            return value;
        }
        value ^ (1 << bit)
    }

    pub fn count_ones(value: u64) -> u32 {
        value.count_ones()
    }

    pub fn count_zeros(value: u64) -> u32 {
        value.count_zeros()
    }

    pub fn rotate_left(value: u64, n: u32) -> u64 {
        value.rotate_left(n)
    }

    pub fn rotate_right(value: u64, n: u32) -> u64 {
        value.rotate_right(n)
    }

    pub fn reverse_bits(value: u64) -> u64 {
        value.reverse_bits()
    }
}

/// Struct packing/unpacking for C compatibility
pub struct StructPacker;

impl StructPacker {
    /// Pack values according to format string
    /// Format: 'b' = i8, 'B' = u8, 'h' = i16, 'H' = u16, 'i' = i32, 'I' = u32, 'q' = i64, 'Q' = u64
    pub fn pack(format: &str, values: &[Value], order: ByteOrder) -> Result<Vec<u8>, String> {
        let mut buffer = ByteBuffer::new();
        buffer.set_byte_order(order);

        let format_chars: Vec<char> = format.chars().collect();
        if format_chars.len() != values.len() {
            return Err(format!(
                "Format string length ({}) doesn't match values length ({})",
                format_chars.len(),
                values.len()
            ));
        }

        for (i, &fmt) in format_chars.iter().enumerate() {
            let value = &values[i];

            match fmt {
                'b' => {
                    let v = value
                        .as_integer()
                        .ok_or("Expected integer for 'b' format")?;
                    buffer.write_i8(v as i8);
                }
                'B' => {
                    let v = value
                        .as_integer()
                        .ok_or("Expected integer for 'B' format")?;
                    buffer.write_u8(v as u8);
                }
                'h' => {
                    let v = value
                        .as_integer()
                        .ok_or("Expected integer for 'h' format")?;
                    buffer.write_i16(v as i16);
                }
                'H' => {
                    let v = value
                        .as_integer()
                        .ok_or("Expected integer for 'H' format")?;
                    buffer.write_u16(v as u16);
                }
                'i' => {
                    let v = value
                        .as_integer()
                        .ok_or("Expected integer for 'i' format")?;
                    buffer.write_i32(v as i32);
                }
                'I' => {
                    let v = value
                        .as_integer()
                        .ok_or("Expected integer for 'I' format")?;
                    buffer.write_u32(v as u32);
                }
                'q' => {
                    let v = value
                        .as_integer()
                        .ok_or("Expected integer for 'q' format")?;
                    buffer.write_i64(v);
                }
                'Q' => {
                    let v = value
                        .as_integer()
                        .ok_or("Expected integer for 'Q' format")?;
                    buffer.write_u64(v as u64);
                }
                'f' => {
                    let v = value.as_float().ok_or("Expected float for 'f' format")?;
                    buffer.write_f32(v as f32);
                }
                'd' => {
                    let v = value.as_float().ok_or("Expected float for 'd' format")?;
                    buffer.write_f64(v);
                }
                _ => return Err(format!("Unknown format character: '{}'", fmt)),
            }
        }

        Ok(buffer.to_vec())
    }

    /// Unpack bytes according to format string
    pub fn unpack(format: &str, bytes: &[u8], order: ByteOrder) -> Result<Vec<Value>, String> {
        let mut buffer = ByteBuffer::from_bytes(bytes.to_vec());
        buffer.set_byte_order(order);

        let mut result = Vec::new();

        for fmt in format.chars() {
            let value = match fmt {
                'b' => Value::Integer(buffer.read_i8()? as i64),
                'B' => Value::Integer(buffer.read_u8()? as i64),
                'h' => Value::Integer(buffer.read_i16()? as i64),
                'H' => Value::Integer(buffer.read_u16()? as i64),
                'i' => Value::Integer(buffer.read_i32()? as i64),
                'I' => Value::Integer(buffer.read_u32()? as i64),
                'q' => Value::Integer(buffer.read_i64()?),
                'Q' => Value::Integer(buffer.read_u64()? as i64),
                'f' => Value::Float(buffer.read_f32()? as f64),
                'd' => Value::Float(buffer.read_f64()?),
                _ => return Err(format!("Unknown format character: '{}'", fmt)),
            };
            result.push(value);
        }

        Ok(result)
    }

    /// Calculate size needed for format string
    pub fn calcsize(format: &str) -> usize {
        let mut size = 0;
        for fmt in format.chars() {
            size += match fmt {
                'b' | 'B' => 1,
                'h' | 'H' => 2,
                'i' | 'I' | 'f' => 4,
                'q' | 'Q' | 'd' => 8,
                _ => 0,
            };
        }
        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_order_parsing() {
        assert_eq!(
            ByteOrder::from_string("little").unwrap(),
            ByteOrder::LittleEndian
        );
        assert_eq!(ByteOrder::from_string("big").unwrap(), ByteOrder::BigEndian);
        assert_eq!(ByteOrder::from_string("native").unwrap(), ByteOrder::Native);
    }

    #[test]
    fn test_byte_buffer_write_read() {
        let mut buffer = ByteBuffer::new();
        buffer.write_u8(42);
        buffer.write_i16(-1000);
        buffer.write_u32(123456);

        buffer.reset();
        assert_eq!(buffer.read_u8().unwrap(), 42);
        assert_eq!(buffer.read_i16().unwrap(), -1000);
        assert_eq!(buffer.read_u32().unwrap(), 123456);
    }

    #[test]
    fn test_hex_encoding() {
        let data = vec![0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
        let hex = BinaryEncoder::to_hex(&data);
        assert_eq!(hex, "123456789abcdef0");

        let decoded = BinaryEncoder::from_hex(&hex).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_base64_encoding() {
        let data = b"Hello, World!";
        let encoded = BinaryEncoder::to_base64(data);
        let decoded = BinaryEncoder::from_base64(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_bit_operations() {
        let value = 0b10101010u64;
        assert!(BitOps::get_bit(value, 1));
        assert!(!BitOps::get_bit(value, 0));

        let value = BitOps::set_bit(value, 0);
        assert!(BitOps::get_bit(value, 0));

        let value = BitOps::clear_bit(value, 1);
        assert!(!BitOps::get_bit(value, 1));

        assert_eq!(BitOps::count_ones(0b11111111), 8);
    }

    #[test]
    fn test_struct_packing() {
        let values = vec![Value::Integer(42), Value::Integer(1000), Value::Float(3.14)];
        let packed = StructPacker::pack("Bhd", &values, ByteOrder::LittleEndian).unwrap();

        let unpacked = StructPacker::unpack("Bhd", &packed, ByteOrder::LittleEndian).unwrap();
        assert_eq!(unpacked.len(), 3);
    }

    #[test]
    fn test_calcsize() {
        assert_eq!(StructPacker::calcsize("BHI"), 7); // 1 + 2 + 4
        assert_eq!(StructPacker::calcsize("qd"), 16); // 8 + 8
    }
}
