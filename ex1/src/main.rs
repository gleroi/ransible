use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::io;

mod module {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    pub struct Response<T>
    {
        pub changed: bool,
        pub msg: String,
        pub result: T,
    }

    #[derive(Deserialize)]
    pub struct Request<T>
    {
        pub _ansible_no_log: bool,
        pub _ansible_verbosity: u8,
        #[serde(flatten)]
        pub args: T,
    }
}

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
    let request : module::Request<Args> = serde_json::from_str(&input)?;

    serde_json::to_writer_pretty(
        &mut stdout,
        &module::Response {
            changed: true,
            msg: "default message".to_string(),
            result: request.args,
        },
    )?;
    Ok(())
}
