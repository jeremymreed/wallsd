use zvariant::Type;
use serde::{Deserialize, Serialize};
use crate::mode;
use crate::status;

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct Command {
    mode: mode::Mode,
    outputs: Vec<String>,
    images: Vec<String>,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct SetOutputModeCommand {
    pub output: String,
    pub mode: mode::Mode,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct SetOutputModeResponse {
    pub status: status::Status,
    pub error: String,
}
