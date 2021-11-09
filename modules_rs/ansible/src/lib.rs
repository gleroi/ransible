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

