use bytes::{BufMut, BytesMut};
use gguf::GGUFFile;
use std::borrow::Borrow;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const BYTES_PER_READ: usize = 1_000_000;

type E = Box<dyn std::error::Error>;

fn main() -> Result<(), E> {
    for fname in env::args().skip(1) {
        read_gguf_file(&fname)?;
    }
    Ok(())
}

fn read_gguf_file(fname: &str) -> Result<GGUFFile, E> {
    let mut buffer = BytesMut::with_capacity(BYTES_PER_READ);
    let mut reader = BufReader::with_capacity(BYTES_PER_READ, File::open(fname)?);
    loop {
        let read: &[u8] = reader.fill_buf()?;
        if read.is_empty() {
            return Err("Failed to read gguf file".into());
        }
        let content_length = read.len();
        println!("read {} bytes", content_length);
        buffer.put(read);
        reader.consume(content_length);
        match GGUFFile::read(buffer.borrow()) {
            Ok(Some(file)) => {
                println!("fileq: {}", serde_yaml::to_string(&file)?);
                return Ok(file);
            }
            Ok(None) => {
                println!("incomplete");
            }
            Err(e) => {
                println!("error: {:?}", e);
                return Err(e.into());
            }
        }
        buffer.reserve(BYTES_PER_READ);
    }
}
