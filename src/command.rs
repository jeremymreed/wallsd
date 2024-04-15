use zvariant::Type;
use serde::{Deserialize, Serialize};
use crate::mode;

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct Command {
    mode: mode::Mode,
    outputs: Vec<String>,
    images: Vec<String>,
}
