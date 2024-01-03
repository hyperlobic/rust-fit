use rust_fit::profile::types::{File as FileType, MesgNum, Sport};
use rust_fit::Reader;
use rust_fit::Data;
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
    let mut reader = Reader::from_reader(BufReader::new(file));

    for mesg in reader.data_messages() {
        let mesg = mesg?;

        match mesg.mesg_num() {
            MesgNum::FileId => {
                if let Some(Data::Enum(file_field)) = mesg.data(0) {
                    let file = FileType::from(*file_field);
                    println!("File mesg: type = {:?}", file);
                } else {
                    println!("no file type defined");
                };
            }
            MesgNum::Record => {
                let timestamp = if let Some(Data::Uint32(timestamp)) = mesg.data(253) {
                    *timestamp
                } else {
                    0
                };

                let lat: f32 = if let Some(Data::Sint32(lat)) = mesg.data(0) {
                    (*lat as f64 / SEMICIRCLES_SCALE) as f32
                } else {
                    0.0
                };

                let long: f32 = if let Some(Data::Sint32(long)) = mesg.data(1) {
                    (*long as f64 / SEMICIRCLES_SCALE) as f32
                } else {
                    0.0
                };

                let hr = if let Some(Data::Uint8(hr)) = mesg.data(3) {
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
                let total_calories = if let Some(Data::Uint16(calories)) = mesg.data(11) {
                    *calories
                } else {
                    0
                };

                let sport: Sport = if let Some(Data::Enum(sport)) = mesg.data(5) {
                    (*sport).into()
                } else {
                    Sport::UnknownValue(0)
                };

                let avg_hr = if let Some(Data::Uint8(hr)) = mesg.data(16) {
                    *hr
                } else {
                    0
                };

                let elapsed_time = if let Some(Data::Uint32(time)) = mesg.data(7) {
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
