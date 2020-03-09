#[macro_use]
extern crate lambda_runtime as lambda;

use lambda::error::HandlerError;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(score_acronym);
    Ok(())
}

fn score_acronym(acro: String, _: lambda::Context) -> Result<u32, HandlerError> {
    Ok(match acro.as_str() {
        "MBG" => u32::max_value(),
        _ => 0,
    })
}
