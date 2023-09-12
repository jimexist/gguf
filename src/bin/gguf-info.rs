use bytes::{BufMut, BytesMut};
use clap::{Parser, ValueEnum};
use gguf::GGUFFile;
use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Yaml,
    Json,
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

    #[arg(short = 't', long, value_enum, default_value_t = OutputFormat::Yaml)]
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
    }
    Ok(())
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
