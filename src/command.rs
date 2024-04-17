use zvariant::Type;
use serde::{Deserialize, Serialize};
use crate::mode;
use crate::status;

// Hack to get around the restriction on single types for the mpsc channels we're using for internal communication.
pub enum InternalCommand {
    SetOutputModeCommand(SetOutputModeCommand),
    SetOutputModeResponse(SetOutputModeResponse)
}

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
