use std::collections::HashMap;
use crate::output_settings::OutputSettings;
use crate::systemd_analyze;
use crate::status;
use crate::command;
use crate::output::Output;
use crate::config::Config;
use crate::mode;
use crate::collection;

// The program's current state.
pub struct State {
    pub config: Config,
    pub outputs: HashMap<String, Output>,
}

impl State {
    pub fn new() -> State {
        State {
            config: Config::load_config(),
            outputs: HashMap::new(),
        }
    }

    pub fn set_mode(&mut self, name: &String, mode: mode::Mode) -> Result<command::InternalCommand, command::InternalCommand> {
        match self.outputs.get_mut(name) {
            Some(output) => {
                output.mode = mode;
                let response = command::GeneralResponse {
                    status: status::Status::Success,
                    error: "".to_string(),
                };

                Ok(command::InternalCommand::GeneralResponse(response))
            },
            None => {
                tracing::error!("Output {} not found", name);
                let response = command::GeneralResponse {
                    status: status::Status::Failure,
                    error: format!("Output {} not found", name),
                };

                Err(command::InternalCommand::GeneralResponse(response))
            }
        }
    }

    pub fn set_oncalendar(&mut self, name: &String, oncalendar: String) -> Result<command::InternalCommand, command::InternalCommand> {
        match self.outputs.get_mut(name) {
            Some(output) => {
                match systemd_analyze::get_next_event(&oncalendar) {
                    Ok(_) => {
                        tracing::debug!("Valid oncalendar string: {:#?}", oncalendar);
                    },
                    Err(error) => {
                        tracing::error!("Invalid oncalendar string: {:#?}", error);
                        let response = command::GeneralResponse {
                            status: status::Status::Failure,
                            error: format!("Invalid oncalendar string: {:#?}", error),
                        };

                        return Err(command::InternalCommand::GeneralResponse(response));
                    }
                }
                output.oncalendar_string = oncalendar;
                let response = command::GeneralResponse {
                    status: status::Status::Success,
                    error: "".to_string(),
                };

                Ok(command::InternalCommand::GeneralResponse(response))
            },
            None => {
                tracing::error!("Output {} not found", name);
                let response = command::GeneralResponse {
                    status: status::Status::Failure,
                    error: format!("Output {} not found", name),
                };

                Err(command::InternalCommand::GeneralResponse(response))
            }
        }
    }

    pub fn set_images(&mut self, name: &String, images: Vec<String>) -> Result<command::InternalCommand, command::InternalCommand> {
        tracing::debug!("set_images called");

        match self.outputs.get_mut(name) {
            Some(output) => {
                let mut collection = collection::Collection::new();

                for image in images {
                    collection.process(&image);
                }

                output.images.clear();
                output.images = collection.collection.clone();

                Ok(command::InternalCommand::GeneralResponseErrorVec(command::GeneralResponseErrorVec {
                    status: status::Status::Success,
                    errors: collection.errors.clone(),
                }))
            },
            None => {
                tracing::error!("Output {} not found", name);
                let response = command::GeneralResponseErrorVec {
                    status: status::Status::Failure,
                    errors: vec![format!("Output {} not found", name)],
                };

                Err(command::InternalCommand::GeneralResponseErrorVec(response))
            }
        }
    }

    pub fn get_outputs_settings(&self) -> Result<command::InternalCommand, command::InternalCommand> {
        tracing::debug!("get_outputs_settings called");

        let mut outputs_settings = Vec::<OutputSettings>::new();

        for output in self.outputs.values() {
            outputs_settings.push(OutputSettings {
                name: output.name.clone(),
                mode: output.mode.clone(),
                oncalendar: output.oncalendar_string.clone(),
                current_wallpaper: String::from("This part isn't done yet."),
                number_of_images: output.images.len() as u64,
            });
        }

        Ok(command::InternalCommand::GetOutputSettingsResponse(command::GetOutputSettingsResponse {
            status: status::Status::Success,
            error: "".to_string(),
            outputs_settings,
        }))
    }
}
