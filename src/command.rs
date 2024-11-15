use zvariant::Type;
use serde::{Deserialize, Serialize};
use crate::mode;
use crate::status;
use crate::output_settings::OutputSettings;

// Hack to get around the restriction on single types for the mpsc channels we're using for internal communication.
#[derive(Debug)]
pub enum InternalCommand {
    SetOutputModeCommand(SetOutputModeCommand),
    SetOutputOncalendarCommand(SetOutputOncalendarCommand),
    SetOutputImagesCommand(SetOutputImagesCommand),
    GetOutputsSettingsCommand,
    ReloadCollectionCommand,
    GeneralResponse(GeneralResponse),
    GeneralResponseErrorVec(GeneralResponseErrorVec),
    GetOutputSettingsResponse(GetOutputSettingsResponse),
    ReloadCollectionResponse(ReloadCollectionResponse),
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct Command {
    mode: mode::Mode,
    outputs: Vec<String>,
    images: Vec<String>,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct GeneralResponse {
    pub status: status::Status,
    pub error: String,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct GeneralResponseErrorVec {
    pub status: status::Status,
    pub errors: Vec<String>,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct GetOutputSettingsResponse {
    pub status: status::Status,
    pub error: String,
    pub outputs_settings: Vec<OutputSettings>,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct ReloadCollectionResponse {
    pub status: status::Status,
    pub error: String,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct SetOutputModeCommand {
    pub output: String,
    pub mode: mode::Mode,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct SetOutputOncalendarCommand {
    pub output: String,
    pub oncalendar: String,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct SetOutputImagesCommand {
    pub output: String,
    pub images: Vec<String>,
}
