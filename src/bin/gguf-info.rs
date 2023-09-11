use gguf::parser::GGUFHeader;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        let f = File::open(arg)?;
        let mut reader = BufReader::with_capacity(32_000_000, f);
        reader.fill_buf()?;
        let header = GGUFHeader::read(reader.buffer())?;
        println!("{:?}, {}", header, reader.stream_position()?);
    }
    Ok(())
}
