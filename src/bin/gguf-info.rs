use gguf::GGUFFile;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        let f = File::open(arg)?;
        let mut reader = BufReader::with_capacity(8_000_000, f);
        reader.fill_buf()?;
        let header = GGUFFile::read(reader.buffer())?;
        println!("{}", serde_yaml::to_string(&header)?);
    }
    Ok(())
}
