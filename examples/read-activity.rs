use rust_fit::profile::types::{File as FileType, MesgNum, Sport};
use rust_fit::record::{read_fit, DataField};
use rust_fit::stream_reader::StreamReader;
use std::fs::File;
use std::io::BufReader;
use std::{env, error::Error};

const SEMICIRCLES_SCALE: f64 = 11930465.0;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("must provide a file name to read");
    }

    let file = File::open(&args[1])?;
    let mut reader = StreamReader::new(BufReader::new(file));

    let fit = read_fit(&mut reader)?;

    for mesg in fit.data {
        match mesg.mesg_num {
            MesgNum::FileId => {
                if let Some(DataField::Enum(file_field)) = mesg.fields.get(&0) {
                    let file = FileType::from(*file_field);
                    println!("File mesg: type = {:?}", file);
                } else {
                    println!("no file type defined");
                };
            }
            MesgNum::Record => {
                let timestamp = if let Some(DataField::UInt32(timestamp)) = mesg.fields.get(&253) {
                    *timestamp
                } else {
                    0
                };

                let lat: f32 = if let Some(DataField::SInt32(lat)) = mesg.fields.get(&0) {
                    (*lat as f64 / SEMICIRCLES_SCALE) as f32
                } else {
                    0.0
                };

                let long: f32 = if let Some(DataField::SInt32(long)) = mesg.fields.get(&1) {
                    (*long as f64 / SEMICIRCLES_SCALE) as f32
                } else {
                    0.0
                };

                let hr = if let Some(DataField::UInt8(hr)) = mesg.fields.get(&3) {
                    *hr
                } else {
                    0
                };

                println!(
                    "Record msg: timestamp = {}; lat long = {},{}; hr = {}",
                    timestamp, lat, long, hr
                );
            }
            MesgNum::Session => {
                let total_calories = if let Some(DataField::UInt16(calories)) = mesg.fields.get(&11) {
                    *calories
                } else {
                    0
                };

                let sport: Sport = if let Some(DataField::Enum(sport)) = mesg.fields.get(&5) {
                    (*sport).into()
                } else {
                    Sport::UnknownValue(0)
                };

                let avg_hr = if let Some(DataField::UInt8(hr)) = mesg.fields.get(&16) {
                    *hr
                } else {
                    0
                };

                let elapsed_time = if let Some(DataField::UInt32(time)) = mesg.fields.get(&7) {
                    (*time as f64) / 1000.0 / 60.0
                } else {
                    0.0
                };

                println!(
                    "Session msg: sport = {:?}; time = {:.2}; total calories = {}; avg hr = {};",
                    sport, elapsed_time, total_calories, avg_hr
                );
            }
            _ => (),
        }
    }

    Ok(())
}
