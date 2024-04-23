use std::collections::HashMap;
use crate::systemd_analyze;
use crate::status;
use crate::command;
use crate::output::Output;
use crate::config::Config;
use crate::mode;

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
}
