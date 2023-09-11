use gguf::parser::parse_gguf_header;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        let data = fs::read(arg)?;
        println!("{:?}", parse_gguf_header(&data));
    }
    Ok(())
}
