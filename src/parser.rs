use crate::{
    GGMLType, GGUFFile, GGUFHeader, GGUFMetadata, GGUFMetadataArrayValue, GGUFMetadataValue,
    GGUFTensorInfo, GGUfMetadataValueType,
};
use nom::bytes::streaming::take;
use nom::combinator::{map, map_res};
use nom::multi::count;
use nom::number::streaming::{le_u32, le_u64, le_u8, *};
use nom::{bytes::streaming::tag, IResult};

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
                let value = GGUFMetadataValue::Array(GGUFMetadataArrayValue {
                    value_type,
                    len,
                    value: v,
                });
                Ok((i, value))
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
    let (i, metadata) = count(gguf_metadata, metadata_count as usize)(i)?;
    Ok((
        i,
        GGUFHeader {
            version,
            tensor_count,
            metadata,
        },
    ))
}

/// parse tensor info
fn gguf_tensor_info(i: &[u8]) -> IResult<&[u8], GGUFTensorInfo> {
    let (i, name) = gguf_string(i)?;
    let (i, n_dimensions) = le_u32(i)?;
    let (i, dimensions) = count(le_u64, n_dimensions as usize)(i)?;
    let (i, tensor_type) = map_res(le_u32, GGMLType::try_from)(i)?;
    let (i, offset) = le_u64(i)?;
    Ok((
        i,
        GGUFTensorInfo {
            name,
            dimensions,
            tensor_type,
            offset,
        },
    ))
}

/// parse file
pub(crate) fn gguf_file(i: &[u8]) -> IResult<&[u8], GGUFFile> {
    let (i, header) = gguf_header(i)?;
    let (i, tensors) = count(gguf_tensor_info, header.tensor_count as usize)(i)?;
    Ok((i, GGUFFile { header, tensors }))
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
