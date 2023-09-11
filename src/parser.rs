use nom::bytes::complete::take;
use nom::combinator::{map, map_res};
use nom::multi::count;
use nom::number::complete::{le_u32, le_u64, le_u8, *};
use nom::{bytes::complete::tag, IResult};
use std::fmt;

/// GGUF metadata value type
#[derive(Debug, Clone, Copy, PartialEq)]
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

/// GGUF metadata value
#[derive(PartialEq)]
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
    Array(Vec<GGUFMetadataValue>),
}

impl fmt::Debug for GGUFMetadataValue {
    /// display the short version for GGUF
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uint16(v) => write!(f, "{}u16", v),
            Self::Int16(v) => write!(f, "{}i16", v),
            Self::Uint32(v) => write!(f, "{}u32", v),
            Self::Int32(v) => write!(f, "{}i32", v),
            Self::Float32(v) => write!(f, "{}f32", v),
            Self::Uint64(v) => write!(f, "{}u64", v),
            Self::Int64(v) => write!(f, "{}i64", v),
            Self::Float64(v) => write!(f, "{}f64", v),
            Self::Bool(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Array(v) => write!(f, "{:?}", v),
            _ => write!(f, "unknown"),
        }
    }
}

/// GGUF metadata
#[derive(Debug, PartialEq)]
pub struct GGUFMetadata {
    pub key: String,
    pub value_type: GGUfMetadataValueType,
    pub value: GGUFMetadataValue,
}

/// GGUF header
#[derive(Debug, PartialEq)]
pub struct GGUFHeader {
    pub version: u32,
    pub tensor_count: u64,
    pub metadata: Vec<GGUFMetadata>,
}

impl GGUFHeader {
    pub fn read(data: &[u8]) -> Result<GGUFHeader, String> {
        let (_, header) = gguf_header(data).expect("failed to parse");
        Ok(header)
    }
}

/// parse gguf string
fn gguf_string(i: &[u8]) -> IResult<&[u8], String> {
    let (i, len) = le_u64(i)?;
    let (i, data) = map_res(take(len), std::str::from_utf8)(i)?;
    Ok((i, data.to_string()))
}

/// the magic of GGUF
fn magic(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("GGUF")(input)
}

/// parse value type of a metadata
fn gguf_metadata_value_type(i: &[u8]) -> IResult<&[u8], GGUfMetadataValueType> {
    map_res(le_u32, GGUfMetadataValueType::try_from)(i)
}

/// parse metadata value
fn gguf_metadata_value(
    value_type: GGUfMetadataValueType,
) -> impl FnMut(&[u8]) -> IResult<&[u8], GGUFMetadataValue> {
    move |i: &[u8]| {
        // parse all metadata value type
        match value_type {
            GGUfMetadataValueType::Uint8 => map(le_u8, GGUFMetadataValue::Uint8)(i),
            GGUfMetadataValueType::Int8 => map(le_i8, GGUFMetadataValue::Int8)(i),
            GGUfMetadataValueType::Uint16 => map(le_u16, GGUFMetadataValue::Uint16)(i),
            GGUfMetadataValueType::Int16 => map(le_i16, GGUFMetadataValue::Int16)(i),
            GGUfMetadataValueType::Uint32 => map(le_u32, GGUFMetadataValue::Uint32)(i),
            GGUfMetadataValueType::Int32 => map(le_i32, GGUFMetadataValue::Int32)(i),
            GGUfMetadataValueType::Float32 => map(le_f32, GGUFMetadataValue::Float32)(i),
            GGUfMetadataValueType::Uint64 => map(le_u64, GGUFMetadataValue::Uint64)(i),
            GGUfMetadataValueType::Int64 => map(le_i64, GGUFMetadataValue::Int64)(i),
            GGUfMetadataValueType::Float64 => map(le_f64, GGUFMetadataValue::Float64)(i),
            GGUfMetadataValueType::Bool => map_res(le_u8, |b| {
                if b == 0 {
                    Ok(GGUFMetadataValue::Bool(false))
                } else if b == 1 {
                    Ok(GGUFMetadataValue::Bool(true))
                } else {
                    Err("invalid bool value".to_string())
                }
            })(i),
            GGUfMetadataValueType::String => map(gguf_string, GGUFMetadataValue::String)(i),
            GGUfMetadataValueType::Array => {
                let (i, value_type) = gguf_metadata_value_type(i)?;
                let (i, len) = le_u64(i)?;
                let (i, v) = count(gguf_metadata_value(value_type), len as usize)(i)?;
                Ok((i, GGUFMetadataValue::Array(v)))
            }
        }
    }
}

/// parse metadata
fn gguf_metadata(i: &[u8]) -> IResult<&[u8], GGUFMetadata> {
    let (i, key) = gguf_string(i)?;
    let (i, value_type) = gguf_metadata_value_type(i)?;
    let (i, value) = gguf_metadata_value(value_type)(i)?;
    Ok((
        i,
        GGUFMetadata {
            key,
            value_type,
            value,
        },
    ))
}

/// parse header
fn gguf_header(i: &[u8]) -> IResult<&[u8], GGUFHeader> {
    let (i, _) = magic(i)?;
    let (i, version) = le_u32(i)?;
    let (i, tensor_count) = le_u64(i)?;
    let (i, metadata_count) = le_u64(i)?;
    let mut metadata = Vec::new();
    let mut i = i;
    for _ in 0..metadata_count {
        let (i2, m) = gguf_metadata(i)?;
        metadata.push(m);
        i = i2;
    }
    Ok((
        i,
        GGUFHeader {
            version,
            tensor_count,
            metadata,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_magic() {
        let data = &[0x47, 0x47, 0x55, 0x46];
        let result = magic(data);
        assert_eq!(result, Ok((&[][..], &data[..])));
    }
}
