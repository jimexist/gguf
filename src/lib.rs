//! # GGUF file parsing and struct definitions
pub mod parser;
use parser::{gguf_file, gguf_header};
use std::fmt;
extern crate serde;
use serde::ser::SerializeSeq;

/// GGUF metadata value type
#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq)]
pub enum GGUfMetadataValueType {
    /// The value is a 8-bit unsigned integer.
    Uint8 = 0,
    /// The value is a 8-bit signed integer.
    Int8 = 1,
    /// The value is a 16-bit unsigned little-endian integer.
    Uint16 = 2,
    /// The value is a 16-bit signed little-endian integer.
    Int16 = 3,
    /// The value is a 32-bit unsigned little-endian integer.
    Uint32 = 4,
    /// The value is a 32-bit signed little-endian integer.
    Int32 = 5,
    /// The value is a 32-bit IEEE754 floating point number.
    Float32 = 6,
    /// The value is a boolean.
    Bool = 7,
    /// The value is a UTF-8 non-null-terminated string, with length prepended.
    String = 8,
    /// The value is an array of other values, with the length and type prepended.
    Array = 9,
    /// The value is a 64-bit unsigned little-endian integer.
    Uint64 = 10,
    /// The value is a 64-bit signed little-endian integer.
    Int64 = 11,
    /// The value is a 64-bit IEEE754 floating point number.
    Float64 = 12,
}

impl TryFrom<u32> for GGUfMetadataValueType {
    type Error = String;

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        Ok(match item {
            0 => GGUfMetadataValueType::Uint8,
            1 => GGUfMetadataValueType::Int8,
            2 => GGUfMetadataValueType::Uint16,
            3 => GGUfMetadataValueType::Int16,
            4 => GGUfMetadataValueType::Uint32,
            5 => GGUfMetadataValueType::Int32,
            6 => GGUfMetadataValueType::Float32,
            7 => GGUfMetadataValueType::Bool,
            8 => GGUfMetadataValueType::String,
            9 => GGUfMetadataValueType::Array,
            10 => GGUfMetadataValueType::Uint64,
            11 => GGUfMetadataValueType::Int64,
            12 => GGUfMetadataValueType::Float64,
            _ => return Err(format!("invalid metadata type 0x{:x}", item)),
        })
    }
}

/// GGUF header
#[derive(PartialEq, serde::Serialize)]
pub struct GGUFHeader {
    pub version: u32,
    pub tensor_count: u64,
    pub metadata: Vec<GGUFMetadata>,
}

impl GGUFHeader {
    pub fn read(buf: &[u8]) -> Result<Option<GGUFHeader>, String> {
        match gguf_header(buf) {
            Ok((_, header)) => Ok(Some(header)),
            Err(nom::Err::Incomplete(_)) => Ok(None),
            Err(e) => Err(format!(
                "Failed to parse GGUF header, please check for file integrity: {:?}",
                e.map_input(|i| {
                    // print only the next few bytes as hex
                    let len = i.len().min(16);
                    let mut s = String::new();
                    for b in &i[..len] {
                        s.push_str(&format!("0x{:02x} ", b));
                    }
                    s
                })
            )),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, serde::Serialize)]
pub enum GGMLType {
    F32 = 0,
    F16 = 1,
    Q4_0 = 2,
    Q4_1 = 3,
    Q5_0 = 6,
    Q5_1 = 7,
    Q8_0 = 8,
    Q8_1 = 9,
    Q2K = 10,
    Q3K = 11,
    Q4K = 12,
    Q5K = 13,
    Q6K = 14,
    Q8K = 15,
    I8 = 16,
    I16 = 17,
    I32 = 18,
    Count = 19,
}

impl TryFrom<u32> for GGMLType {
    type Error = String;

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        Ok(match item {
            0 => GGMLType::F32,
            1 => GGMLType::F16,
            2 => GGMLType::Q4_0,
            3 => GGMLType::Q4_1,
            6 => GGMLType::Q5_0,
            7 => GGMLType::Q5_1,
            8 => GGMLType::Q8_0,
            9 => GGMLType::Q8_1,
            10 => GGMLType::Q2K,
            11 => GGMLType::Q3K,
            12 => GGMLType::Q4K,
            13 => GGMLType::Q5K,
            14 => GGMLType::Q6K,
            15 => GGMLType::Q8K,
            16 => GGMLType::I8,
            17 => GGMLType::I16,
            18 => GGMLType::I32,
            19 => GGMLType::Count,
            _ => return Err(format!("invalid GGML type 0x{:x}", item)),
        })
    }
}

