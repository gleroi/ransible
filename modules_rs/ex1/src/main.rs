use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::io;
use ansible;

//TODO:
//- add a module trait/struct to provide common functionalities
//- log/output functionalities
//- return value helper

#[derive(Deserialize, Serialize)]
struct Args {
    plop: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout_handle = io::stdout();
    let mut stdout = stdout_handle.lock();

    let mut args = env::args();
    let input = fs::read_to_string(args.nth(1).unwrap())?;
    let request : ansible::Request<Args> = serde_json::from_str(&input)?;

    serde_json::to_writer_pretty(
        &mut stdout,
        &ansible::Response {
            changed: true,
            msg: "default message".to_string(),
            result: request.args,
        },
    )?;
    Ok(())
}
