use bytes::{BufMut, BytesMut};
use clap::{Parser, ValueEnum};
use comfy_table::Table;
use gguf::{GGUFFile, GGUFMetadataValue};
use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Yaml,
    Json,
    Table,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the file to read
    path: std::path::PathBuf,

    /// Size of read buffer (grows linearly)
    #[arg(long, default_value_t = 1_000_000)]
    read_buffer_size: usize,

    #[arg(short = 't', long, value_enum, default_value_t = OutputFormat::Table)]
    output_format: OutputFormat,
}

type E = Box<dyn std::error::Error>;

fn main() -> Result<(), E> {
    let args = Args::parse();
    let read_file = read_gguf_file(args.path, args.read_buffer_size)?;
    match args.output_format {
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&read_file)?);
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&read_file)?);
        }
        OutputFormat::Table => {
            let metadata = build_metadata_table(&read_file)?;
            println!("Metadata:");
            println!("{metadata}");
            let tensor_info = build_tensor_info_table(&read_file)?;
            println!("Tensors:");
            println!("{tensor_info}");
        }
    }
    Ok(())
}

fn build_metadata_table(read_file: &GGUFFile) -> Result<String, E> {
    let mut table = Table::new();
    table.set_header(vec![
        "#".to_string(),
        "Key".to_string(),
        "Type".to_string(),
        "Value".to_string(),
    ]);
    for (idx, metadata) in read_file.header.metadata.iter().enumerate() {
        // write value type, but for array also include array length
        let value_type_len_postfix = match &metadata.value {
            GGUFMetadataValue::Array(array_value) => format!(" ({})", array_value.len),
            _ => "".to_string(),
        };
        let value_type_col = format!("{:?}{}", metadata.value_type, value_type_len_postfix);
        table.add_row(vec![
            format!("{}", idx + 1),
            metadata.key.clone(),
            value_type_col,
            format!("{:?}", metadata.value),
        ]);
    }
    Ok(table.to_string())
}

fn build_tensor_info_table(read_file: &GGUFFile) -> Result<String, E> {
    let mut table = Table::new();
    table.set_header(vec![
        "#".to_string(),
        "Name".to_string(),
        "Type".to_string(),
        "Dimensions".to_string(),
        "Offset".to_string(),
    ]);
    for (idx, tensor) in read_file.tensors.iter().enumerate() {
        table.add_row(vec![
            format!("{}", idx + 1),
            tensor.name.clone(),
            format!("{:?}", tensor.tensor_type),
            format!("{:?}", tensor.dimensions),
            format!("{}", tensor.offset),
        ]);
    }
    Ok(table.to_string())
}

/// Read a gguf file by trying out different buffer sizes
fn read_gguf_file(fname: PathBuf, read_buffer_size: usize) -> Result<GGUFFile, E> {
    let mut buffer = BytesMut::with_capacity(read_buffer_size);
    let mut reader = BufReader::with_capacity(read_buffer_size, File::open(fname)?);
    loop {
        let read: &[u8] = reader.fill_buf()?;
        if read.is_empty() {
            return Err("Failed to read gguf file".into());
        }
        let content_length = read.len();
        buffer.put(read);
        reader.consume(content_length);
        match GGUFFile::read(buffer.borrow()) {
            Ok(Some(file)) => {
                return Ok(file);
            }
            Ok(None) => {
                // skip
            }
            Err(e) => {
                return Err(e.into());
            }
        }
        buffer.reserve(read_buffer_size);
    }
}