#[derive(PartialEq, Debug, serde::Serialize)]
pub struct GGUFTensorInfo {
    pub name: String,
    pub dimensions: Vec<u64>,
    #[serde(rename = "type")]
    pub tensor_type: GGMLType,
    pub offset: u64,
}

#[derive(PartialEq, serde::Serialize)]
pub struct GGUFFile {
    pub header: GGUFHeader,
    pub tensors: Vec<GGUFTensorInfo>,
}

impl GGUFFile {
    pub fn read(buf: &[u8]) -> Result<Option<GGUFFile>, String> {
        match gguf_file(buf) {
            Ok((_, file)) => Ok(Some(file)),
            Err(nom::Err::Incomplete(_)) => Ok(None),
            Err(e) => Err(format!(
                "Failed to parse GGUF file, please check for file integrity: {:?}",
                e.map_input(|i| {
                    // print only the next few bytes as hex
                    let len = i.len().min(16);
                    let mut s = String::new();
                    for b in &i[..len] {
                        s.push_str(&format!("0x{:02x} ", b));
                    }
                    s
                })
            )),
        }
    }
}

/// GGUF metadata
#[derive(PartialEq, serde::Serialize)]
pub struct GGUFMetadata {
    pub key: String,
    #[serde(rename = "type")]
    pub value_type: GGUfMetadataValueType,
    pub value: GGUFMetadataValue,
}

/// GGUF metadata value
#[derive(PartialEq, serde::Serialize)]
#[serde(untagged)]
pub enum GGUFMetadataValue {
    Uint8(u8),
    Int8(i8),
    Uint16(u16),
    Int16(i16),
    Uint32(u32),
    Int32(i32),
    Float32(f32),
    Uint64(u64),
    Int64(i64),
    Float64(f64),
    Bool(bool),
    String(String),
    Array(GGUFMetadataArrayValue),
}

impl fmt::Debug for GGUFMetadataValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uint8(v) => write!(f, "{}", v),
            Self::Int8(v) => write!(f, "{}", v),
            Self::Uint16(v) => write!(f, "{}", v),
            Self::Int16(v) => write!(f, "{}", v),
            Self::Uint32(v) => write!(f, "{}", v),
            Self::Int32(v) => write!(f, "{}", v),
            Self::Float32(v) => write!(f, "{}", v),
            Self::Uint64(v) => write!(f, "{}", v),
            Self::Int64(v) => write!(f, "{}", v),
            Self::Float64(v) => write!(f, "{}", v),
            Self::Bool(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Array(v) => {
                // write up to 3 values
                let len = v.value.len().min(3);
                for i in 0..len {
                    write!(f, "{:?}", v.value[i])?;
                    if i < len - 1 {
                        write!(f, ", ")?;
                    }
                }
                if v.value.len() > 3 {
                    write!(f, ", ...")?;
                }
                Ok(())
            }
        }
    }
}

#[derive(PartialEq, Debug, serde::Serialize)]
pub struct GGUFMetadataArrayValue {
    #[serde(rename = "type")]
    pub value_type: GGUfMetadataValueType,
    pub len: u64,
    #[serde(serialize_with = "serialize_array")]
    pub value: Vec<GGUFMetadataValue>,
}

/// serialize_array
fn serialize_array<S>(v: &[GGUFMetadataValue], s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let len = v.len().min(3);
    let has_more = v.len() > 3;
    let mut seq = s.serialize_seq(Some(if has_more { 4 } else { len }))?;
    for e in &v[..len] {
        seq.serialize_element(e)?;
    }
    if has_more {
        let ellipse = format!("... and {} more items", v.len() - 3);
        seq.serialize_element(&ellipse)?;
    }
    seq.end()
}
