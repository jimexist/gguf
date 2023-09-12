//! # GGUF file parsing and struct definitions
pub mod parser;
use parser::gguf_file;
extern crate serde;
use serde::ser::SerializeSeq;

/// GGUF metadata value type
#[derive(serde::Serialize, Clone, Copy, PartialEq)]
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

#[derive(PartialEq, Clone, Copy, serde::Serialize)]
pub enum GGMLType {
    GgmlF32 = 0,
    GgmlF16 = 1,
    GgmlQ4_0 = 2,
    GgmlQ4_1 = 3,
    GgmlQ5_0 = 6,
    GgmlQ5_1 = 7,
    GgmlQ8_0 = 8,
    GgmlQ8_1 = 9,
    GgmlQ2K = 10,
    GgmlQ3K = 11,
    GgmlQ4K = 12,
    GgmlQ5K = 13,
    GgmlQ6K = 14,
    GgmlQ8K = 15,
    GgmlI8 = 16,
    GgmlI16 = 17,
    GgmlI32 = 18,
    GgmlCount = 19,
}

impl TryFrom<u32> for GGMLType {
    type Error = String;

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        Ok(match item {
            0 => GGMLType::GgmlF32,
            1 => GGMLType::GgmlF16,
            2 => GGMLType::GgmlQ4_0,
            3 => GGMLType::GgmlQ4_1,
            6 => GGMLType::GgmlQ5_0,
            7 => GGMLType::GgmlQ5_1,
            8 => GGMLType::GgmlQ8_0,
            9 => GGMLType::GgmlQ8_1,
            10 => GGMLType::GgmlQ2K,
            11 => GGMLType::GgmlQ3K,
            12 => GGMLType::GgmlQ4K,
            13 => GGMLType::GgmlQ5K,
            14 => GGMLType::GgmlQ6K,
            15 => GGMLType::GgmlQ8K,
            16 => GGMLType::GgmlI8,
            17 => GGMLType::GgmlI16,
            18 => GGMLType::GgmlI32,
            19 => GGMLType::GgmlCount,
            _ => return Err(format!("invalid GGML type 0x{:x}", item)),
        })
    }
}

#[derive(PartialEq, serde::Serialize)]
pub struct GGUFTensorInfo {
    pub name: String,
    pub dimensions: Vec<u64>,
    pub tensor_type: GGMLType,
    pub offset: u64,
}

#[derive(PartialEq, serde::Serialize)]
pub struct GGUFFile {
    pub header: GGUFHeader,
    pub tensors: Vec<GGUFTensorInfo>,
}

impl GGUFFile {
    pub fn read(buf: &[u8]) -> Result<GGUFFile, String> {
        let (_, result) = gguf_file(buf).expect("failed to parse gguf file");
        Ok(result)
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

#[derive(PartialEq, serde::Serialize)]
pub struct GGUFMetadataArrayValue {
    #[serde(rename = "type")]
    pub value_type: GGUfMetadataValueType,
    pub len: u64,
    #[serde(serialize_with = "serialize_array")]
    pub value: Vec<GGUFMetadataValue>,
}

/// serialize_array
fn serialize_array<S>(v: &Vec<GGUFMetadataValue>, s: S) -> Result<S::Ok, S::Error>
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
