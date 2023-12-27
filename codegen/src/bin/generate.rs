use std::error::Error;

use rust_fit_codegen::gen_profile::generate_code_types;
use rust_fit_codegen::profile_types::{convert_types, read_types};

fn main() -> Result<(), Box<dyn Error>> {
    let types_csv = read_types("./fit/fit-profile-types.csv")?;
    let types = convert_types(&types_csv)?;
    generate_code_types(&types, "./src/profile_types.rs");

    Ok(())
}
