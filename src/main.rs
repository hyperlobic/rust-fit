use std::{env, error::Error};
use rust_fit::fit_record::read_fit;
use rust_fit::stream_reader::StreamReader;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    let file = File::open(&args[1])?;
    let mut reader = StreamReader::new(BufReader::new(file));

    let fit = read_fit(&mut reader)?;

    let json = serde_json::to_string(&fit)?;

    println!("{}", json);

    Ok(())
}
