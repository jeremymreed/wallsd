use std::collections::HashMap;
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
                let response = command::SetOutputModeResponse {
                    status: status::Status::Success,
                    error: "".to_string(),
                };

                Ok(command::InternalCommand::SetOutputModeResponse(response))
            },
            None => {
                tracing::error!("Output {} not found", name);
                let response = command::SetOutputModeResponse {
                    status: status::Status::Failure,
                    error: format!("Output {} not found", name),
                };

                Err(command::InternalCommand::SetOutputModeResponse(response))
            }
        }
    }
}
